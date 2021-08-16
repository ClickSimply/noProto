//! Represents a ULID type which has a 6 byte timestamp and 10 bytes of randomness
//! 
//! Useful for storing time stamp data that doesn't have collisions.
//! 
//! ```
//! use no_proto::error::NP_Error;
//! use no_proto::NP_Factory;
//! use no_proto::pointer::ulid::NP_ULID;
//! 
//! let factory: NP_Factory = NP_Factory::new(r#"{
//!    "type": "ulid"
//! }"#)?;
//!
//! let mut new_buffer = factory.empty_buffer(None);
//! let ulid = NP_ULID::generate(1604965249484, 50);
//! new_buffer.set(&[], &ulid)?;
//! 
//! assert_eq!("1EPQP4CEC3KANC3XYNG9YKAQ", new_buffer.get::<&NP_ULID>(&[])?.unwrap().to_string());
//!
//! # Ok::<(), NP_Error>(()) 
//! ```
//! 

use crate::{memory::NP_Memory, schema::{NP_Parsed_Schema}};
use alloc::vec::Vec;
use crate::utils::to_base32;
use crate::json_flex::{JSMAP, NP_JSON};
use crate::schema::{NP_TypeKeys};
use crate::{pointer::NP_Value, error::NP_Error, utils::{Rand}};
use core::{fmt::{Debug, Formatter}};

use alloc::string::String;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::borrow::ToOwned;

use super::NP_Cursor;


/// Holds ULIDs which are good for time series keys.
/// 
/// Check out documentation [here](../ulid/index.html).
/// 
#[derive(Eq, PartialEq, Clone)]
#[repr(C)]
pub struct NP_ULID {
    value: [u8; 16]
}

/// ULID alias for shared type
pub type _NP_ULID<'a> = &'a NP_ULID;

impl<'value> super::NP_Scalar<'value> for &NP_ULID {}

impl NP_ULID {

    /// Creates a new ULID from the timestamp and provided seed.
    /// 
    /// The random seed is used to generate the ID, the same seed will always lead to the same random bytes so try to use something actually random for the seed.
    /// 
    /// The time should be passed in as the unix epoch in milliseconds.
    pub fn generate(now_ms: u64, random_seed: u32) -> NP_ULID {
        let mut rng = Rand::new(random_seed);

        let mut id: [u8; 16] = [0; 16];

        let time_bytes = now_ms.to_be_bytes();

        for x in 0..id.len() {
            if x < 6 {
                id[x] = time_bytes[x + 2];
            } else {
                id[x] = rng.gen_range(0, 255) as u8;
            }
        }

        NP_ULID {
            value: id
        }
    }

    /// Generates a ULID with the given time and a provided random number generator.
    /// This is the preferrable way to generate a ULID, if you can provide a better RNG function than the psudorandom one built into this library, you should.
    /// 
    pub fn generate_with_rand<F>(now_ms: u64, random_fn: F) -> NP_ULID where F: Fn() -> u8 {

        let mut id: [u8; 16] = [0; 16];

        let time_bytes = now_ms.to_be_bytes();

        for x in 0..id.len() {
            if x < 6 {
                id[x] = time_bytes[x + 2];
            } else {
                id[x] = random_fn();
            }
        }

        NP_ULID {
            value: id
        }
    }
    
    /// Get just the time component for this ULID
    pub fn get_time(&self) -> u64 {
        let mut time_bytes: [u8; 8] = [0; 8];
        for (i, x) in self.value.iter().take(6).enumerate() {
            time_bytes[i + 2] = *x;
        }
        u64::from_be_bytes(time_bytes)
    }

    /// Generates a stringified version of this ULID with base32.
    /// 
    pub fn to_string(&self) -> String {
        let mut result: String = "".to_owned();

        let mut time_bytes: [u8; 16] = [0; 16];
        let mut rand_bytes: [u8; 16] = [0; 16];

        for (i, x) in self.value.iter().enumerate() {
            if i < 6 {
                time_bytes[i + 10] = *x;
            } else {
                rand_bytes[i] = *x;
            }
        }

        result.push_str(to_base32(u128::from_be_bytes(time_bytes), 10).as_str());
        result.push_str(to_base32(u128::from_be_bytes(rand_bytes), 16).as_str());

        result
    }
}


impl Default for NP_ULID {
    fn default() -> Self { 
        NP_ULID { value: [0u8; 16]}
     }
}

impl Debug for NP_ULID {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<'value> NP_Value<'value> for &NP_ULID {

    fn type_idx() -> (&'value str, NP_TypeKeys) { ("ulid", NP_TypeKeys::Ulid) }
    fn self_type_idx(&self) -> (&'value str, NP_TypeKeys) { ("ulid", NP_TypeKeys::Ulid) }

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
                Some(unsafe { &*(x.as_ptr() as *const NP_ULID) })
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
        schema_bytes.push(NP_TypeKeys::Ulid as u8);
        schema.push(NP_Parsed_Schema::Ulid { 
            i: NP_TypeKeys::Ulid,
            sortable: true
        });
        return Ok((true, schema_bytes, schema))

    }

    fn default_value(_schema: &NP_Parsed_Schema) -> Option<Self> {
        None
    }

    fn from_bytes_to_schema(mut schema: Vec<NP_Parsed_Schema>, _address: usize, _bytes: &[u8]) -> (bool, Vec<NP_Parsed_Schema>) {
        schema.push(NP_Parsed_Schema::Ulid {
            i: NP_TypeKeys::Ulid,
            sortable: true
        });
        (true, schema)
    }
}

