//! Internal buffer memory management

use crate::{schema::NP_Parsed_Schema};
use crate::{error::NP_Error};
use core::cell::UnsafeCell;
use alloc::vec::Vec;




#[doc(hidden)]
pub trait NP_Memory {
    fn is_mutable(&self) -> bool;
    fn get_root(&self) -> usize;
    fn get_schemas(&self) -> &Vec<NP_Parsed_Schema>;
    fn get_schema(&self, idx: usize) -> &NP_Parsed_Schema;
    fn malloc_borrow(&self, bytes: &[u8])  -> Result<usize, NP_Error>;
    fn malloc(&self, bytes: Vec<u8>) -> Result<usize, NP_Error>;
    fn read_bytes(&self) -> &[u8];
    fn write_bytes(&self) -> &mut [u8];
    fn get_1_byte(&self, address: usize) -> Option<u8>;
    fn get_2_bytes(&self, address: usize) -> Option<&[u8; 2]>;
    fn get_4_bytes(&self, address: usize) -> Option<&[u8; 4]>;
    fn get_8_bytes(&self, address: usize) -> Option<&[u8; 8]>;
    fn get_16_bytes(&self, address: usize) -> Option<&[u8; 16]>;
    fn get_32_bytes(&self, address: usize) -> Option<&[u8; 32]>;
    fn dump(self) -> Vec<u8>;
}

#[doc(hidden)]
#[derive(Debug)]
pub struct NP_Memory_Writable {
    bytes: UnsafeCell<Vec<u8>>,
    pub root: usize,
    pub schema: Vec<NP_Parsed_Schema>
}

#[doc(hidden)]
impl NP_Memory_Writable {

    pub fn clone(&self) -> Self {
        Self {
            root: self.root,
            bytes: UnsafeCell::new(self.read_bytes().to_vec()),
            schema: self.schema.clone()
        }
    }

    #[inline(always)]
    pub fn existing(bytes: Vec<u8>, schema: Vec<NP_Parsed_Schema>, root: usize) -> Self {

        Self {
            root,
            bytes: UnsafeCell::new(bytes),
            schema: schema
        }
    }

    #[inline(always)]
    pub fn new(capacity: Option<usize>, schema: Vec<NP_Parsed_Schema>, root: usize) -> Self {
        let use_size = match capacity {
            Some(x) => x,
            None => 1024
        };

        let mut new_bytes = Vec::with_capacity(use_size);

        // size, root pointer
        new_bytes.extend(&[0u8; 3]);

        Self {
            root,
            bytes: UnsafeCell::new(new_bytes),
            schema: schema,
        }
    }

}

impl NP_Memory for NP_Memory_Writable {

    #[inline(always)]
    fn is_mutable(&self) -> bool {
        true
    }

    #[inline(always)]
    fn get_root(&self) -> usize {
        self.root
    }

    #[inline(always)]
    fn get_schemas(&self) -> &Vec<NP_Parsed_Schema> {
        &self.schema
    }

    #[inline(always)]
    fn get_schema(&self, idx: usize) -> &NP_Parsed_Schema {
        &self.schema[idx]
    }

    #[inline(always)]
    fn malloc_borrow(&self, bytes: &[u8])  -> Result<usize, NP_Error> {
        let self_bytes = unsafe { &mut *self.bytes.get() };

        let location = self_bytes.len();

        // not enough space left?
        if location + bytes.len() >= core::u16::MAX as usize {
            return Err(NP_Error::new("Not enough space available in buffer!"))
        }

        self_bytes.extend(bytes);
        Ok(location)
    }

    #[inline(always)]
    fn malloc(&self, bytes: Vec<u8>) -> Result<usize, NP_Error> {
        self.malloc_borrow(&bytes)
    }

    #[inline(always)]
    fn read_bytes(&self) -> &[u8] {
        let self_bytes = unsafe { &*self.bytes.get() };
        self_bytes
    }   

    #[inline(always)]
    fn write_bytes(&self) -> &mut [u8] {
        let self_bytes = unsafe { &mut *self.bytes.get() };
        self_bytes
    }

    #[inline(always)]
    fn get_1_byte(&self, address: usize) -> Option<u8> {

        // empty value
        if address == 0 {
            return None;
        }

        let self_bytes = unsafe { &*self.bytes.get() };
 
        Some(self_bytes[address])
    }

    #[inline(always)]
    fn get_2_bytes(&self, address: usize) -> Option<&[u8; 2]> {

        // empty value
        if address == 0 {
            return None;
        }

        let self_bytes = unsafe { &*self.bytes.get() };

        if self_bytes.len() < address + 2 {
            return None;
        }

        let slice = &self_bytes[address..(address + 2)];

        Some(unsafe { &*(slice as *const [u8] as *const [u8; 2]) })
    }

    #[inline(always)]
    fn get_4_bytes(&self, address: usize) -> Option<&[u8; 4]> {

        // empty value
        if address == 0 {
            return None;
        }

        let self_bytes = unsafe { &*self.bytes.get() };

        if self_bytes.len() < address + 4 {
            return None;
        }

        let slice = &self_bytes[address..(address + 4)];

        Some(unsafe { &*(slice as *const [u8] as *const [u8; 4]) })
    }

    #[inline(always)]
    fn get_8_bytes(&self, address: usize) -> Option<&[u8; 8]> {

        // empty value
        if address == 0 {
            return None;
        }

        let self_bytes = unsafe { &*self.bytes.get() };

        if self_bytes.len() < address + 8 {
            return None;
        }

        let slice = &self_bytes[address..(address + 8)];

        Some(unsafe { &*(slice as *const [u8] as *const [u8; 8]) })
    }

    #[inline(always)]
    fn get_16_bytes(&self, address: usize) -> Option<&[u8; 16]> {

        // empty value
        if address == 0 {
            return None;
        }

        let self_bytes = unsafe { &*self.bytes.get() };

        if self_bytes.len() < address + 16 {
            return None;
        }

        let slice = &self_bytes[address..(address + 16)];

        Some(unsafe { &*(slice as *const [u8] as *const [u8; 16]) })
    }

    #[inline(always)]
    fn get_32_bytes(&self, address: usize) -> Option<&[u8; 32]> {

        // empty value
        if address == 0 {
            return None;
        }

        let self_bytes = unsafe { &*self.bytes.get() };

        if self_bytes.len() < address + 32 {
            return None;
        }

        let slice = &self_bytes[address..(address + 32)];

        Some(unsafe { &*(slice as *const [u8] as *const [u8; 32]) })
    }

    fn dump(self) -> Vec<u8> {
        self.bytes.into_inner()
    }
}
