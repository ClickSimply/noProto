//! Represents arbitrary bytes type
//! 
//! ```
//! use no_proto::error::NP_Error;
//! use no_proto::NP_Factory;
//! use no_proto::pointer::bytes::NP_Bytes;
//! 
//! let factory: NP_Factory = NP_Factory::new(r#"{
//!    "type": "bytes"
//! }"#)?;
//!
//! let mut new_buffer = factory.empty_buffer(None);
//! new_buffer.set(&[], &[0u8, 1, 2, 3, 4] as NP_Bytes)?;
//! 
//! assert_eq!(&[0u8, 1, 2, 3, 4] as NP_Bytes, new_buffer.get::<NP_Bytes>(&[])?.unwrap());
//!
//! # Ok::<(), NP_Error>(()) 
//! ```
//! 

use crate::{json_flex::JSMAP, schema::{NP_Parsed_Schema}};
use crate::error::NP_Error;
use crate::{schema::{NP_TypeKeys}, pointer::NP_Value, json_flex::NP_JSON};

use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::{borrow::ToOwned};
use super::{NP_Cursor};
use crate::NP_Memory;
use alloc::string::ToString;

/// Arbitrary bytes
pub type NP_Bytes<'bytes> = &'bytes [u8];

impl<'value> super::NP_Scalar<'value> for &[u8] {}

impl<'value> NP_Value<'value> for &'value [u8] {


    fn type_idx() -> (&'value str, NP_TypeKeys) { ("bytes", NP_TypeKeys::Bytes) }
    fn self_type_idx(&self) -> (&'value str, NP_TypeKeys) { ("bytes", NP_TypeKeys::Bytes) }

    fn schema_to_json(schema: &Vec<NP_Parsed_Schema>, address: usize)-> Result<NP_JSON, NP_Error> {
        let mut schema_json = JSMAP::new();
        schema_json.insert("type".to_owned(), NP_JSON::String(Self::type_idx().0.to_string()));

        match &schema[address] {
            NP_Parsed_Schema::Bytes { i: _, sortable: _, default, size} => {
                if *size > 0 {
                    schema_json.insert("size".to_owned(), NP_JSON::Integer(*size as i64));
                }
              
                // no default right now
                if let Some(d) = default {
                    let default_bytes: Vec<NP_JSON> = d.iter().map(|value| {
                        NP_JSON::Integer(i64::from(*value))
                    }).collect();
                    schema_json.insert("default".to_owned(), NP_JSON::Array(default_bytes));
                }
            },
            _ => { }
        }


        Ok(NP_JSON::Dictionary(schema_json))
    }

    fn default_value(schema: &'value NP_Parsed_Schema) -> Option<Self> {

        match schema {
            NP_Parsed_Schema::Bytes { default, .. } => {
                if let Some(d) = default {
                    Some(&d[..])
                } else {
                    None
                }
            },
            _ => None
        }
    }

 
    fn set_value<'set, M: NP_Memory>(cursor: NP_Cursor, memory: &'set M, value: Self) -> Result<NP_Cursor, NP_Error> where Self: 'set + Sized {

        let c_value = cursor.get_value(memory);
    
        let bytes = value;
    
        let str_size = bytes.len() as usize;
    
        let mut write_bytes = memory.write_bytes();
    
        let size = match memory.get_schema(cursor.schema_addr) {
            NP_Parsed_Schema::Bytes { size, .. } => *size,
            _ => 0
        };
    
        if size > 0 {
            // fixed size bytes
    
            if c_value.get_addr_value() == 0 {
                // malloc new bytes
    
                let mut empty_bytes: Vec<u8> = Vec::with_capacity(size as usize);
                for _x in 0..size {
                    empty_bytes.push(0);
                }
    
                let new_addr = memory.malloc(empty_bytes)? as usize;
                c_value.set_addr_value(new_addr as u16);
            }

            let addr = c_value.get_addr_value() as usize;

            write_bytes = memory.write_bytes();
    
            for x in 0..(size as usize) {
                if x < bytes.len() {
                    // assign values of bytes
                    write_bytes[(addr + x)] = bytes[x];
                } else {
                    // rest is zeros
                    write_bytes[(addr + x)] = 0;
                }
            }
    
            return Ok(cursor);
        }
    
        // flexible size
        let addr_value = c_value.get_addr_value() as usize;
    
        let prev_size: usize = if addr_value != 0 {
            let size_bytes: &[u8; 2] = memory.get_2_bytes(addr_value).unwrap_or(&[0; 2]);
            u16::from_be_bytes(*size_bytes) as usize
        } else {
            0 as usize
        };
    
        if prev_size >= str_size as usize {
            // previous string is larger than this one, use existing memory
    
            // update string length in buffer
            if str_size > core::u16::MAX as usize {
                return Err(NP_Error::new("String too large!"));
            }
            let size_bytes = (str_size as u16).to_be_bytes();
            // set string size
            for x in 0..size_bytes.len() {
                write_bytes[(addr_value + x)] = size_bytes[x];
            }
    
            let offset = 2;
    
            // set bytes
            for x in 0..bytes.len() {
                write_bytes[(addr_value + x + offset) as usize] = bytes[x];
            }
    
            return Ok(cursor);
        } else {
            // not enough space or space has not been allocted yet
    
            // first bytes are string length
            let new_addr = {
                if str_size > core::u16::MAX as usize {
                    return Err(NP_Error::new("String too large!"));
                }
                let size_bytes = (str_size as u16).to_be_bytes();
                memory.malloc_borrow(&size_bytes)?
            };
    
            c_value.set_addr_value(new_addr as u16);
    
            memory.malloc_borrow(bytes)?;
    
            return Ok(cursor);
        }
    }
    

    fn into_value<M: NP_Memory>(cursor: &NP_Cursor, memory: &'value M) -> Result<Option<Self>, NP_Error> where Self: Sized {

        let c_value = cursor.get_value(memory);

        let value_addr = c_value.get_addr_value() as usize;
        // empty value
        if value_addr == 0 {
            return Ok(None);
        }

        match memory.get_schema(cursor.schema_addr) {
            NP_Parsed_Schema::Bytes {
                i: _,
                sortable: _,
                default: _,
                size,
            } => {
                if *size > 0 {
                    // fixed size

                    // get bytes
                    let bytes = &memory.read_bytes()[(value_addr)..(value_addr + (*size as usize))];

                    return Ok(Some(bytes));
                } else {
                    // dynamic size
                    // get size of bytes

                    let bytes_size: usize = u16::from_be_bytes(*memory.get_2_bytes(value_addr).unwrap_or(&[0; 2])) as usize;

                    // get bytes
                    let bytes = &memory.read_bytes()[(value_addr + 2)..(value_addr + 2 + bytes_size)];

                    return Ok(Some(bytes));
                }
            }
            _ => Err(NP_Error::new("unreachable")),
        }
    }

    fn to_json<M: NP_Memory>(cursor: &NP_Cursor, memory: &'value M) -> NP_JSON {


        match Self::into_value(cursor, memory) {
            Ok(x) => {
                match x {
                    Some(y) => {

                        let bytes = y.iter().map(|x| NP_JSON::Integer(*x as i64)).collect();

                        NP_JSON::Array(bytes)
                    },
                    None => {

                        match &memory.get_schema(cursor.schema_addr) {
                            NP_Parsed_Schema::Bytes { default, .. } => {
                                match default {
                                    Some(x) => {
                                        let bytes = x.iter().map(|v| {
                                            NP_JSON::Integer(*v as i64)
                                        }).collect::<Vec<NP_JSON>>();

                                        NP_JSON::Array(bytes)
                                    },
                                    None => NP_JSON::Null
                                }
                            },
                            _ => NP_JSON::Null
                        }
                    }
                }
            },
            Err(_e) => {
                NP_JSON::Null
            }
        }
    }
    fn get_size<M: NP_Memory>(cursor: &NP_Cursor, memory: &M) -> Result<usize, NP_Error> {

        let c_value = cursor.get_value(memory);
        let value_addr = c_value.get_addr_value() as usize;
        
        // empty value
        if value_addr == 0 {
            return Ok(0);
        }

        match memory.get_schema(cursor.schema_addr) {
            NP_Parsed_Schema::Bytes { size, .. } => {
                // fixed size
                if *size > 0 {
                    return Ok(*size as usize);
                }

                // dynamic size
                let bytes_size: usize = u16::from_be_bytes(*memory.get_2_bytes(value_addr).unwrap_or(&[0; 2])) as usize;

                // return total size of this string plus length
                return Ok(bytes_size + 2);
            }
            _ => Err(NP_Error::new("unreachable")),
        }
    }

    fn from_json_to_schema(mut schema: Vec<NP_Parsed_Schema>, json_schema: &Box<NP_JSON>) -> Result<(bool, Vec<u8>, Vec<NP_Parsed_Schema>), NP_Error> {


        let mut has_fixed_size = false;
        let mut schema_data: Vec<u8> = Vec::new();
        schema_data.push(NP_TypeKeys::Bytes as u8);

        let size = match json_schema["size"] {
            NP_JSON::Integer(x) => {
                has_fixed_size = true;
                if x < 1 {
                    return Err(NP_Error::new("Fixed size for bytes must be larger than 1!"));
                }
                if x > u16::MAX.into() {
                    return Err(NP_Error::new("Fixed size for bytes cannot be larger than 2^16!"));
                }
                schema_data.extend((x as u16).to_be_bytes().to_vec());
                x as u16
            },
            NP_JSON::Float(x) => {
                has_fixed_size = true;
                if x < 1.0 {
                    return Err(NP_Error::new("Fixed size for bytes must be larger than 1!"));
                }
                if x > u16::MAX.into() {
                    return Err(NP_Error::new("Fixed size for bytes cannot be larger than 2^16!"));
                }

                schema_data.extend((x as u16).to_be_bytes().to_vec());
                x as u16
            },
            _ => {
                schema_data.extend(0u16.to_be_bytes().to_vec());
                0
            }
        };

        let default = match &json_schema["default"] {
            NP_JSON::Array(bytes) => {

                let default_bytes: Vec<u8> = bytes.iter().map(|v| {
                    match v {
                        NP_JSON::Integer(x) => { *x as u8},
                        _ => { 0u8 }
                    }
                }).collect();
                let length = default_bytes.len() as u16 + 1;
                schema_data.extend(length.to_be_bytes().to_vec());
                schema_data.extend(default_bytes.clone());
                Some(default_bytes)
            },
            _ => {
                schema_data.extend(0u16.to_be_bytes().to_vec());
                None
            }
        };
        

        schema.push(NP_Parsed_Schema::Bytes {
            i: NP_TypeKeys::Bytes,
            size: size,
            default: default,
            sortable: has_fixed_size
        });

        return Ok((has_fixed_size, schema_data, schema));
    }

    fn from_bytes_to_schema(mut schema: Vec<NP_Parsed_Schema>, address: usize, bytes: &[u8]) -> (bool, Vec<NP_Parsed_Schema>) {
        // fixed size
        let fixed_size = u16::from_be_bytes([
            bytes[address + 1],
            bytes[address + 2]
        ]);

        // default value size
        let default_size = u16::from_be_bytes([
            bytes[address + 3],
            bytes[address + 4]
        ]) as usize;

        if default_size == 0 {
            schema.push(NP_Parsed_Schema::Bytes {
                i: NP_TypeKeys::Bytes,
                default: None,
                sortable: fixed_size > 0,
                size: fixed_size
            });
        } else {
            let default_bytes = &bytes[(address + 5)..(address + 5 + (default_size - 1))];

            schema.push(NP_Parsed_Schema::Bytes {
                i: NP_TypeKeys::Bytes,
                default: Some(default_bytes.to_vec()),
                size: fixed_size,
                sortable: fixed_size > 0
            });    
        }

        (fixed_size > 0, schema)

    }
}
