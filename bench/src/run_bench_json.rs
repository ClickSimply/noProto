use crate::LOOPS;

use std::{io::prelude::*, str::{from_utf8_unchecked}};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use json::{JsonValue};
use std::time::{SystemTime};


pub struct JSONBench();

impl JSONBench {

    pub fn size_bench() -> (usize, usize) {

        let encoded = Self::encode_single();

        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write(&encoded[..]).unwrap();
        let compressed = e.finish().unwrap();

        println!("JSON:        size: {}b, zlib: {}b", encoded.len(), compressed.len());
        return (encoded.len(), compressed.len())
    }

    pub fn encode_bench(base: u128) -> String {
        let start = SystemTime::now();

        for _x in 0..LOOPS {
            let buffer = Self::encode_single();
            assert_eq!(buffer.len(), 439);
        }

        let time = SystemTime::now().duration_since(start).expect("Time went backwards");
        println!("JSON:        {:>9.0} ops/ms {:.2}", LOOPS as f64 / time.as_millis() as f64, (base as f64 / time.as_micros() as f64));

        format!("{:>6.0}", LOOPS as f64 / time.as_millis() as f64)
    }

    #[inline(always)]
    fn encode_single() -> Vec<u8> {
        let mut json_object = object!{
            fruit: 2,
            initialized: true,
            location: "http://arstechnica.com",
            list: []
        };

        for x in 0..3 {
            json_object["list"][x] = object!{
                name: "Hello, World!",
                rating: 3.1415432432445543543 + (x as f64),
                postfix: "!",
                sibling: {
                    time: 123456 + (x as i32),
                    ratio: 3.14159,
                    size: 10000 + (x as u16)
                }
            };
        }


        json_object.dump().as_bytes().to_vec()
    }



    pub fn update_bench(base: u128) -> String  {
        let buffer = Self::encode_single();

        let start = SystemTime::now();

        for _x in 0..LOOPS {
            let mut container = json::parse(unsafe { from_utf8_unchecked(&buffer) }).unwrap();

            container["list"][0]["name"] = JsonValue::String(String::from("bob"));

            assert_eq!(container.dump().len(), 429);
        }

        let time = SystemTime::now().duration_since(start).expect("Time went backwards");
        println!("JSON:        {:>9.0} ops/ms {:.2}", LOOPS as f64 / time.as_millis() as f64, (base as f64 / time.as_micros() as f64));
        format!("{:>6.0}", LOOPS as f64 / time.as_millis() as f64)
    }

    pub fn decode_one_bench(base: u128) -> String  {
        let buffer = Self::encode_single();

        let start = SystemTime::now();

        for _x in 0..LOOPS {
            let container = json::parse(unsafe { from_utf8_unchecked(&buffer) }).unwrap();
            assert_eq!(container["location"], JsonValue::String(String::from("http://arstechnica.com")));
        }

        let time = SystemTime::now().duration_since(start).expect("Time went backwards");
        println!("JSON:        {:>9.0} ops/ms {:.2}", LOOPS as f64 / time.as_millis() as f64, (base as f64 / time.as_micros() as f64));
        format!("{:>6.0}", LOOPS as f64 / time.as_millis() as f64)
    }

    pub fn decode_bench(base: u128) -> String {
        let buffer = Self::encode_single();

        let start = SystemTime::now();

        let hello_world = JsonValue::String(String::from("Hello, World!"));
        let ars_technica = JsonValue::String(String::from("http://arstechnica.com"));

        for _x in 0..LOOPS {
            let container = json::parse(unsafe { from_utf8_unchecked(&buffer) }).unwrap();

            assert_eq!(container["location"], ars_technica);
            assert_eq!(container["fruit"].as_f64(), Some(2.0f64));
            assert_eq!(container["initialized"], JsonValue::Boolean(true));
            let mut loops = 0;
            if let JsonValue::Array(list) = &container["list"] {
                
                list.iter().enumerate().for_each(|(x, foobar)| {
                    loops += 1;
                    assert_eq!(foobar["name"], hello_world);
                    assert_eq!(foobar["rating"].as_f64().unwrap(), 3.1415432432445543543 + (x as f64));
                    assert_eq!(foobar["postfix"], JsonValue::String(String::from("!")));
                    let sibling = &foobar["sibling"];
                    assert_eq!(sibling["time"].as_f64().unwrap(), 123456f64 + (x as f64));
                    assert_eq!(sibling["ratio"].as_f64().unwrap(), 3.14159);
                    assert_eq!(sibling["size"].as_f64().unwrap(), 10000f64 + (x as f64));
                });
            }
            assert!(loops == 3);
        }

        let time = SystemTime::now().duration_since(start).expect("Time went backwards");
        println!("JSON:        {:>9.0} ops/ms {:.2}", LOOPS as f64 / time.as_millis() as f64, (base as f64 / time.as_micros() as f64));
        format!("{:>6.0}", LOOPS as f64 / time.as_millis() as f64)
    }
}
