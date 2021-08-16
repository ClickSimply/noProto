//! NoProto supports Rust's native [`bool`](https://doc.rust-lang.org/std/primitive.bool.html) type.
//! 
//! ```
//! use no_proto::error::NP_Error;
//! use no_proto::NP_Factory;
//! use no_proto::pointer::bytes::NP_Bytes;
//! 
//! let factory: NP_Factory = NP_Factory::new("bool()")?;
//!
//! let mut new_buffer = factory.new_buffer(None);
//! new_buffer.set(&[], true)?;
//! 
//! assert_eq!(true, new_buffer.get::<bool>(&[])?.unwrap());
//!
//! # Ok::<(), NP_Error>(()) 
//! ```

use alloc::sync::Arc;
use alloc::string::String;
use crate::{idl::{JS_AST, JS_Schema}, json_flex::JSMAP, schema::{NP_Bool_Data, NP_Parsed_Schema, NP_Value_Kind}};
use crate::error::NP_Error;
use crate::{schema::{NP_TypeKeys}, pointer::NP_Value, json_flex::NP_JSON};

use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::{borrow::ToOwned};
use crate::NP_Memory;
use alloc::string::ToString;

use super::NP_Cursor;

impl<'value> super::NP_Scalar<'value> for bool {

    fn schema_default(_schema: &NP_Parsed_Schema) -> Option<Self> where Self: Sized {
        Some(Self::default())
    }
    fn np_max_value(_cursor: &NP_Cursor, _memory: &NP_Memory) -> Option<Self> {
        Some(true)
    }

    fn np_min_value(_cursor: &NP_Cursor, _memory: &NP_Memory) -> Option<Self> {
        Some(false)
    }
}

impl<'value> NP_Value<'value> for bool {

    fn type_idx() -> (&'value str, NP_TypeKeys) { ("bool", NP_TypeKeys::Boolean) }
    fn self_type_idx(&self) -> (&'value str, NP_TypeKeys) { ("bool", NP_TypeKeys::Boolean) }

    fn schema_to_json(schema: &Vec<NP_Parsed_Schema>, address: usize)-> Result<NP_JSON, NP_Error> {
        let mut schema_json = JSMAP::new();
        schema_json.insert("type".to_owned(), NP_JSON::String(Self::type_idx().0.to_string()));

        let data = unsafe { &*(*schema[address].data as *const NP_Bool_Data) };

        if let Some(d) = data.default {
            schema_json.insert("default".to_owned(), match d {
                true => NP_JSON::True,
                false => NP_JSON::False
            });
        }
         

        Ok(NP_JSON::Dictionary(schema_json))
    }

    fn default_value(_depth: usize, address: usize, schema: &Vec<NP_Parsed_Schema>) -> Option<Self> {
        let data = unsafe { &*(*schema[address].data as *const NP_Bool_Data) };

        data.default
    }

    fn set_from_json<'set>(_depth: usize, _apply_null: bool, cursor: NP_Cursor, memory: &'set NP_Memory, value: &Box<NP_JSON>) -> Result<(), NP_Error> where Self: 'set + Sized {
        match **value {
            NP_JSON::True => {
                Self::set_value(cursor, memory, true)?;
            },
            NP_JSON::False => {
                Self::set_value(cursor, memory, false)?;
            },
            _ => {}
        }

        Ok(())
    }

    fn set_value<'set>(cursor: NP_Cursor, memory: &'set NP_Memory, value: Self) -> Result<NP_Cursor, NP_Error> where Self: 'set + Sized {

        let c_value = || { cursor.get_value(memory) };
        let mut value_address = c_value().get_addr_value();  

        if value_address != 0 { // existing value, replace

            // overwrite existing values in buffer
            memory.write_bytes()[value_address as usize] = if value == true {
                1
            } else {
                0
            };

            return Ok(cursor);

        } else { // new value

            let bytes = if value == true {
                [1] as [u8; 1]
            } else {
                [0] as [u8; 1]
            };

            value_address = memory.malloc_borrow(&bytes)? as u32;
            cursor.get_value_mut(memory).set_addr_value(value_address as u32);

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

        Ok(match memory.get_1_byte(value_addr) {
            Some(x) => {
                Some(if x == 1 { true } else { false })
            },
            None => None
        })
    }

    fn to_json(_depth:usize, cursor: &NP_Cursor, memory: &'value NP_Memory) -> NP_JSON {

        match Self::into_value(cursor, memory) {
            Ok(x) => {
                match x {
                    Some(y) => {
                        if y == true {
                            NP_JSON::True
                        } else {
                            NP_JSON::False
                        }
                    },
                    None => {
                        
                        let data = unsafe { &*(*memory.get_schema(cursor.schema_addr).data as *const NP_Bool_Data) };

                        if let Some(d) = data.default {
                            if d == true {
                                NP_JSON::True
                            } else {
                                NP_JSON::False
                            }
                        } else {
                            NP_JSON::Null
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
        if c_value().get_addr_value() == 0 {
            Ok(0) 
        } else {
            Ok(core::mem::size_of::<u8>())
        }
    }

    fn schema_to_idl(schema: &Vec<NP_Parsed_Schema>, address: usize)-> Result<String, NP_Error> {

        let data = unsafe { &*(*schema[address].data as *const NP_Bool_Data) };
        
        let mut result = String::from("bool(");
        if let Some(x) = data.default {
            result.push_str("{default: ");
            if x == true {
                result.push_str("true");
            } else {
                result.push_str("false");
            }
            result.push_str("}");
        }
        result.push_str(")");
        Ok(result)
          
    }

    fn from_idl_to_schema(mut schema: Vec<NP_Parsed_Schema>, _name: &str, idl: &JS_Schema, args: &Vec<JS_AST>) -> Result<(bool, Vec<u8>, Vec<NP_Parsed_Schema>), NP_Error> {

        let mut default: Option<bool> = None;
        if args.len() > 0 {
            match &args[0] {
                JS_AST::object { properties } => {
                    for (key, value) in properties {
                        match idl.get_str(key).trim() {
                            "default" => {
                                match value {
                                    JS_AST::bool { state } => {
                                        default = Some(*state);
                                    },
                                    _ => { }
                                }
                            },
                            _ => { }
                        }
                    }
                },
                _ => { }
            }
        }

        let mut schema_data: Vec<u8> = Vec::new();
        schema_data.push(NP_TypeKeys::Boolean as u8);

        let default = match default {
            Some(x) => {
                if x == false {
                    schema_data.push(2);
                } else {
                    schema_data.push(1);
                }
                Some(x)  
            },
            _ => {
                schema_data.push(0);
                None
            }
        };

        schema.push(NP_Parsed_Schema {
            val: NP_Value_Kind::Fixed(1),
            i: NP_TypeKeys::Boolean,
            sortable: true,
            data: Arc::new(Box::into_raw(Box::new(NP_Bool_Data { default })) as *const u8)
        });

        return Ok((true, schema_data, schema));

    }

    fn from_json_to_schema(mut schema: Vec<NP_Parsed_Schema>, json_schema: &Box<NP_JSON>) -> Result<(bool, Vec<u8>, Vec<NP_Parsed_Schema>), NP_Error> {

        let mut schema_data: Vec<u8> = Vec::new();
        schema_data.push(NP_TypeKeys::Boolean as u8);

        let default = match json_schema["default"] {
            NP_JSON::False => {
                schema_data.push(2);
                Some(false)
            },
            NP_JSON::True => {
                schema_data.push(1);
                Some(true)
            },
            _ => {
                schema_data.push(0);
                None
            }
        };

        schema.push(NP_Parsed_Schema {
            val: NP_Value_Kind::Fixed(1),
            i: NP_TypeKeys::Boolean,
            data: Arc::new(Box::into_raw(Box::new(NP_Bool_Data { default })) as *const u8),
            sortable: true
        });

        return Ok((true, schema_data, schema));
  
    }
    fn from_bytes_to_schema(mut schema: Vec<NP_Parsed_Schema>, address: usize, bytes: &[u8]) -> (bool, Vec<NP_Parsed_Schema>) {
        schema.push(NP_Parsed_Schema {
            val: NP_Value_Kind::Fixed(1),
            i: NP_TypeKeys::Boolean,
            sortable: true,
            data: Arc::new(Box::into_raw(Box::new(NP_Bool_Data { default: match bytes[address + 1] {
                0 => None,
                1 => Some(true),
                2 => Some(false),
                _ => unreachable!()
            } })) as *const u8)
        });
        (true, schema)
     }
}


#[test]
fn schema_parsing_works_idl() -> Result<(), NP_Error> {
    let schema = "bool({default: false})";
    let factory = crate::NP_Factory::new(schema)?;
    assert_eq!(schema, factory.schema.to_idl()?);
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_idl()?);

    let schema = "bool()";
    let factory = crate::NP_Factory::new(schema)?;
    assert_eq!(schema, factory.schema.to_idl()?);
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_idl()?);
    Ok(())
}

#[test]
fn schema_parsing_works() -> Result<(), NP_Error> {
    let schema = "{\"type\":\"bool\",\"default\":false}";
    let factory = crate::NP_Factory::new_json(schema)?;
    assert_eq!(schema, factory.schema.to_json()?.stringify());
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_json()?.stringify());

    let schema = "{\"type\":\"bool\"}";
    let factory = crate::NP_Factory::new_json(schema)?;
    assert_eq!(schema, factory.schema.to_json()?.stringify());
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_json()?.stringify());
    Ok(())
}

#[test]
fn default_value_works() -> Result<(), NP_Error> {
    let schema = "{\"type\":\"bool\",\"default\":false}";
    let factory = crate::NP_Factory::new_json(schema)?;
    let buffer = factory.new_buffer(None);
    assert_eq!(buffer.get::<bool>(&[])?.unwrap(), false);

    Ok(())
}


#[test]
fn set_clear_value_and_compaction_works() -> Result<(), NP_Error> {
    let schema = "{\"type\":\"bool\"}";
    let factory = crate::NP_Factory::new_json(schema)?;
    let mut buffer = factory.new_buffer(None);
    buffer.set(&[], false)?;
    assert_eq!(buffer.get::<bool>(&[])?.unwrap(), false);
    buffer.del(&[])?;
    assert_eq!(buffer.get::<bool>(&[])?, None);

    buffer.compact(None)?;
    assert_eq!(buffer.calc_bytes()?.current_buffer, 6usize);

    Ok(())
}