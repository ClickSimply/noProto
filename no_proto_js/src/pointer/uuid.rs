//! Represents a V4 UUID, good for globally unique identifiers
//! 
//! `uuid` types are always represented with this struct.
//! 
//! ```
//! use no_proto::error::NP_Error;
//! use no_proto::NP_Factory;
//! use no_proto::pointer::uuid::NP_UUID;
//! 
//! let factory: NP_Factory = NP_Factory::new(r#"{
//!    "type": "uuid"
//! }"#)?;
//!
//! let mut new_buffer = factory.empty_buffer(None);
//! let uuid = NP_UUID::generate(50);
//! new_buffer.set(&[], &uuid)?;
//! 
//! assert_eq!("48E6AAB0-7DF5-409F-4D57-4D969FA065EE", new_buffer.get::<&NP_UUID>(&[])?.unwrap().to_string());
//!
//! # Ok::<(), NP_Error>(()) 
//! ```
//! 

use alloc::prelude::v1::Box;
use crate::pointer::NP_Scalar;
use crate::{memory::NP_Memory, schema::{NP_Parsed_Schema}};
use alloc::vec::Vec;
use crate::json_flex::{JSMAP, NP_JSON};
use crate::schema::{NP_TypeKeys};
use crate::{pointer::NP_Value, error::NP_Error, utils::{Rand}};
use core::{fmt::{Debug, Formatter, Write}};

use alloc::string::String;
use alloc::borrow::ToOwned;
use alloc::string::ToString;

use super::NP_Cursor;


/// Holds UUID which is good for random keys.
/// 
/// Check out documentation [here](../uuid/index.html).
/// 
#[derive(Eq, PartialEq, Clone)]
#[repr(C)]
pub struct NP_UUID {
    /// The random bytes for this UUID
    pub value: [u8; 16]
}

impl NP_Scalar for &NP_UUID {}

/// ULID alias for shared value
pub type _NP_UUID<'a> = &'a NP_UUID;

impl NP_UUID {

    /// Generate a new UUID with a given random seed.  You should attempt to provide a seed with as much randomness as possible.
    /// 
    pub fn generate(random_seed: u32) -> Self {


        let mut uuid = NP_UUID {
            value: [0; 16]
        };

        let mut rng = Rand::new(random_seed);

        for x in 0..uuid.value.len() {
            if x == 6 {
                uuid.value[x] = 64 + rng.gen_range(0, 15) as u8;
            } else {
                uuid.value[x] = rng.gen_range(0, 255) as u8;
            }
        }

        uuid
    }

    /// Generates a UUID with a provided random number generator.
    /// This is the preferrable way to generate a ULID, if you can provide a better RNG function than the psudorandom one built into this library, you should.
    /// 
    pub fn generate_with_rand<F>(random_fn: F) -> Self where F: Fn() -> u8 {
        let mut uuid = NP_UUID {
            value: [0; 16]
        };

        for x in 0..uuid.value.len() {
            if x == 6 {
                uuid.value[x] = 64 + (random_fn() % 17) - 1;
            } else {
                uuid.value[x] = random_fn();
            }
        }

        uuid
    }// 503 760 4833

    /// Convert a string UUID into it's byte values
    /// 
    pub fn from_string(uuid: &str) -> NP_UUID {
        let cleaned: String = String::from(uuid).replace("-", "");

        let mut value: [u8; 16] = [0; 16];

        for x in 0..16usize {
            let step = x * 2;
            match u8::from_str_radix(&cleaned[step..(step + 2)], 16) {
                Ok(byte) => { value[x] = byte },
                _ => {}
            }
        }

        NP_UUID { value }
    }

    /// Generates a stringified version of the UUID.
    /// 
    pub fn to_string(&self) -> String {

        let mut result = String::with_capacity(32);

        for x in 0..self.value.len() {
            if x == 4 || x == 6 || x == 8 || x == 10 {
                result.push_str("-");
            }
            let byte = self.value[x] as u8;
            write!(result, "{:02X}", byte).unwrap_or(());
        }

        result
    }
}

impl Debug for NP_UUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Default for NP_UUID {
    fn default() -> Self { 
        NP_UUID { value: [0; 16] }
     }
}

impl<'value> NP_Value<'value> for &NP_UUID {

    fn type_idx() -> (&'value str, NP_TypeKeys) { ("uuid", NP_TypeKeys::Uuid) }
    fn self_type_idx(&self) -> (&'value str, NP_TypeKeys) { ("uuid", NP_TypeKeys::Uuid) }

    fn schema_to_json(_schema: &Vec<NP_Parsed_Schema>, _address: usize)-> Result<NP_JSON, NP_Error> {
        let mut schema_json = JSMAP::new();
        schema_json.insert("type".to_owned(), NP_JSON::String(Self::type_idx().0.to_string()));

        Ok(NP_JSON::Dictionary(schema_json))
    }

    fn set_value<'set, M: NP_Memory>(cursor: NP_Cursor, memory: &'set M, value: Self) -> Result<NP_Cursor, NP_Error> where Self: 'set + Sized {

        let c_value = cursor.get_value(memory);

        let mut value_address = c_value.get_addr_value() as usize;

        if value_address != 0 { // existing value, replace
            let bytes = value.value;
            let write_bytes = memory.write_bytes();

            // overwrite existing values in buffer
            for x in 0..bytes.len() {
                write_bytes[value_address + x] = bytes[x];
            }

        } else { // new value

            value_address = memory.malloc_borrow(&value.value)?;
            c_value.set_addr_value(value_address as u16);
        }                    
        
        Ok(cursor)
    }

    fn into_value<M: NP_Memory>(cursor: &NP_Cursor, memory: &'value M) -> Result<Option<Self>, NP_Error> where Self: Sized {

        let c_value = cursor.get_value(memory);

        let value_addr = c_value.get_addr_value();

        // empty value
        if value_addr == 0 {
            return Ok(None);
        }

        Ok(match memory.get_16_bytes(value_addr as usize) {
            Some(x) => {
                Some(unsafe { &*(x.as_ptr() as *const NP_UUID) })
            },
            None => None
        })
    }

    fn to_json<M: NP_Memory>(cursor: &NP_Cursor, memory: &'value M) -> NP_JSON {

        match Self::into_value(cursor, memory) {
            Ok(x) => {
                match x {
                    Some(y) => {
                        NP_JSON::String(y.to_string())
                    },
                    None => {
                        NP_JSON::Null
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

        if c_value.get_addr_value() == 0 {
            Ok(0) 
        } else {
            Ok(16)
        }
    }

    fn from_json_to_schema(mut schema: Vec<NP_Parsed_Schema>, _json_schema: &Box<NP_JSON>) -> Result<(bool, Vec<u8>, Vec<NP_Parsed_Schema>), NP_Error> {

       
        let mut schema_bytes: Vec<u8> = Vec::new();
        schema_bytes.push(NP_TypeKeys::Uuid as u8);
        schema.push(NP_Parsed_Schema::Uuid { 
            i: NP_TypeKeys::Uuid,
            sortable: true
        });
        return Ok((true, schema_bytes, schema))
    
    }

    fn default_value(_schema: &NP_Parsed_Schema) -> Option<Self> {
        None
    }

    fn from_bytes_to_schema(mut schema: Vec<NP_Parsed_Schema>, _address: usize, _bytes: &[u8]) -> (bool, Vec<NP_Parsed_Schema>) {
        schema.push(NP_Parsed_Schema::Uuid {
            i: NP_TypeKeys::Uuid,
            sortable: true
        });
        (true, schema)
    }
}
