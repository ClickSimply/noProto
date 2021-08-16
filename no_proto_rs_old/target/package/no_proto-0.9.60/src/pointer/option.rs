//! Represents the string value of a choice in a schema
//! 
//! ```
//! use no_proto::error::NP_Error;
//! use no_proto::NP_Factory;
//! use no_proto::pointer::option::NP_Enum;
//! 
//! let factory: NP_Factory = NP_Factory::new(r#"enum({choices: ["red", "green", "blue"] })"#)?;
//!
//! let mut new_buffer = factory.new_buffer(None);
//! new_buffer.set(&[], NP_Enum::new("green"))?;
//! 
//! assert_eq!(NP_Enum::new("green"), new_buffer.get::<NP_Enum>(&[])?.unwrap());
//!
//! # Ok::<(), NP_Error>(()) 
//! ```
//! 

use crate::{JS_Schema, idl::JS_AST, schema::{NP_Enum_Data, NP_Value_Kind}};
use crate::{memory::NP_Memory, schema::{NP_Parsed_Schema}};
use alloc::{sync::Arc, vec::Vec};
use crate::json_flex::{JSMAP, NP_JSON};
use crate::schema::{NP_TypeKeys};
use crate::{pointer::NP_Value, error::NP_Error};
use core::{fmt::{Debug}};

use alloc::string::String;
use alloc::boxed::Box;
use alloc::borrow::ToOwned;
use alloc::{string::ToString};
use super::{NP_Cursor};

/// Holds Enum / Option type data.
/// 
/// Check out documentation [here](../option/index.html).
/// 
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NP_Enum {
    /// No value
    None,
    /// Value
    Some(String)
}

impl<'value> super::NP_Scalar<'value> for NP_Enum {
    fn schema_default(_schema: &NP_Parsed_Schema) -> Option<Self> where Self: Sized {
        Some(Self::default())
    }

    fn np_max_value(cursor: &NP_Cursor, memory: &NP_Memory) -> Option<Self> {
        let data = unsafe { &*(*memory.get_schema(cursor.schema_addr).data as *const NP_Enum_Data) };
        Some(data.choices[data.choices.len() - 1].clone())
    }

    fn np_min_value(cursor: &NP_Cursor, memory: &NP_Memory) -> Option<Self> {
        let data = unsafe { &*(*memory.get_schema(cursor.schema_addr).data as *const NP_Enum_Data) };
        Some(data.choices[0].clone())
    }

}

impl NP_Enum {
    /// Create a new option type with the given string
    pub fn new<S: Into<String>>(value: S) -> Self {
        NP_Enum::Some(value.into())
    }



    /// get length of value
    pub fn len(&self) -> usize {
        match self {
            NP_Enum::None => 0,
            NP_Enum::Some(x) => x.len()
        }
    }

    /// get value as bytes
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            NP_Enum::None => &[],
            NP_Enum::Some(x) => x.as_bytes()
        }
    }

    /// get string of value
    pub fn to_str(&self) -> &str {
        match self {
            NP_Enum::None => "",
            NP_Enum::Some(x) => x
        }
    }

    /// get string of value
    pub fn to_string(&self) -> String {
        match self {
            NP_Enum::None => String::from(""),
            NP_Enum::Some(x) => x.clone()
        }
    }
}

impl Default for NP_Enum {
    fn default() -> Self { 
        NP_Enum::None
     }
}

impl<'value> NP_Value<'value> for NP_Enum {

    fn type_idx() -> (&'value str, NP_TypeKeys) { ("option", NP_TypeKeys::Enum) }
    fn self_type_idx(&self) -> (&'value str, NP_TypeKeys) { ("option", NP_TypeKeys::Enum) }

    fn schema_to_json(schema: &Vec<NP_Parsed_Schema>, address: usize)-> Result<NP_JSON, NP_Error> {
        let mut schema_json = JSMAP::new();
        schema_json.insert("type".to_owned(), NP_JSON::String(Self::type_idx().0.to_string()));

        let data = unsafe { &*(*schema[address].data as *const NP_Enum_Data) };

        let options: Vec<NP_JSON> = data.choices.iter().map(|value| {
            NP_JSON::String(value.to_string())
        }).collect();
    
        if let Some(d) = &data.default {
            if let NP_Enum::Some(x) = &d {
                schema_json.insert("default".to_owned(), NP_JSON::String(x.to_string()));
            }
        }

        schema_json.insert("choices".to_owned(), NP_JSON::Array(options));
        

        Ok(NP_JSON::Dictionary(schema_json))
    }

    fn set_from_json<'set>(_depth: usize, _apply_null: bool, cursor: NP_Cursor, memory: &'set NP_Memory, value: &Box<NP_JSON>) -> Result<(), NP_Error> where Self: 'set + Sized {
        match &**value {
            NP_JSON::String(x) => {
                Self::set_value(cursor, memory, Self::new(x.clone()))?;
            },
            _ => { }
        }

        Ok(())
    }

    fn set_value<'set>(cursor: NP_Cursor, memory: &'set NP_Memory, value: Self) -> Result<NP_Cursor, NP_Error> where Self: 'set + Sized {

        let c_value = || { cursor.get_value(memory) };

        let data = unsafe { &*(*memory.get_schema(cursor.schema_addr).data as *const NP_Enum_Data) };

        let mut value_num: i32 = -1;

        {
            let mut ct: u16 = 0;

            for opt in &data.choices {
                if opt == &value {
                    value_num = ct as i32;
                }
                ct += 1;
            };

            if value_num == -1 {
                return Err(NP_Error::new("Option not found, cannot set uknown option!"));
            }
        }

        let bytes = value_num as u8;

        let mut addr_value = c_value().get_addr_value() as usize;

        if addr_value != 0 { // existing value, replace

            let write_bytes = memory.write_bytes();

            write_bytes[addr_value] = bytes;
            return Ok(cursor);

        } else { // new value

            addr_value = memory.malloc_borrow(&[bytes])?;
            cursor.get_value_mut(memory).set_addr_value(addr_value as u32);

            return Ok(cursor);
        }     
                     
    }

    fn schema_to_idl(schema: &Vec<NP_Parsed_Schema>, address: usize)-> Result<String, NP_Error> {
        let mut result = String::from("enum({");

        let data = unsafe { &*(*schema[address].data as *const NP_Enum_Data) };

        if let Some(x) = &data.default {
            if let NP_Enum::Some(stri) = x {
                result.push_str("default: \"");
                result.push_str(&stri);
                result.push_str("\", ");
            }
        }

        result.push_str("choices: [");

        let last_choice = data.choices.len() - 1;
        for (idx, choice) in data.choices.iter().enumerate() {
            result.push_str("\"");
            if let NP_Enum::Some(stri) = choice {
                result.push_str(stri.as_str());
            }
            result.push_str("\"");
            if idx < last_choice {
                result.push_str(", ");
            }
        }
        result.push_str("]");
       

        result.push_str("})");

        Ok(result)
    }

    fn from_idl_to_schema(mut schema: Vec<NP_Parsed_Schema>, _name: &str, idl: &JS_Schema, args: &Vec<JS_AST>) -> Result<(bool, Vec<u8>, Vec<NP_Parsed_Schema>), NP_Error> {
        let mut schema_data: Vec<u8> = Vec::new();
        schema_data.push(NP_TypeKeys::Enum as u8);

        let mut choices: Vec<NP_Enum> = Vec::new();

        let mut default_stir: Option<String> = None;

        let mut default_value: Option<NP_Enum> = None;
        let mut default_index: Option<u8> = None;

        if args.len() > 0 {
            match &args[0] {
                JS_AST::object { properties } => {
                    for (key, value) in properties {
                        match idl.get_str(key).trim() {
                            "default" => {
                                match value {
                                    JS_AST::string { addr } => {
                                        default_stir = Some(String::from(idl.get_str(addr)));
                                    },
                                    _ => { }
                                }
                            },
                            "choices" => {
                                match value {
                                    JS_AST::array { values } => {
                                        for choice in values {
                                            match choice {
                                                JS_AST::string { addr } => {
                                                    let stir = idl.get_str(addr);
                                                    if stir.len() > 255 {
                                                        return Err(NP_Error::new("'enum' choices cannot be longer than 255 characters each!"))
                                                    }
                                                    choices.push(NP_Enum::new(String::from(stir)));
                                                },
                                                _ => { }
                                            }
                                        }
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

        if choices.len() > 254 {
            return Err(NP_Error::new("Enum types cannot have more than 254 choices!"))
        } else if choices.len() == 0 {
            return Err(NP_Error::new("Enum types must have at least one choice!"))
        }

        if let Some(x) = &default_stir {
            for (idx, choice) in choices.iter().enumerate() {
                if x == choice.to_str() {
                    default_value = Some(choice.clone());
                    default_index = Some(idx as u8);
                }
            }
        }

        // default value
        match &default_index {
            Some(x) => schema_data.push(*x + 1),
            None => schema_data.push(0)
        }

        // choices
        schema_data.push(choices.len() as u8);
        for choice in &choices {
            schema_data.push(choice.len() as u8);
            schema_data.extend(choice.as_bytes().to_vec())
        }

        schema.push(NP_Parsed_Schema { 
            val: NP_Value_Kind::Fixed(1),
            i: NP_TypeKeys::Enum,
            sortable: true,
            data: Arc::new(Box::into_raw(Box::new(NP_Enum_Data { choices, default: default_value})) as *const u8)
        });

        return Ok((true, schema_data, schema));
    }

    fn into_value(cursor: &NP_Cursor, memory: &'value NP_Memory) -> Result<Option<Self>, NP_Error> where Self: Sized {

        let c_value = || { cursor.get_value(memory) };

        let value_addr = c_value().get_addr_value() as usize;

        // empty value
        if value_addr == 0 {
            return Ok(None);
        }

        let data = unsafe { &*(*memory.get_schema(cursor.schema_addr).data as *const NP_Enum_Data) };
  
        Ok(match memory.get_1_byte(value_addr) {
            Some(x) => {
                let value_num = x as usize;

                if value_num > data.choices.len() {
                    None
                } else {
                    Some(data.choices[value_num].clone())
                }
            },
            None => None
        })
        
    }

    fn default_value(_depth: usize, schema_addr: usize,schema: &Vec<NP_Parsed_Schema>) -> Option<Self> {

        let data = unsafe { &*(*schema[schema_addr].data as *const NP_Enum_Data) };


        if let Some(d) = &data.default {
            Some(d.clone())
        } else {
            None
        }
           
    }

    fn to_json(_depth:usize, cursor: &NP_Cursor, memory: &'value NP_Memory) -> NP_JSON {

        match Self::into_value(cursor, memory) {
            Ok(x) => {
                match x {
                    Some(y) => {
                        match y {
                            NP_Enum::Some(str_value) => {
                                NP_JSON::String(str_value.to_string())
                            },
                            NP_Enum::None => {
                                let data = unsafe { &*(*memory.get_schema(cursor.schema_addr).data as *const NP_Enum_Data) };

                                if let Some(d) = &data.default {
                                    match d {
                                        NP_Enum::Some(val) => {
                                            NP_JSON::String(val.clone())
                                        },
                                        NP_Enum::None => {
                                            NP_JSON::Null
                                        }
                                    }
                                } else {
                                    NP_JSON::Null
                                }
                                  
                            }
                        }
                    },
                    None => {
                        let data = unsafe { &*(*memory.get_schema(cursor.schema_addr).data as *const NP_Enum_Data) };

                        if let Some(d) = &data.default {
                            match d {
                                NP_Enum::Some(x) => NP_JSON::String(x.clone()),
                                NP_Enum::None => NP_JSON::Null
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

        let value_address = c_value().get_addr_value() as usize;

        if value_address == 0 {
            return Ok(0) 
        } else {
            Ok(core::mem::size_of::<u8>())
        }
    }

    fn from_json_to_schema(mut schema: Vec<NP_Parsed_Schema>, json_schema: &Box<NP_JSON>) -> Result<(bool, Vec<u8>, Vec<NP_Parsed_Schema>), NP_Error> {


        let mut schema_data: Vec<u8> = Vec::new();
        schema_data.push(NP_TypeKeys::Enum as u8);

        let mut choices: Vec<NP_Enum> = Vec::new();

        let mut default_stir: Option<String> = None;

        match &json_schema["default"] {
            NP_JSON::String(def) => {
                default_stir = Some(def.clone());
            },
            _ => {}
        }

        let mut default_value: Option<NP_Enum> = None;
        let mut default_index: Option<u8> = None;

        match &json_schema["choices"] {
            NP_JSON::Array(x) => {
                for opt in x {
                    match opt {
                        NP_JSON::String(stir) => {
                            if stir.len() > 255 {
                                return Err(NP_Error::new("'option' choices cannot be longer than 255 characters each!"))
                            }

                            if let Some(def) = &default_stir {
                                if def == stir {
                                    default_value = Some(NP_Enum::new(def.clone()));
                                    default_index = Some(choices.len() as u8);
                                }
                            }
                            choices.push(NP_Enum::new(stir.clone()));
                        },
                        _ => {}
                    }
                }
            },
            _ => {
                return Err(NP_Error::new("'option' type requires a 'choices' key with an array of strings!"))
            }
        }

        if choices.len() > 254 {
            return Err(NP_Error::new("'option' type cannot have more than 254 choices!"))
        }

        // default value
        match &default_index {
            Some(x) => schema_data.push(*x + 1),
            None => schema_data.push(0)
        }

        // choices
        schema_data.push(choices.len() as u8);
        for choice in &choices {
            schema_data.push(choice.len() as u8);
            schema_data.extend(choice.as_bytes().to_vec())
        }

        schema.push(NP_Parsed_Schema { 
            val: NP_Value_Kind::Fixed(1),
            i: NP_TypeKeys::Enum,
            sortable: true,
            data: Arc::new(Box::into_raw(Box::new(NP_Enum_Data { choices: choices, default: default_value })) as *const u8)
        });

        return Ok((true, schema_data, schema));
    
    }

    fn from_bytes_to_schema(mut schema: Vec<NP_Parsed_Schema>, address: usize, bytes: &[u8]) -> (bool, Vec<NP_Parsed_Schema>) {
        let mut default_index: Option<u8> = None;
        let mut default_value: Option<NP_Enum> = None;

        if bytes[address + 1] > 0 {
            default_index = Some(bytes[address + 1] - 1);
        }

        let choices_len = bytes[address + 2];

        let mut choices: Vec<NP_Enum> = Vec::new();
        let mut offset: usize = address + 3;
        for x in 0..choices_len {
            let choice_size = bytes[offset] as usize;
            let choice_bytes = &bytes[(offset + 1)..(offset + 1 + choice_size)];
            let choice_string = unsafe { core::str::from_utf8_unchecked(choice_bytes) };
            choices.push(NP_Enum::new(choice_string.to_string()));
            offset += 1 + choice_size;

            if let Some(def) = default_index {
                if def == x {
                    default_value = Some(NP_Enum::new(choice_string.to_string()));
                }
            }
        }

        schema.push(NP_Parsed_Schema {
            val: NP_Value_Kind::Fixed(1),
            i: NP_TypeKeys::Enum,
            sortable: true,
            data: Arc::new(Box::into_raw(Box::new(NP_Enum_Data { choices: choices, default: default_value })) as *const u8)
        });

        (true, schema)
    }
}

#[test]
fn schema_parsing_works_idl() -> Result<(), NP_Error> {
    let schema = r#"enum({default: "hello", choices: ["hello", "world"]})"#;
    let factory = crate::NP_Factory::new(schema)?;
    assert_eq!(schema, factory.schema.to_idl()?);
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_idl()?);

    let schema = r#"enum({choices: ["hello", "world"]})"#;
    let factory = crate::NP_Factory::new(schema)?;
    assert_eq!(schema, factory.schema.to_idl()?);
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_idl()?);
    
    Ok(())
}

#[test]
fn schema_parsing_works() -> Result<(), NP_Error> {
    let schema = "{\"type\":\"option\",\"default\":\"hello\",\"choices\":[\"hello\",\"world\"]}";
    let factory = crate::NP_Factory::new_json(schema)?;
    assert_eq!(schema, factory.schema.to_json()?.stringify());
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_json()?.stringify());

    let schema = "{\"type\":\"option\",\"choices\":[\"hello\",\"world\"]}";
    let factory = crate::NP_Factory::new_json(schema)?;
    assert_eq!(schema, factory.schema.to_json()?.stringify());
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_json()?.stringify());
    
    Ok(())
}


#[test]
fn default_value_works() -> Result<(), NP_Error> {
    let schema = "{\"type\":\"option\",\"default\":\"hello\",\"choices\":[\"hello\",\"world\"]}";
    let factory = crate::NP_Factory::new_json(schema)?;
    let buffer = factory.new_buffer(None);
    assert_eq!(buffer.get::<NP_Enum>(&[])?.unwrap(), NP_Enum::new("hello"));

    Ok(())
}

#[test]
fn set_clear_value_and_compaction_works() -> Result<(), NP_Error> {
    let schema = "{\"type\":\"option\",\"choices\":[\"hello\",\"world\"]}";
    let factory = crate::NP_Factory::new_json(schema)?;
    let mut buffer = factory.new_buffer(None);
    buffer.set(&[], NP_Enum::new("hello"))?;
    assert_eq!(buffer.get::<NP_Enum>(&[])?, Some(NP_Enum::new("hello")));
    buffer.del(&[])?;
    assert_eq!(buffer.get::<NP_Enum>(&[])?, None);

    buffer.compact(None)?;
    assert_eq!(buffer.calc_bytes()?.current_buffer, 6usize);

    Ok(())
}