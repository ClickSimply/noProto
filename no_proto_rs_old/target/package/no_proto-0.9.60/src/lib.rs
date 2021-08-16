#![warn(missing_docs)]
#![allow(non_camel_case_types)]
#![no_std]

//! ## NoProto: Flexible, Fast & Compact Serialization with RPC
//! 
//! <img src="https://github.com/only-cliches/NoProto/raw/master/logo_small.png"/>
//! 
//! [Github](https://github.com/only-cliches/NoProto) | [Crates.io](https://crates.io/crates/no_proto) | [Documentation](https://docs.rs/no_proto)
//! 
//! [![MIT license](https://img.shields.io/badge/License-MIT-blue.svg)](https://lbesson.mit-license.org/)
//! [![crates.io](https://img.shields.io/crates/v/no_proto.svg)](https://crates.io/crates/no_proto)
//! [![docs.rs](https://docs.rs/no_proto/badge.svg)](https://docs.rs/no_proto/latest/no_proto/)
//! [![GitHub stars](https://img.shields.io/github/stars/only-cliches/NoProto.svg?style=social&label=Star&maxAge=2592000)](https://GitHub.com/only-cliches/NoProto/stargazers/)
//! ### Features  
//! 
//! **Lightweight**<br/>
//! - Zero dependencies
//! - `no_std` support, WASM ready
//! - Most compact non compiling storage format
//! 
//! **Stable**<br/>
//! - Safely accept untrusted buffers
//! - Passes Miri compiler safety checks
//! - Panic and unwrap free
//! 
//! **Easy**<br/>
//! - Extensive Documentation & Testing
//! - Full interop with JSON, Import and Export JSON values
//! - [Thoroughly documented](https://docs.rs/no_proto/latest/no_proto/format/index.html) & simple data storage format
//! 
//! **Fast**<br/>
//! - Zero copy deserialization
//! - Most updates are append only
//! - Deserialization is incrimental
//! 
//! **Powerful**<br/>
//! - Native byte-wise sorting
//! - Supports recursive data types
//! - Supports most common native data types
//! - Supports collections (list, map, struct & tuple)
//! - Supports arbitrary nesting of collection types
//! - Schemas support default values and non destructive updates
//! - Transport agnostic [RPC Framework](https://docs.rs/no_proto/latest/no_proto/rpc/index.html).
//! 
//! 
//! ### Why ANOTHER Serialization Format?
//! 1. NoProto combines the **performance** of compiled formats with the **flexibilty** of dynamic formats:
//! 
//! **Compiled** formats like Flatbuffers, CapN Proto and bincode have amazing performance and extremely compact buffers, but you MUST compile the data types into your application.  This means if the schema of the data changes the application must be recompiled to accomodate the new schema.
//! 
//! **Dynamic** formats like JSON, MessagePack and BSON give flexibilty to store any data with any schema at runtime but the buffers are fat and performance is somewhere between horrible and hopefully acceptable.
//! 
//! NoProto takes the performance advantages of compiled formats and implements them in a flexible format.
//! 
//! 2. NoProto is a **key-value database focused format**:
//! 
//! **Byte Wise Sorting** Ever try to store a signed integer as a sortable key in a database?  NoProto can do that.  Almost every data type is stored in the buffer as byte-wise sortable, meaning buffers can be compared at the byte level for sorting *without deserializing*.
//! 
//! **Primary Key Management** Compound sortable keys are extremely easy to generate, maintain and update with NoProto. You don't need a custom sort function in your key-value store, you just need this library.
//! 
//! **UUID & ULID Support** NoProto is one of the few formats that come with first class suport for these popular primary key data types.  It can easily encode, decode and generate these data types.
//! 
//! **Fastest Updates** NoProto is the only format that supports *all mutations* without deserializng.  It can do the common database read -> update -> write operation between 50x - 300x faster than other dynamic formats. [Benchamrks](#benchmarks)
//! 
//! 
//! ### Comparison With Other Formats
//! 
//! <br/>
//! <details>
//! <summary><b>Compared to Apache Avro</b></summary>
//! - Far more space efficient<br/>
//! - Significantly faster serialization & deserialization<br/>
//! - All values are optional (no void or null type)<br/>
//! - Supports more native types (like unsigned ints)<br/>
//! - Updates without deserializng/serializing<br/>
//! - Works with `no_std`.<br/>
//! - Safely handle untrusted data.<br/>
//! </details>
//! <br/>
//! <details>
//! <summary><b>Compared to Protocol Buffers</b></summary>
//! - Comparable serialization & deserialization performance<br/>
//! - Updating buffers is an order of magnitude faster<br/>
//! - Schemas are dynamic at runtime, no compilation step<br/>
//! - All values are optional<br/>
//! - Supports more types and better nested type support<br/>
//! - Byte-wise sorting is first class operation<br/>
//! - Updates without deserializng/serializing<br/>
//! - Safely handle untrusted data.<br/>
//! - All values are optional and can be inserted in any order.<br/>
//! </details>
//! <br/>
//! <details>
//! <summary><b>Compared to JSON / BSON</b></summary>
//! - Far more space efficient<br/>
//! - Significantly faster serialization & deserialization<br/>
//! - Deserializtion is zero copy<br/>
//! - Has schemas / type safe<br/>
//! - Supports byte-wise sorting<br/>
//! - Supports raw bytes & other native types<br/>
//! - Updates without deserializng/serializing<br/>
//! - Works with `no_std`.<br/>
//! - Safely handle untrusted data.<br/>
//! </details>
//! <br/>
//! <details>
//! <summary><b>Compared to Flatbuffers / Bincode</b></summary>
//! - Data types can change or be created at runtime<br/>
//! - Updating buffers is an order of magnitude faster<br/>
//! - Supports byte-wise sorting<br/>
//! - Updates without deserializng/serializing<br/>
//! - Works with `no_std`.<br/>
//! - Safely handle untrusted data.<br/>
//! - All values are optional and can be inserted in any order.<br/>
//! </details>
//! <br/><br/>
//! 
//! | Format           | Zero-Copy | Size Limit | Mutable | Schemas  | Byte-wise Sorting |
//! |------------------|-----------|------------|---------|----------|-------------------|
//! | **Runtime Libs** |           |            |         |          |                   | 
//! | *NoProto*        | ✓         | ~4GB       | ✓       | ✓        | ✓                 |
//! | Apache Avro      | ✗         | 2^63 Bytes | ✗       | ✓        | ✓                 |
//! | JSON             | ✗         | Unlimited  | ✓       | ✗        | ✗                 |
//! | BSON             | ✗         | ~16MB      | ✓       | ✗        | ✗                 |
//! | MessagePack      | ✗         | Unlimited  | ✓       | ✗        | ✗                 |
//! | **Compiled Libs**|           |            |         |          |                   | 
//! | FlatBuffers      | ✓         | ~2GB       | ✗       | ✓        | ✗                 |
//! | Bincode          | ✓         | ?          | ✓       | ✓        | ✗                 |
//! | Protocol Buffers | ✗         | ~2GB       | ✗       | ✓        | ✗                 |
//! | Cap'N Proto      | ✓         | 2^64 Bytes | ✗       | ✓        | ✗                 |
//! | Veriform         | ✗         | ?          | ✗       | ✗        | ✗                 |
//! 
//! 
//! # Quick Example
//! ```rust
//! use no_proto::error::NP_Error;
//! use no_proto::NP_Factory;
//! 
//! // An ES6 like IDL is used to describe schema for the factory
//! // Each factory represents a single schema
//! // One factory can be used to serialize/deserialize any number of buffers
//! let user_factory = NP_Factory::new(r#"
//!     struct({ fields: {
//!         name: string(),
//!         age: u16({ default: 0 }),
//!         tags: list({ of: string() })
//!     }})
//! "#)?;
//! 
//! 
//! // create a new empty buffer
//! let mut user_buffer = user_factory.new_buffer(None); // optional capacity
//! 
//! // set the "name" field
//! user_buffer.set(&["name"], "Billy Joel")?;
//! 
//! // read the "name" field
//! let name = user_buffer.get::<&str>(&["name"])?;
//! assert_eq!(name, Some("Billy Joel"));
//! 
//! // set a nested value, the first tag in the tag list
//! user_buffer.set(&["tags", "0"], "first tag")?;
//! 
//! // read the first tag from the tag list
//! let tag = user_buffer.get::<&str>(&["tags", "0"])?;
//! assert_eq!(tag, Some("first tag"));
//! 
//! // close buffer and get internal bytes
//! let user_bytes: Vec<u8> = user_buffer.finish().bytes();
//! 
//! // open the buffer again
//! let user_buffer = user_factory.open_buffer(user_bytes);
//! 
//! // read the "name" field again
//! let name = user_buffer.get::<&str>(&["name"])?;
//! assert_eq!(name, Some("Billy Joel"));
//! 
//! // get the age field
//! let age = user_buffer.get::<u16>(&["age"])?;
//! // returns default value from schema
//! assert_eq!(age, Some(0u16));
//! 
//! // close again
//! let user_bytes: Vec<u8> = user_buffer.finish().bytes();
//! 
//! 
//! // we can now save user_bytes to disk, 
//! // send it over the network, or whatever else is needed with the data
//! 
//! 
//! # Ok::<(), NP_Error>(()) 
//! ```
//! 
//! ## Guided Learning / Next Steps:
//! 1. [`Schemas`](https://docs.rs/no_proto/latest/no_proto/schema/index.html) - Learn how to build & work with schemas.
//! 2. [`Factories`](https://docs.rs/no_proto/latest/no_proto/struct.NP_Factory.html) - Parsing schemas into something you can work with.
//! 3. [`Buffers`](https://docs.rs/no_proto/latest/no_proto/buffer/struct.NP_Buffer.html) - How to create, update & compact buffers/data.
//! 4. [`RPC Framework`](https://docs.rs/no_proto/latest/no_proto/rpc/index.html) - How to use the RPC Framework APIs.
//! 5. [`Data & Schema Format`](https://docs.rs/no_proto/latest/no_proto/format/index.html) - Learn how data is saved into the buffer and schemas.
//! 
//! ## Benchmarks
//! While it's difficult to properly benchmark libraries like these in a fair way, I've made an attempt in the graph below.  These benchmarks are available in the `bench` folder and you can easily run them yourself with `cargo run --release`. 
//! 
//! The format and data used in the benchmarks were taken from the `flatbuffers` benchmarks github repo.  You should always benchmark/test your own use case for each library before making any choices on what to use.
//! 
//! **Legend**: Ops / Millisecond, higher is better
//! 
//! | Format / Lib                                               | Encode  | Decode All | Decode 1 | Update 1 | Size (bytes) | Size (Zlib) |
//! |------------------------------------------------------------|---------|------------|----------|----------|--------------|-------------|
//! | **Runtime Libs**                                           |         |            |          |          |              |             |
//! | *NoProto*                                                  |         |            |          |          |              |             |
//! |        [no_proto](https://crates.io/crates/no_proto)       |    1393 |       1883 |    55556 |     9524 |          308 |         198 |
//! | Apache Avro                                                |         |            |          |          |              |             |
//! |         [avro-rs](https://crates.io/crates/avro-rs)        |     156 |         57 |       56 |       40 |          702 |         337 |
//! | FlexBuffers                                                |         |            |          |          |              |             |
//! |     [flexbuffers](https://crates.io/crates/flexbuffers)    |     444 |        962 |    24390 |      294 |          490 |         309 |
//! | JSON                                                       |         |            |          |          |              |             |
//! |            [json](https://crates.io/crates/json)           |     609 |        481 |      607 |      439 |          439 |         184 |
//! |      [serde_json](https://crates.io/crates/serde_json)     |     938 |        646 |      644 |      403 |          446 |         198 |
//! | BSON                                                       |         |            |          |          |              |             |
//! |            [bson](https://crates.io/crates/bson)           |     129 |        116 |      123 |       90 |          414 |         216 |
//! |         [rawbson](https://crates.io/crates/rawbson)        |     130 |       1117 |    17857 |       89 |          414 |         216 |
//! | MessagePack                                                |         |            |          |          |              |             |
//! |             [rmp](https://crates.io/crates/rmp)            |     661 |        623 |      832 |      202 |          311 |         193 |
//! |  [messagepack-rs](https://crates.io/crates/messagepack-rs) |     152 |        266 |      284 |      138 |          296 |         187 |
//! | **Compiled Libs**                                          |         |            |          |          |              |             |
//! | Flatbuffers                                                |         |            |          |          |              |             |
//! |     [flatbuffers](https://crates.io/crates/flatbuffers)    |    3165 |      16393 |   250000 |     2532 |          264 |         181 |
//! | Bincode                                                    |         |            |          |          |              |             |
//! |         [bincode](https://crates.io/crates/bincode)        |    6757 |       9259 |    10000 |     4115 |          163 |         129 |
//! | Postcard                                                   |         |            |          |          |              |             |
//! |        [postcard](https://crates.io/crates/postcard)       |    3067 |       7519 |     7937 |     2469 |          128 |         119 |
//! | Protocol Buffers                                           |         |            |          |          |              |             |
//! |        [protobuf](https://crates.io/crates/protobuf)       |     953 |       1305 |     1312 |      529 |          154 |         141 |
//! |           [prost](https://crates.io/crates/prost)          |    1464 |       2020 |     2232 |     1040 |          154 |         142 |
//! | Abomonation                                                |         |            |          |          |              |             |
//! |     [abomonation](https://crates.io/crates/abomonation)    |    2342 |     125000 |   500000 |     2183 |          261 |         160 |
//! | Rkyv                                                       |         |            |          |          |              |             |
//! |            [rkyv](https://crates.io/crates/rkyv)           |    1605 |      37037 |   200000 |     1531 |          180 |         154 |
//! 
//! - **Encode**: Transfer a collection of fields of test data into a serialized `Vec<u8>`.
//! - **Decode All**: Deserialize the test object from the `Vec<u8>` into all fields.
//! - **Decode 1**: Deserialize the test object from the `Vec<u8>` into one field.
//! - **Update 1**: Deserialize, update a single field, then serialize back into `Vec<u8>`.
//! 
//! **Runtime VS Compiled Libs**: Some formats require data types to be compiled into the application, which increases performance but means data types *cannot change at runtime*.  If data types need to mutate during runtime or can't be known before the application is compiled (like with databases), you must use a format that doesn't compile data types into the application, like JSON or NoProto.
//! 
//! Complete benchmark source code is available [here](https://github.com/only-cliches/NoProto/tree/master/bench).  Suggestions for improving the quality of these benchmarks is appreciated.
//! 
//! ## NoProto Strengths
//! If your use case fits any of the points below, NoProto might be a good choice for your application.
//! 
//! 1. Flexible At Runtime<br/>
//! If you need to work with data types that will change or be created at runtime, you normally have to pick something like JSON since highly optimized formats like Flatbuffers and Bincode depend on compiling the data types into your application (making everything fixed at runtime). When it comes to formats that can change/implement data types at runtime, NoProto is fastest format we're aware of (if you know if one that might be faster, let us know!).
//! 
//! 2. Safely Accept Untrusted Data</br>
//! The worse case failure mode for NoProto buffers is junk data.  While other formats can cause denial of service attacks or allow unsafe memory access, there is no such failure case with NoProto.  There is no way to construct a NoProto buffer that would cause any detrement in performance to the host application or lead to unsafe memory access.  Also, there is no panic causing code in the library, meaning it will never crash your application.
//! 
//! 3. Extremely Fast Updates<br/>
//! If you have a workflow in your application that is read -> modify -> write with buffers, NoProto will usually outperform every other format, including Bincode and Flatbuffers. This is because NoProto never actually deserializes, it doesn't need to.  This includes complicated mutations like pushing a value onto a nested list or replacing entire structs.
//! 
//! 4. All Fields Optional, Insert/Update In Any Order<br/>
//! Many formats require that all values be present to close the buffer, further they may require data to be inserted in a specific order to accomodate the encoding/decoding scheme.  With NoProto, all fields are optional and any update/insert can happen in any order.  
//! 
//! 5. Incremental Deserializing<br/>
//! You only pay for the fields you read, no more. There is no deserializing step in NoProto, opening a buffer performs no operations. Once you start asking for fields, the library will navigate the buffer using the format rules to get just what you asked for and nothing else. If you have a workflow in your application where you read a buffer and only grab a few fields inside it, NoProto will outperform most other libraries.
//! 
//! 6. Bytewise Sorting<br/>
//! Almost all of NoProto's data types are designed to serialize into bytewise sortable values, *including signed integers*.  When used with Tuples, making database keys with compound sorting is extremly easy.  When you combine that with first class support for `UUID`s and `ULID`s NoProto makes an excellent tool for parsing and creating primary keys for databases like RocksDB, LevelDB and TiKV. 
//! 
//! 7. `no_std` Support<br/>
//! If you need a serialization format with low memory usage that works in `no_std` environments, NoProto is one of the few good choices.
//! 
//! 8. Stable<br/>
//! NoProto will never cause a panic in your application.  It has *zero* panics or unwraps, meaning there is no code path that could lead to a panic.  Fallback behavior is to provide a sane default path or bubble an error up to the caller.
//! 
//! 9. CPU Independent<br/>
//! All numbers and pointers in NoProto buffers are always stored in big endian, so you can safely create buffers on any CPU architecture and know that they will work with any other CPU architecture.
//! 
//! 
//! ### When to use Flatbuffers / Bincode / CapN Proto
//! If you can safely compile all your data types into your application, all the buffers/data is trusted, and you don't intend to mutate buffers after they're created, Bincode/Flatbuffers/CapNProto is a better choice for you.
//! 
//! ### When to use JSON / BSON / MessagePack
//! If your data changes so often that schemas don't really make sense or the format you use must be self describing, JSON/BSON/MessagePack is a better choice.   Although I'd argue that if you *can* make schemas work you should.  Once you can use a format with schemas you save a ton of space in the resulting buffers and performance far better.
//! 
//! ## Limitations
//! - Structs and Tuples cannot have more than 255 items.
//! - Lists and Maps cannot have more than 2^16 (~64k) items.
//! - You cannot nest more than 255 levels deep.
//! - Struct field names cannot be longer than 255 UTF8 bytes.
//! - Enum/Option types are limited to 255 options and each option cannot be more than 255 UTF8 Bytes.
//! - Map keys cannot be larger than 255 UTF8 bytes.
//! - Buffers cannot be larger than 2^32 bytes or ~4GB.
//! 
//! ## Unsafe
//! This library makes use of `unsafe` to get better performance.  Generally speaking, it's not possible to have a high performance serialization library without `unsafe`.  It is only used where performance improvements are significant and additional checks are performed so that the worst case for any `unsafe` block is it leads to junk data in a buffer.
//! 
//! ----------------------
//! 
//! MIT License
//! 
//! Copyright (c) 2021 Scott Lott
//! 
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//! 
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//! 
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE. 

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod idl;
pub mod pointer;
pub mod collection;
pub mod buffer;
pub mod schema;
pub mod error;
pub mod json_flex;
pub mod format;
pub mod memory;
#[cfg(feature = "np_rpc")]
pub mod rpc;
#[cfg(feature = "np_rpc")]
#[allow(missing_docs)]
#[doc(hidden)]
pub mod hashmap;
mod utils;

#[macro_use]
extern crate alloc;

use core::ops::{Deref, DerefMut};
// use crate::buffer_ro::NP_Buffer_RO;
use crate::memory::NP_Memory;
use crate::json_flex::NP_JSON;
use crate::schema::NP_Schema;
use crate::json_flex::json_decode;
use crate::error::NP_Error;
use buffer::{NP_Buffer, DEFAULT_ROOT_PTR_ADDR};
use alloc::vec::Vec;
use alloc::string::String;
use idl::JS_Schema;
use schema::NP_Parsed_Schema;

/// Generate a path from a string.  The path must use dot notation between the path segments.
/// 
/// This requires allocation and will impact performance.
/// 
/// ```
/// use no_proto::error::NP_Error;
/// use no_proto::NP_Factory;
/// use no_proto::np_path;
/// 
/// 
/// assert_eq!(&np_path!("some.crazy.path"), &["some", "crazy", "path"]);
/// 
/// let user_factory = NP_Factory::new(r#"
///     struct({fields: {
///         name: string(),
///         todos: list({ of: string() })
///     }})
/// "#)?;
/// 
/// let mut user_buffer = user_factory.new_buffer(None);
/// user_buffer.set(&np_path!("todos.2"), "some todo")?;
/// user_buffer.set(&np_path!("name"), "Bob Dylan")?;
/// 
/// assert_eq!(Some("some todo"), user_buffer.get::<&str>(&["todos", "2"])?);
/// assert_eq!(Some("Bob Dylan"), user_buffer.get::<&str>(&["name"])?);
/// 
/// # Ok::<(), NP_Error>(()) 
/// ```
/// 
#[macro_export]
macro_rules! np_path {
    ($str1: tt) => {
        {
            let path: Vec<&str> = $str1.split(".").filter_map(|s| {
                if s.len() > 0 { Some(s) } else { None }
            }).collect();
            path
        }
    }
}


/// Factories are created from schemas.  Once you have a factory you can use it to create new buffers or open existing ones.
/// 
/// The easiest way to create a factory is to pass a JSON string schema into the static `new` method.  [Learn about schemas here.](./schema/index.html)
/// 
/// You can also create a factory with a compiled byte schema using the static `new_bytes` method.
/// 
/// # Example
/// ```
/// use no_proto::error::NP_Error;
/// use no_proto::NP_Factory;
/// 
/// let user_factory = NP_Factory::new(r#"
///     struct({fields: {
///         name:  string(),
///         pass:  string(),
///         age:   u16(),
///         todos: list({of: string()})
///     }})
/// "#)?;
/// 
/// 
/// // user_factory can now be used to make or open buffers that contain the data in the schema.
/// 
/// // create new buffer
/// let mut user_buffer = user_factory.new_buffer(None); // optional capacity, optional address size
///    
/// // set the "name" field of the struct
/// user_buffer.set(&["name"], "Billy Joel")?;
/// 
/// // set the first todo
/// user_buffer.set(&["todos", "0"], "Write a rust library.")?;
/// 
/// // close buffer 
/// let user_vec:Vec<u8> = user_buffer.finish().bytes();
/// 
/// // open existing buffer for reading
/// let user_buffer_2 = user_factory.open_buffer(user_vec);
/// 
/// // read field name
/// let name_field = user_buffer_2.get::<&str>(&["name"])?;
/// assert_eq!(name_field, Some("Billy Joel"));
/// 
/// 
/// // read first todo
/// let todo_value = user_buffer_2.get::<&str>(&["todos", "0"])?;
/// assert_eq!(todo_value, Some("Write a rust library."));
/// 
/// // read second todo
/// let todo_value = user_buffer_2.get::<&str>(&["todos", "1"])?;
/// assert_eq!(todo_value, None);
/// 
/// 
/// // close buffer again
/// let user_vec: Vec<u8> = user_buffer_2.finish().bytes();
/// // user_vec is a serialized Vec<u8> with our data
/// 
/// # Ok::<(), NP_Error>(()) 
/// ```
/// 
/// ## Next Step
/// 
/// Read about how to use buffers to access, mutate and compact data.
/// 
/// [Go to NP_Buffer docs](./buffer/struct.NP_Buffer.html)
/// 
#[derive(Debug)]
pub struct NP_Factory {
    /// schema data used by this factory
    pub schema: NP_Schema,
    schema_bytes: Vec<u8>
}

unsafe impl Send for NP_Factory {}
unsafe impl Sync for NP_Factory {}


/// When calling `maybe_compact` on a buffer, this struct is provided to help make a choice on wether to compact or not.
#[derive(Debug, Eq, PartialEq)]
pub struct NP_Size_Data {
    /// The size of the existing buffer
    pub current_buffer: usize,
    /// The estimated size of buffer after compaction
    pub after_compaction: usize,
    /// How many known wasted bytes in existing buffer
    pub wasted_bytes: usize
}

impl NP_Factory {

    /// Generate a new factory from an ES6 schema
    /// 
    /// The operation will fail if the string can't be parsed or the schema is otherwise invalid.
    /// 
    pub fn new<S>(es6_schema: S) -> Result<Self, NP_Error> where S: Into<String> {
        let idl = JS_Schema::new(es6_schema.into())?;

        let (is_sortable, schema_bytes, mut schema) = NP_Schema::from_idl(Vec::new(), &idl, &idl.ast)?;
        
        schema = NP_Schema::resolve_portals(schema)?;

        Ok(Self {
            schema_bytes: schema_bytes,
            schema:  NP_Schema {
                is_sortable: is_sortable,
                parsed: schema
            }
        }) 
    }
    
    /// Generate a new factory from the given JSON schema.
    /// 
    /// This operation will fail if the schema provided is invalid or if the schema is not valid JSON.  If it fails you should get a useful error message letting you know what the problem is.
    /// 
    pub fn new_json<S>(json_schema: S) -> Result<Self, NP_Error> where S: Into<String> {

        let parsed_value = json_decode(json_schema.into())?;

        let (is_sortable, schema_bytes, mut schema) = NP_Schema::from_json(Vec::new(), &parsed_value)?;

        schema = NP_Schema::resolve_portals(schema)?;

        Ok(Self {
            schema_bytes: schema_bytes,
            schema:  NP_Schema {
                is_sortable: is_sortable,
                parsed: schema
            }
        })      
        
    }

    /// Create a new factory from a compiled schema byte array.
    /// The byte schemas are at least an order of magnitude faster to parse than JSON schemas.
    /// 
    pub fn new_bytes(schema_bytes: &[u8]) -> Result<Self, NP_Error> {
        
        let (is_sortable, mut schema) = NP_Schema::from_bytes(Vec::new(), 0, schema_bytes);

        schema = NP_Schema::resolve_portals(schema)?;

        Ok(Self {
            schema_bytes: Vec::from(schema_bytes),
            schema:  NP_Schema { 
                is_sortable: is_sortable,
                parsed: schema
            }
        })
    }

    /// Get a copy of the compiled schema byte array
    /// 
    pub fn export_schema_bytes(&self) -> &[u8] {
        &self.schema_bytes[..]
    }

    /// Exports this factorie's schema to ES6 IDL.  This works regardless of wether the factory was created with `NP_Factory::new` or `NP_Factory::new_bytes`.
    /// 
    pub fn export_schema_idl(&self) -> Result<String, NP_Error> {
        self.schema.to_idl()
    }

    /// Exports this factorie's schema to JSON.  This works regardless of wether the factory was created with `NP_Factory::new` or `NP_Factory::new_bytes`.
    /// 
    pub fn export_schema_json(&self) -> Result<NP_JSON, NP_Error> {
        self.schema.to_json()
    }

    /// Open existing Vec<u8> as buffer for this factory.  
    /// 
    pub fn open_buffer(&self, bytes: Vec<u8>) -> NP_Buffer {
        NP_Buffer::_new(NP_Memory::existing_owned(bytes, &self.schema.parsed, DEFAULT_ROOT_PTR_ADDR))
    }

    /// Open existing buffer as ready only ref, can much faster if you don't need to mutate anything.
    /// 
    /// All operations that would lead to mutation fail.  You can't perform any mutations on a buffer opened with this method.
    /// 
    /// Also, read only buffers are `Sync` and `Send` so good for multithreaded environments.
    /// 
    pub fn open_buffer_ref<'buffer>(&'buffer self, bytes: &'buffer [u8]) -> NP_Buffer {
        NP_Buffer::_new(NP_Memory::existing_ref(bytes, &self.schema.parsed, DEFAULT_ROOT_PTR_ADDR))
    }

    /// Open existing buffer as mutable ref, can be much faster to skip copying.  The `data_len` property is how many bytes the data in the buffer is using up.
    /// 
    /// Some mutations cannot be done without appending bytes to the existing buffer.  Since it's impossible to append bytes to a `&mut [u8]` type, you should provide mutable slice with extra bytes on the end if you plan to mutate the buffer.
    /// 
    /// The `data_len` is at which byte the data ends in the buffer, this will be moved as needed by compaction and mutation operations.  
    /// 
    /// If the `&mut [u8]` type has the same length as `data_len`, mutations that require additional bytes will fail. `&mut [u8].len() - data_len` is how many bytes the buffer has for new allocations.
    /// 
    /// 
    pub fn open_buffer_ref_mut<'buffer>(&'buffer self, bytes: &'buffer mut [u8], data_len: usize) -> NP_Buffer {
        NP_Buffer::_new(NP_Memory::existing_ref_mut(bytes, data_len, &self.schema.parsed, DEFAULT_ROOT_PTR_ADDR))
    }

    /// Generate a new empty buffer from this factory.
    /// 
    /// The first opional argument, capacity, can be used to set the space of the underlying Vec<u8> when it's created.  If you know you're going to be putting lots of data into the buffer, it's a good idea to set this to a large number comparable to the amount of data you're putting in.  The default is 1,024 bytes.
    /// 
    /// 
    pub fn new_buffer<'buffer>(&'buffer self, capacity: Option<usize>) -> NP_Buffer {
        NP_Buffer::_new(NP_Memory::new(capacity, &self.schema.parsed, DEFAULT_ROOT_PTR_ADDR))
    }

    /// Generate a new empty buffer from this factory.
    /// 
    /// Make sure the mutable slice is large enough to fit all the data you plan on putting into it.
    /// 
    pub fn new_buffer_ref_mut<'buffer>(&'buffer self, bytes: &'buffer mut [u8]) -> NP_Buffer {
        NP_Buffer::_new(NP_Memory::new_ref_mut(bytes, &self.schema.parsed, DEFAULT_ROOT_PTR_ADDR))
    }

    /// Convert a regular buffer into a packed buffer. A "packed" buffer contains the schema and the buffer data together.
    /// 
    /// You can optionally store buffers with their schema attached so you don't have to track the schema seperatly.
    /// 
    /// The schema is stored in a very compact, binary format.  A JSON version of the schema can be generated from the binary version at any time.
    /// 
    pub fn pack_buffer(&self, buffer: NP_Buffer) -> NP_Packed_Buffer {
        NP_Packed_Buffer {
            buffer: NP_Buffer::_new(NP_Memory::existing_owned(buffer.finish().bytes(), &self.schema.parsed as *const Vec<NP_Parsed_Schema>, DEFAULT_ROOT_PTR_ADDR)),
            schema_bytes: self.export_schema_bytes().to_vec(),
            schema: self.schema.clone()
        }
    }
}

/// Packed Buffer Container
pub struct NP_Packed_Buffer {
    buffer: NP_Buffer,
    schema_bytes: Vec<u8>,
    /// Schema data for this packed buffer
    pub schema: NP_Schema
}

impl NP_Packed_Buffer {

    /// Open a packed buffer
    pub fn open(buffer: Vec<u8>) -> Result<Self, NP_Error> {
        if buffer[0] != 1 {
            return Err(NP_Error::new("Trying to use NP_Packed_Buffer::open on non packed buffer!"))
        }

        let schema_len = u16::from_be_bytes(unsafe { *((&buffer[1..3]) as *const [u8] as *const [u8; 2]) }) as usize;

        let schema_bytes = &buffer[3..(3 + schema_len)];

        let (is_sortable, mut schema) = NP_Schema::from_bytes(Vec::new(), 0, schema_bytes);

        schema = NP_Schema::resolve_portals(schema)?;

        let buffer_bytes = &buffer[(3 + schema_len)..];

        Ok(Self {
            buffer: NP_Buffer::_new(NP_Memory::existing_owned(buffer_bytes.to_vec(), &schema as *const Vec<NP_Parsed_Schema>, DEFAULT_ROOT_PTR_ADDR)),
            schema_bytes: schema_bytes.to_vec(),
            schema: NP_Schema {
                is_sortable: is_sortable,
                parsed: schema
            }
        })
    }

    /// Close this buffer and pack it
    pub fn close_packed(self) -> Vec<u8> {
        let mut new_buffer: Vec<u8> = Vec::new();
        new_buffer.push(1); // indicate this is a packed buffer
        let schema = self.export_schema_bytes();
        // schema size
        new_buffer.extend_from_slice(&(schema.len() as u16).to_be_bytes());
        // schema data
        new_buffer.extend_from_slice(self.export_schema_bytes());
        // buffer data
        new_buffer.extend(self.buffer.finish().bytes());
        new_buffer
    }

    /// Convert this packed buffer into a regular buffer
    pub fn into_buffer(self) -> NP_Buffer {
        self.buffer
    }

    /// Get the schema bytes for this packed buffer
    pub fn export_schema_bytes(&self) -> &[u8] {
        &self.schema_bytes[..]
    }

    /// Exports this schema to ES6 IDL.  This works regardless of how the initial buffer schema was created.
    /// 
    pub fn export_schema_idl(&self) -> Result<String, NP_Error> {
        self.schema.to_idl()
    }

    /// Exports this schema to JSON.  This works regardless of how the initial buffer schema was created.
    /// 
    pub fn export_schema_json(&self) -> Result<NP_JSON, NP_Error> {
        self.schema.to_json()
    }
}

impl Deref for NP_Packed_Buffer {
    type Target = NP_Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for NP_Packed_Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

#[test]
fn threading_works() {
    let fact = NP_Factory::new("string()").unwrap();
    let buffer = fact.new_buffer(None);
    std::thread::spawn(move || {
        let f = fact.export_schema_bytes();
        let b = buffer;
        assert_eq!(6, b.calc_bytes().unwrap().current_buffer);
        assert_eq!(8, f.len());
    }).join().unwrap()
}