//! Represents arbitrary bytes type
//! 
//! ```
//! use no_proto::error::NP_Error;
//! use no_proto::NP_Factory;
//! use no_proto::pointer::bytes::NP_Bytes;
//! 
//! let factory: NP_Factory = NP_Factory::new("bytes()")?;
//!
//! let mut new_buffer = factory.new_buffer(None);
//! new_buffer.set(&[], &[0u8, 1, 2, 3, 4] as &[u8])?;
//! 
//! assert_eq!(&[0u8, 1, 2, 3, 4] as &[u8], new_buffer.get::<&[u8]>(&[])?.unwrap());
//!
//! # Ok::<(), NP_Error>(()) 
//! ```
//! 

use alloc::{string::String, sync::Arc};
use crate::{idl::{JS_AST, JS_Schema}, json_flex::JSMAP, schema::{NP_Bytes_Data, NP_Parsed_Schema, NP_Value_Kind}};
use crate::error::NP_Error;
use crate::{schema::{NP_TypeKeys}, pointer::NP_Value, json_flex::NP_JSON};

use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::{borrow::ToOwned};
use super::{NP_Cursor};
use crate::NP_Memory;
use alloc::string::ToString;

/// Arbitrary bytes
/// Alias for Vec<u8>
pub type NP_Bytes = Vec<u8>;

/// Arbitrary bytes, borrowed
/// Alias for &[u8]
pub type NP_Borrow_Bytes<'bytes> = &'bytes [u8];


impl<'value> super::NP_Scalar<'value> for NP_Bytes {
    fn schema_default(schema: &NP_Parsed_Schema) -> Option<Self> where Self: Sized {
        let data = unsafe { &*(*schema.data as *const NP_Bytes_Data) };

        Some(if data.size > 0 {
            let mut v: Vec<u8> = Vec::with_capacity(data.size as usize);
            for _x in 0..data.size {
                v.push(0u8);
            }
            v
        } else {
            Vec::new()
        })
         
    }

    fn np_max_value(cursor: &NP_Cursor, memory: &NP_Memory) -> Option<Self> {
        let data = unsafe { &*(*memory.get_schema(cursor.schema_addr).data as *const NP_Bytes_Data) };

        let size = data.size;

        if size == 0 {
            None
        } else {
            let mut value: Vec<u8> = Vec::with_capacity(size as usize);

            for _x in 0..size {
                value.push(255);
            }

            Some(value)
        }
    }

    fn np_min_value(cursor: &NP_Cursor, memory: &NP_Memory) -> Option<Self> {
        let data = unsafe { &*(*memory.get_schema(cursor.schema_addr).data as *const NP_Bytes_Data) };

        let size = data.size;

        if size == 0 {
            None
        } else {
            let mut value: Vec<u8> = Vec::with_capacity(size as usize);

            for _x in 0..size {
                value.push(0);
            }

            Some(value)
        }
    }

}

impl<'value> NP_Value<'value> for NP_Bytes {



    fn type_idx() -> (&'value str, NP_TypeKeys) { ("bytes", NP_TypeKeys::Bytes) }
    fn self_type_idx(&self) -> (&'value str, NP_TypeKeys) { ("bytes", NP_TypeKeys::Bytes) }

    fn schema_to_json(schema: &Vec<NP_Parsed_Schema>, address: usize)-> Result<NP_JSON, NP_Error> {
        let mut schema_json = JSMAP::new();
        schema_json.insert("type".to_owned(), NP_JSON::String(Self::type_idx().0.to_string()));

        let data = unsafe { &*(*schema[address].data as *const NP_Bytes_Data) };

        if data.size > 0 {
            schema_json.insert("size".to_owned(), NP_JSON::Integer(data.size as i64));
        }
        
        // no default right now
        if let Some(d) = &data.default {
            let default_bytes: Vec<NP_JSON> = d.iter().map(|value| {
                NP_JSON::Integer(i64::from(*value))
            }).collect();
            schema_json.insert("default".to_owned(), NP_JSON::Array(default_bytes));
        }
      


        Ok(NP_JSON::Dictionary(schema_json))
    }

    fn default_value(_depth: usize, address: usize, schema: &Vec<NP_Parsed_Schema>) -> Option<Self> {
        let data = unsafe { &*(*schema[address].data as *const NP_Bytes_Data) };

        if let Some(d) = &data.default {
            Some(d.clone())
        } else {
            None
        }
       
    }

 
    fn set_value<'set>(cursor: NP_Cursor, memory: &'set NP_Memory, value: Self) -> Result<NP_Cursor, NP_Error> where Self: 'set + Sized {
        NP_Borrow_Bytes::set_value(cursor, memory, &value)
    }


    fn schema_to_idl(schema: &Vec<NP_Parsed_Schema>, address: usize)-> Result<String, NP_Error> {
        let data = unsafe { &*(*schema[address].data as *const NP_Bytes_Data) };

        let mut properties: Vec<String> = Vec::new();

        if let Some(x) = &data.default {
            let mut def = String::from("default: ");
            def.push_str("[");
            def.push_str(x.iter().map(|b| b.to_string()).collect::<Vec<String>>().join(",").as_str());
            def.push_str("]");
            properties.push(def);
        }

        if data.size > 0 {
            let mut def = String::from("size: ");
            def.push_str(data.size.to_string().as_str());
            properties.push(def);
        }

        Ok(if properties.len() == 0 {
            String::from("bytes()")
        } else {
            let mut final_str = String::from("bytes({");
            final_str.push_str(properties.join(", ").as_str());
            final_str.push_str("})");
            final_str
        })

    }

    fn from_idl_to_schema(mut schema: Vec<NP_Parsed_Schema>, _name: &str, idl: &JS_Schema, args: &Vec<JS_AST>) -> Result<(bool, Vec<u8>, Vec<NP_Parsed_Schema>), NP_Error> {
        let mut schema_data: Vec<u8> = Vec::new();
        schema_data.push(NP_TypeKeys::Bytes as u8);

        let mut has_fixed_size = false;
        let mut size = 0u32;

        let mut default: Option<Vec<u8>> = Option::None;

        if args.len() > 0 {
            match &args[0] {
                JS_AST::object { properties } => {
                    for (key, value) in properties.iter() {
                        match idl.get_str(key).trim() {
                            "size" => {
                                match value {
                                    JS_AST::number { addr } => {
                                        match idl.get_str(addr).trim().parse::<u32>() {
                                            Ok(x) => {
                                                size = x;
                                                has_fixed_size = true;
                                            },
                                            Err(_e) => { return Err(NP_Error::new("size property must be an integer!")) }
                                        }
                                    },
                                    _ => { }
                                }
                            },
                            "default" => {
                                match value {
                                    JS_AST::array { values } => {
                                        let mut default_vals: Vec<u8> = Vec::new();

                                        for val in values {
                                            match val {
                                                JS_AST::number { addr } => {
                                                    match idl.get_str(addr).parse::<u8>() {
                                                        Ok(x) => {
                                                            default_vals.push(x);
                                                        },
                                                        _ => {}
                                                    }
                                                },
                                                _ => { }
                                            }
                                        }

                                        default = Some(default_vals);
                                    },
                                    _ => { }
                                }
                            }
                            _ => { }
                        }
                    }
                }
                _ => { }
            }
        };

        if has_fixed_size {
            schema_data.extend_from_slice(&size.to_be_bytes());
        } else {
            schema_data.extend_from_slice(&0u32.to_be_bytes());
        }

        if let Some(x) = &default {
            schema_data.extend_from_slice(&((x.len() + 1) as u16).to_be_bytes());
            schema_data.extend_from_slice(&x[..]);
        } else {
            schema_data.extend(0u16.to_be_bytes().to_vec());
        }

        schema.push(NP_Parsed_Schema {
            val: if size > 0 {
                NP_Value_Kind::Fixed(size as u32)
            } else {
                NP_Value_Kind::Pointer
            },
            i: NP_TypeKeys::Bytes,
            sortable: has_fixed_size,
            data: Arc::new(Box::into_raw(Box::new(NP_Bytes_Data { size, default })) as *const u8)
        });

        return Ok((has_fixed_size, schema_data, schema));
    }
    
    fn set_from_json<'set>(_depth: usize, _apply_null: bool, cursor: NP_Cursor, memory: &'set NP_Memory, value: &Box<NP_JSON>) -> Result<(), NP_Error> where Self: 'set + Sized {
        match &**value {
            NP_JSON::Array(bytes) => {
                let mut target: Vec<u8> = Vec::new();

                bytes.iter().for_each(|json| {
                    match json {
                        NP_JSON::Integer(x) => {
                            target.push(*x as u8);
                        },
                        NP_JSON::Float(x) => {
                            target.push(*x as u8);
                        },
                        _ => {
                            target.push(0);
                        }
                    }
                });

                Self::set_value(cursor, memory, target)?;
            },
            _ => { }
        }

        Ok(())
    }

    fn into_value(cursor: &NP_Cursor, memory: &'value NP_Memory) -> Result<Option<Self>, NP_Error> where Self: Sized {
        match NP_Borrow_Bytes::into_value(cursor, memory)? {
            Some(bytes) => Ok(Some(bytes.to_vec())),
            None => Ok(None)
        }
    }

    fn to_json(_depth:usize, cursor: &NP_Cursor, memory: &'value NP_Memory) -> NP_JSON {


        match Self::into_value(cursor, memory) {
            Ok(x) => {
                match x {
                    Some(y) => {

                        let bytes = y.iter().map(|x| NP_JSON::Integer(*x as i64)).collect();

                        NP_JSON::Array(bytes)
                    },
                    None => {

                        let data = unsafe { &*(*memory.get_schema(cursor.schema_addr).data as *const NP_Bytes_Data) };

                        match &data.default {
                            Some(x) => {
                                let bytes = x.iter().map(|v| {
                                    NP_JSON::Integer(*v as i64)
                                }).collect::<Vec<NP_JSON>>();

                                NP_JSON::Array(bytes)
                            },
                            None => NP_JSON::Null
                        }
                       
                    }
                }
            },
            Err(_e) => {
                NP_JSON::Null
            }
        }
    }
    fn get_size(_depth:usize, cursor: &NP_Cursor, memory: &NP_Memory) -> Result<usize, NP_Error> {

        let c_value = || { cursor.get_value(memory) };
        let value_addr = c_value().get_addr_value() as usize;
        
        // empty value
        if value_addr == 0 {
            return Ok(0);
        }

        let data = unsafe { &*(*memory.get_schema(cursor.schema_addr).data as *const NP_Bytes_Data) };

        // fixed size
        if data.size > 0 {
            return Ok(data.size as usize);
        }

        // dynamic size
        let bytes_size: usize = u32::from_be_bytes(*memory.get_4_bytes(value_addr).unwrap_or(&[0; 4])) as usize;

        // return total size of this string plus length
        return Ok(bytes_size + 4);
        
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
                if x > u32::MAX.into() {
                    return Err(NP_Error::new("Fixed size for bytes cannot be larger than 2^32!"));
                }
                schema_data.extend((x as u32).to_be_bytes().to_vec());
                x as u32
            },
            NP_JSON::Float(x) => {
                has_fixed_size = true;
                if x < 1.0 {
                    return Err(NP_Error::new("Fixed size for bytes must be larger than 1!"));
                }
                if x > u32::MAX.into() {
                    return Err(NP_Error::new("Fixed size for bytes cannot be larger than 2^32!"));
                }

                schema_data.extend((x as u32).to_be_bytes().to_vec());
                x as u32
            },
            _ => {
                schema_data.extend(0u32.to_be_bytes().to_vec());
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
        

        schema.push(NP_Parsed_Schema {
            val: if size > 0 {
                NP_Value_Kind::Fixed(size as u32)
            } else {
                NP_Value_Kind::Pointer
            },
            i: NP_TypeKeys::Bytes,
            data: Arc::new(Box::into_raw(Box::new(NP_Bytes_Data { size, default })) as *const u8),
            sortable: has_fixed_size
        });

        return Ok((has_fixed_size, schema_data, schema));
    }

    fn from_bytes_to_schema(mut schema: Vec<NP_Parsed_Schema>, address: usize, bytes: &[u8]) -> (bool, Vec<NP_Parsed_Schema>) {
        // fixed size
        let fixed_size = u32::from_be_bytes([
            bytes[address + 1],
            bytes[address + 2],
            bytes[address + 3],
            bytes[address + 4]
        ]);

        // default value size
        let default_size = u16::from_be_bytes([
            bytes[address + 5],
            bytes[address + 6]
        ]) as usize;

        if default_size == 0 {
            schema.push(NP_Parsed_Schema {
                val: if fixed_size > 0 {
                    NP_Value_Kind::Fixed(fixed_size as u32)
                } else {
                    NP_Value_Kind::Pointer
                },
                i: NP_TypeKeys::Bytes,
                sortable: fixed_size > 0,
                data: Arc::new(Box::into_raw(Box::new(NP_Bytes_Data { size: fixed_size, default: None })) as *const u8)
            });
        } else {
            let default_bytes = &bytes[(address + 7)..(address + 7 + (default_size - 1))];

            schema.push(NP_Parsed_Schema {
                val: if fixed_size > 0 {
                    NP_Value_Kind::Fixed(fixed_size as u32)
                } else {
                    NP_Value_Kind::Pointer
                },
                i: NP_TypeKeys::Bytes,
                data: Arc::new(Box::into_raw(Box::new(NP_Bytes_Data { size: fixed_size, default: Some(default_bytes.to_vec()) })) as *const u8),
                sortable: fixed_size > 0
            });    
        }

        (fixed_size > 0, schema)

    }
}

impl<'value> super::NP_Scalar<'value> for &[u8] {
    fn schema_default(_schema: &NP_Parsed_Schema) -> Option<Self> where Self: Sized {
        None
    }

    fn np_max_value(_cursor: &NP_Cursor, _memory: &NP_Memory) -> Option<Self> {
        None
    }

    fn np_min_value(_cursor: &NP_Cursor, _memory: &NP_Memory) -> Option<Self> {
        None
    }
}

impl<'value> NP_Value<'value> for NP_Borrow_Bytes<'value> {



    fn type_idx() -> (&'value str, NP_TypeKeys) { NP_Bytes::type_idx() }
    fn self_type_idx(&self) -> (&'value str, NP_TypeKeys) { NP_Bytes::type_idx() }

    fn schema_to_json(schema: &Vec<NP_Parsed_Schema>, address: usize)-> Result<NP_JSON, NP_Error> {
        NP_Bytes::schema_to_json(schema, address)
    }

    fn set_from_json<'set>(_depth: usize, _apply_null: bool, _cursor: NP_Cursor, _memory: &'set NP_Memory, _value: &Box<NP_JSON>) -> Result<(), NP_Error> where Self: 'set + Sized {
        Ok(())
    }

    fn default_value(_depth: usize, addr: usize, schema: &'value Vec<NP_Parsed_Schema>) -> Option<Self> {
        let data = unsafe { &*(*schema[addr].data as *const NP_Bytes_Data) };

        if let Some(d) = &data.default {
            Some(&d[..])
        } else {
            None
        }
          
    }

    // This is never called
    fn schema_to_idl(_schema: &Vec<NP_Parsed_Schema>, _address: usize)-> Result<String, NP_Error> {
        Ok(String::from("bytes()"))
    }

    // This is never called
    fn from_idl_to_schema(schema: Vec<NP_Parsed_Schema>, _name: &str, _idl: &JS_Schema, _args: &Vec<JS_AST>) -> Result<(bool, Vec<u8>, Vec<NP_Parsed_Schema>), NP_Error> {
        Self::from_json_to_schema(schema, &Box::new(NP_JSON::Null))
    }

 
    fn set_value<'set>(cursor: NP_Cursor, memory: &'set NP_Memory, value: Self) -> Result<NP_Cursor, NP_Error> where Self: 'set + Sized {

        let c_value = || { cursor.get_value(memory) };
    
        let bytes = value;
    
        let str_size = bytes.len() as usize;
    
        let mut write_bytes = memory.write_bytes();

        let data = unsafe { &*(*memory.get_schema(cursor.schema_addr).data as *const NP_Bytes_Data) };
    
        let size = data.size;
    
        if size > 0 {
            // fixed size bytes
    
            if c_value().get_addr_value() == 0 {
                // malloc new bytes
    
                let mut empty_bytes: Vec<u8> = Vec::with_capacity(size as usize);
                for _x in 0..size {
                    empty_bytes.push(0);
                }
    
                let new_addr = memory.malloc(empty_bytes)? as usize;
                cursor.get_value_mut(memory).set_addr_value(new_addr as u32);
            }

            let addr = c_value().get_addr_value() as usize;

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
        let addr_value = c_value().get_addr_value() as usize;
    
        let prev_size: usize = if addr_value != 0 {
            let size_bytes: &[u8; 4] = memory.get_4_bytes(addr_value).unwrap_or(&[0; 4]);
            u32::from_be_bytes(*size_bytes) as usize
        } else {
            0 as usize
        };
    
        if prev_size >= str_size as usize {
            // previous string is larger than this one, use existing memory
    
            // update string length in buffer
            if str_size > core::u32::MAX as usize {
                return Err(NP_Error::new("String too large!"));
            }
            let size_bytes = (str_size as u16).to_be_bytes();
            // set string size
            for x in 0..size_bytes.len() {
                write_bytes[(addr_value + x)] = size_bytes[x];
            }
    
            let offset = 4;
    
            // set bytes
            for x in 0..bytes.len() {
                write_bytes[(addr_value + x + offset) as usize] = bytes[x];
            }
    
            return Ok(cursor);
        } else {
            // not enough space or space has not been allocted yet
    
            // first bytes are string length
            let new_addr = {
                if str_size > core::u32::MAX as usize {
                    return Err(NP_Error::new("Bytes too large!"));
                }
                let size_bytes = (str_size as u32).to_be_bytes();
                memory.malloc_borrow(&size_bytes)?
            };
    
            cursor.get_value_mut(memory).set_addr_value(new_addr as u32);
    
            memory.malloc_borrow(bytes)?;
    
            return Ok(cursor);
        }
    }
    

    fn into_value(cursor: &NP_Cursor, memory: &'value NP_Memory) -> Result<Option<Self>, NP_Error> where Self: Sized {

        let c_value = || { cursor.get_value(memory) };

        let value_addr = c_value().get_addr_value() as usize;
        // empty value
        if value_addr == 0 {
            return Ok(None);
        }

        let data = unsafe { &*(*memory.get_schema(cursor.schema_addr).data as *const NP_Bytes_Data) };

        if data.size > 0 {
            // fixed size

            // get bytes
            let bytes = &memory.read_bytes()[(value_addr)..(value_addr + (data.size as usize))];

            return Ok(Some(bytes));
        } else {
            // dynamic size
            // get size of bytes

            let bytes_size: usize = u32::from_be_bytes(*memory.get_4_bytes(value_addr).unwrap_or(&[0; 4])) as usize;

            // get bytes
            let bytes = &memory.read_bytes()[(value_addr + 4)..(value_addr + 4 + bytes_size)];

            return Ok(Some(bytes));
        }
         
    }

    fn to_json(depth:usize, cursor: &NP_Cursor, memory: &'value NP_Memory) -> NP_JSON {
        NP_Bytes::to_json(depth, cursor, memory)
    }

    fn get_size(depth:usize, cursor: &NP_Cursor, memory: &NP_Memory) -> Result<usize, NP_Error> {
        NP_Bytes::get_size(depth, cursor, memory)
    }

    fn from_json_to_schema(schema: Vec<NP_Parsed_Schema>, json_schema: &Box<NP_JSON>) -> Result<(bool, Vec<u8>, Vec<NP_Parsed_Schema>), NP_Error> {
        NP_Bytes::from_json_to_schema(schema, json_schema)
    }

    fn from_bytes_to_schema(schema: Vec<NP_Parsed_Schema>, address: usize, bytes: &[u8]) -> (bool, Vec<NP_Parsed_Schema>) {
        NP_Bytes::from_bytes_to_schema(schema, address, bytes)
    }
}

#[test]
fn schema_parsing_works_idl() -> Result<(), NP_Error> {
    let schema = "bytes({default: [22,208,10,78,1,19,85], size: 10})";
    let factory = crate::NP_Factory::new(schema)?;
    assert_eq!(schema, factory.schema.to_idl()?);
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_idl()?);

    let schema = "bytes({size: 10})";
    let factory = crate::NP_Factory::new(schema)?;
    assert_eq!(schema, factory.schema.to_idl()?);
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_idl()?);

    let schema = "bytes()";
    let factory = crate::NP_Factory::new(schema)?;
    assert_eq!(schema, factory.schema.to_idl()?);
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_idl()?);
    
    Ok(())
}

#[test]
fn schema_parsing_works() -> Result<(), NP_Error> {
    let schema = "{\"type\":\"bytes\",\"default\":[22,208,10,78,1,19,85]}";
    let factory = crate::NP_Factory::new_json(schema)?;
    assert_eq!(schema, factory.schema.to_json()?.stringify());
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_json()?.stringify());

    let schema = "{\"type\":\"bytes\",\"size\":10}";
    let factory = crate::NP_Factory::new_json(schema)?;
    assert_eq!(schema, factory.schema.to_json()?.stringify());
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_json()?.stringify());

    let schema = "{\"type\":\"bytes\"}";
    let factory = crate::NP_Factory::new_json(schema)?;
    assert_eq!(schema, factory.schema.to_json()?.stringify());
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_json()?.stringify());
    
    Ok(())
}


#[test]
fn default_value_works() -> Result<(), NP_Error> {
    let schema = "{\"type\":\"bytes\",\"default\":[1,2,3,4]}";
    let factory = crate::NP_Factory::new_json(schema)?;
    let buffer = factory.new_buffer(None);
    assert_eq!(buffer.get::<&[u8]>(&[])?.unwrap(), &[1,2,3,4]);

    Ok(())
}

#[test]
fn fixed_size_works() -> Result<(), NP_Error> {
    let schema = "{\"type\":\"bytes\",\"size\": 20}";
    let factory = crate::NP_Factory::new_json(schema)?;
    let mut buffer = factory.new_buffer(None);
    buffer.set(&[], &[1u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22] as &[u8])?;
    assert_eq!(buffer.get::<&[u8]>(&[])?.unwrap(), &[1u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20] as &[u8]);

    Ok(())
}

#[test]
fn set_clear_value_and_compaction_works() -> Result<(), NP_Error> {
    let schema = "{\"type\":\"bytes\"}";
    let factory = crate::NP_Factory::new_json(schema)?;
    let mut buffer = factory.new_buffer(None);
    buffer.set(&[], &[1u8,2,3,4,5,6,7,8,9,10,11,12,13] as &[u8])?;
    assert_eq!(buffer.get::<&[u8]>(&[])?.unwrap(), &[1u8,2,3,4,5,6,7,8,9,10,11,12,13] as &[u8]);
    buffer.del(&[])?;
    assert_eq!(buffer.get::<&[u8]>(&[])?, None);

    buffer.compact(None)?;
    assert_eq!(buffer.calc_bytes()?.current_buffer, 6usize);

    Ok(())
}