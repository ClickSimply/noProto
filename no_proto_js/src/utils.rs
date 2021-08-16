use core::str;
use alloc::string::String;
use alloc::borrow::ToOwned;
use alloc::vec::Vec;

use crate::error::NP_Error;

const KX: u32 = 123456789;
const KY: u32 = 362436069;
const KZ: u32 = 521288629;
const KW: u32 = 88675123;

#[inline(always)]
pub fn to_unsigned(byte: u8) -> u8 {
    if byte >= 128 { byte - 128 } else { byte + 128 }
}

#[inline(always)]
pub fn to_signed(byte: u8) -> u8 {
    if byte < 128 { byte + 128 } else { byte - 128 }
}



pub struct Rand {
    x: u32, y: u32, z: u32, w: u32
}

impl Rand {
    pub fn new(seed: u32) -> Rand {
        Rand{
            x: KX^seed, y: KY^seed,
            z: KZ, w: KW
        }
    }

    // Xorshift 128, taken from German Wikipedia
    pub fn rand(&mut self) -> u32 {
        let t = self.x^self.x.wrapping_shl(11);
        self.x = self.y; self.y = self.z; self.z = self.w;
        self.w ^= self.w.wrapping_shr(19)^t^t.wrapping_shr(8);
        return self.w;
    }

    pub fn gen_range(&mut self, a: i32, b: i32) -> i32 {
        let m = (b-a+1) as u32;
        return a+(self.rand()%m) as i32;
    }
}

#[inline(always)]
pub fn opt_err<T>(optin: Option<T>) -> Result<T, NP_Error> {
    match optin {
        Some(x) => Ok(x),
        None => Err(NP_Error::new("No value found here!"))
    }
}

pub fn to_base32(num: u128, length: i32) -> String {

    let mut result: Vec<&str> = Vec::with_capacity(length as usize);
    for _x in 0..length {
        result.push("");
    }

    let base_values: [&str; 32] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H", "J", "K", "M", "N", "P", "Q", "R", "S", "T", "V", "W", "X", "Y", "Z"];

    let mut value = num;
    let i = length - 1;
    for x in 0..i {
        let modulus = value % 32; 
        result[(i - x) as usize] = base_values[modulus as usize];
        value = (value - modulus) / 32;
    }

    let mut final_string: String = "".to_owned();
    for x in result {
        final_string.push_str(x);
    }

    final_string
}