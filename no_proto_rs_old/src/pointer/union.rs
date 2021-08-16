//! Clone type for recursive or duplicating data types.
//! 

use crate::{idl::{JS_AST, JS_Schema}, schema::{NP_Schema_Addr, NP_Value_Kind}};
use crate::NP_Schema;
use crate::{memory::NP_Memory, schema::{NP_Parsed_Schema}};
use alloc::vec::Vec;

use crate::json_flex::{JSMAP, NP_JSON};
use crate::schema::{NP_TypeKeys};
use crate::{pointer::NP_Value, error::NP_Error};


use alloc::string::String;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::borrow::ToOwned;

use super::{NP_Cursor, NP_Scalar};

/// Defines the behavior of the union data type
#[derive(Debug)]
pub struct NP_Union {
    /// The value of this union
    pub value: Option<String>,
    /// If the set value is a default
    pub is_default: bool
}


impl<'value> NP_Scalar<'value> for NP_Union {
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


impl NP_Union {

    /// Select into a union type
    pub fn select(mut cursor: NP_Cursor, types: &Vec<(u8, String, usize)>,  key: &str, make_path: bool, schema_query: bool, memory: &NP_Memory) -> Result<Option<NP_Cursor>, NP_Error> {
        match types.iter().position(|val| { val.1 == key }) {
            Some(x) => {

                if schema_query {
                    let schema_value = &types[x];
                    cursor.parent_schema_addr = cursor.schema_addr;
                    cursor.schema_addr = schema_value.2;
                    return Ok(Some(cursor))
                }

                let union_value = cursor.get_value(memory);

                let addr_value = union_value.get_addr_value();

                if addr_value == 0 { // no value here
                    if make_path { // need to make a new value
                        // 1 byte for union value, 2 bytes for pointer
                        let new_addr = memory.malloc_borrow(&[0u8; 3])?;
                        union_value.set_addr_value(new_addr as u32);
                    } else { // found nothing
                        return Ok(None)
                    }
                }

                let addr_value = union_value.get_addr_value() as usize;

                let union_index = memory.read_bytes()[addr_value];

                // nothing set at union and can't make value, so return None
                if union_index == 0 && make_path == false {
                    return Ok(None)
                }

                if union_index == 0 { // no value at this union yet
                    // set index of union value
                    memory.write_bytes()[addr_value] = (x as u8) + 1;
                    // return cursor 
                    return Ok(Some(NP_Cursor::new(addr_value + 1, types[x].2, cursor.schema_addr)))
                } else if union_index == (x as u8) + 1 { // union value matches query
                    // return cursor
                    return Ok(Some(NP_Cursor::new(addr_value + 1, types[x].2, cursor.schema_addr)))
                } else { // value is set by requested index does not match value in union
                    return Ok(None);
                }
            },
            None => return Ok(None)
        }
    }
}


impl<'value> NP_Value<'value> for NP_Union {
    fn type_idx() -> (&'value str, NP_TypeKeys) {
        ("union", NP_TypeKeys::Union)
    }

    fn self_type_idx(&self) -> (&'value str, NP_TypeKeys) {
        ("union", NP_TypeKeys::Union)
    }

    fn schema_to_json(schema: &Vec<NP_Parsed_Schema>, address: usize)-> Result<NP_JSON, NP_Error> {
        // let mut schema_json = JSMAP::new();
        // schema_json.insert("type".to_owned(), NP_JSON::String(Self::type_idx().0.to_string()));

        // let types: Vec<NP_JSON> = match &schema[address] {
        //     NP_TypeKeys::Union { types, .. } => {
        //         types.into_iter().map(|column| {
        //             let mut cols: Vec<NP_JSON> = Vec::new();
        //             cols.push(NP_JSON::String(column.1.to_string()));
        //             cols.push(NP_Schema::_type_to_json(&schema, column.2).unwrap_or(NP_JSON::Null));
        //             NP_JSON::Array(cols)
        //         }).collect()
        //     },
        //     _ => Vec::new()
        // };

        // schema_json.insert("types".to_owned(), NP_JSON::Array(types));

        // Ok(NP_JSON::Dictionary(schema_json))
        todo!()
    }

    fn schema_to_idl(_schema: &Vec<NP_Parsed_Schema>, _address: usize)-> Result<String, NP_Error> {
        todo!()
    }

    fn from_idl_to_schema(_schema: Vec<NP_Parsed_Schema>, _name: &str, _idl: &JS_Schema, _args: &Vec<JS_AST>) -> Result<(bool, Vec<u8>, Vec<NP_Parsed_Schema>), NP_Error> {
        todo!()
    }

    fn from_json_to_schema(mut schema: Vec<NP_Parsed_Schema>, json_schema: &Box<NP_JSON>) -> Result<(bool, Vec<u8>, Vec<NP_Parsed_Schema>), NP_Error> {

        // let mut schema_bytes: Vec<u8> = Vec::new();
        // schema_bytes.push(NP_TypeKeys::Union as u8);

        // let schema_table_addr = schema.len();
        // schema.push(NP_TypeKeys::Union {
        //     val: NP_Value_Kind::Pointer,
        //     i: NP_TypeKeys::Union,
        //     sortable: false,
        //     types: Vec::new(),
        //     default: 0
        // });

        // let mut columns_mapped = Vec::new();

        // let mut types: Vec<(u8, String, NP_Schema_Addr)> = Vec::new();

        // let mut column_data: Vec<(String, Vec<u8>)> = Vec::new();

        // let mut schema_parsed: Vec<NP_Parsed_Schema> = schema;

        // match &json_schema["types"] {
        //     NP_JSON::Array(cols) => {
        //         let mut x: u8 = 0;
        //         for col in cols {
        //             let column_name = match &col[0] {
        //                 NP_JSON::String(x) => x.clone(),
        //                 _ => "".to_owned()
        //             };
        //             if column_name.len() > 255 {
        //                 return Err(NP_Error::new("Union type names cannot be longer than 255 characters!"))
        //             }

        //             let column_schema_addr = schema_parsed.len();
        //             types.push((x, column_name.clone(), column_schema_addr));
        //             let (_is_sortable, column_type, schema_p) = NP_Schema::from_json(schema_parsed, &Box::new(col[1].clone()))?;
        //             schema_parsed = schema_p;
        //             columns_mapped.push(column_name.to_string());
        //             column_data.push((column_name, column_type));
        //             x += 1;
        //         }
        //     },
        //     _ => { 
        //         return Err(NP_Error::new("Unions require a 'types' property that is an array of schemas!"))
        //     }
        // }


        // schema_parsed[schema_table_addr] = NP_TypeKeys::Union {
        //     val: NP_Value_Kind::Pointer,
        //     i: NP_TypeKeys::Union,
        //     sortable: false,
        //     types: types,
        //     default: 0
        // };

        // if column_data.len() > 254 {
        //     return Err(NP_Error::new("Unions cannot have more than 254 types!"))
        // }

        // if column_data.len() == 0 {
        //     return Err(NP_Error::new("Unions must have at least one type!"))
        // }

        // // number of columns
        // schema_bytes.push(column_data.len() as u8);

        // for col in column_data {
        //     // colum name
        //     let bytes = col.0.as_bytes().to_vec();
        //     schema_bytes.push(bytes.len() as u8);
        //     schema_bytes.extend(bytes);

        //     if col.1.len() > u16::MAX as usize {
        //         return Err(NP_Error::new("Schema overflow error!"))
        //     }
            
        //     // column type
        //     schema_bytes.extend((col.1.len() as u16).to_be_bytes().to_vec());
        //     schema_bytes.extend(col.1);
        // }

        // return Ok((false, schema_bytes, schema_parsed))
        todo!()
   
    }

    fn from_bytes_to_schema(mut schema: Vec<NP_Parsed_Schema>, address: usize, bytes: &[u8]) -> (bool, Vec<NP_Parsed_Schema>) {
        // let column_len = bytes[address + 1];

        // let mut parsed_types: Vec<(u8, String,  NP_Schema_Addr)> = Vec::new();

        // let table_schema_addr = schema.len();

        // schema.push(NP_TypeKeys::Union {
        //     val: NP_Value_Kind::Pointer,
        //     i: NP_TypeKeys::Union,
        //     sortable: false,
        //     default: 0,
        //     types: Vec::new()
        // });

        // let mut schema_parsed = schema;

        // let mut offset = address + 2;

        // let mut hash_map = Vec::new();

        // for x in 0..column_len as usize {
        //     let col_name_len = bytes[offset] as usize;
        //     let col_name_bytes = &bytes[(offset + 1)..(offset + 1 + col_name_len)];
        //     let col_name = unsafe { core::str::from_utf8_unchecked(col_name_bytes) };

        //     offset += 1 + col_name_len;

        //     let schema_size = u16::from_be_bytes([
        //         bytes[offset],
        //         bytes[offset + 1]
        //     ]) as usize;

        //     let column_addr = schema_parsed.len();
        //     let (_, schema) = NP_Schema::from_bytes(schema_parsed, offset + 2, bytes);
        //     schema_parsed = schema;
        //     parsed_types.push((x as u8, col_name.to_string(), column_addr));
        //     hash_map.push(col_name.to_string());
        //     offset += schema_size + 2;
        // }

        // schema_parsed[table_schema_addr] = NP_TypeKeys::Union {
        //     val: NP_Value_Kind::Pointer,
        //     i: NP_TypeKeys::Union,
        //     sortable: false,
        //     types: parsed_types,
        //     default: 0
        // };

        // (false, schema_parsed)
        todo!()
    }

    fn set_from_json<'set>(_depth: usize, _apply_null: bool, _cursor: NP_Cursor, _memory: &'set NP_Memory, _value: &Box<NP_JSON>) -> Result<(), NP_Error> where Self: 'set + Sized {
        todo!()
    }

    fn default_value(_depth: usize, _schema_addr: usize, _schemas: &Vec<NP_Parsed_Schema>) -> Option<Self> {
        todo!()
    }

    /// Pull the data from the buffer and convert into type
    /// 
    fn into_value(_cursor: &NP_Cursor, _memory: &'value NP_Memory) -> Result<Option<Self>, NP_Error> where Self: Sized {
        // let message = "This type doesn't support into!".to_owned();
        // Err(NP_Error::new(message.as_str()))
        todo!()
    }

    fn to_json(_depth:usize, _cursor: &NP_Cursor, _memory: &'value NP_Memory) -> NP_JSON {
        // match memory.get_schema(cursor.schema_addr) {
        //     NP_TypeKeys::Portal { schema, parent_schema, .. } => {
        //         let mut next = cursor.clone();
        //         next.schema_addr = *schema;
        //         next.parent_schema_addr = *parent_schema;
        //         NP_Cursor::json_encode(depth + 1, &next, memory)
        //     },
        //     _ => NP_JSON::Null
        // }
        todo!()
    }

    fn get_size(_depth:usize, _cursor: &'value NP_Cursor, _memory: &'value NP_Memory) -> Result<usize, NP_Error> {
        // match memory.get_schema(cursor.schema_addr) {
        //     NP_TypeKeys::Portal { schema, parent_schema, .. } => {
        //         let mut next = cursor.clone();
        //         next.schema_addr = *schema;
        //         next.parent_schema_addr = *parent_schema;
        //         NP_Cursor::calc_size(depth + 1, &next, memory)
        //     },
        //     _ => Err(NP_Error::Unreachable)
        // }
        todo!()
    }

    fn do_compact<, M2: NP_Memory>(_depth:usize, mut _from_cursor: NP_Cursor, _from_memory: &'value NP_Memory, mut _to_cursor: NP_Cursor, _to_memory: &'value NP_Memory2) -> Result<NP_Cursor, NP_Error> where Self: 'value + Sized {
        // match from_memory.get_schema(from_cursor.schema_addr) {
        //     NP_TypeKeys::Portal { schema, parent_schema, .. } => {
        //         from_cursor.schema_addr = *schema;
        //         from_cursor.parent_schema_addr = *parent_schema;
        //         to_cursor.schema_addr = *schema;
        //         to_cursor.parent_schema_addr = *parent_schema;
        //         NP_Cursor::compact(depth + 1, from_cursor, from_memory, to_cursor, to_memory)
        //     },
        //     _ => Err(NP_Error::Unreachable)
        // }
        todo!()
    }
}



#[test]
fn schema_parsing_works() -> Result<(), NP_Error> {

    let schema = r#"{"type":"union","types":[["value1",{"type":"string"}],["value2",{"type":"uint8"}]]}"#;
    let factory = crate::NP_Factory::new_json(schema)?;
    assert_eq!(schema, factory.schema.to_json()?.stringify());
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_json()?.stringify());

    Ok(())
} 

// #[test]
// fn set_clear_value_and_compaction_works() -> Result<(), NP_Error> {
//     let schema = r#"{
//         "type": "union",
//         "default": "uknown",
//         "types": [
//             ["uknown", {"type": "bool", "default": true}],
//             ["unemployed", {"type": "bool"}],
//             ["employed", {"type": "string"}],
//             ["school", {"type": "string"}],
//             ["selfemployed", {"type": "bool"}]
//         ]
//     }"#;
//     let factory = crate::NP_Factory::new_json(schema)?;
//     let mut buffer = factory.new_buffer(None);

//     buffer.set(&["nested", "street"], "hello street")?;
//     buffer.set(&["nested", "nested", "nested", "nested", "street"], "hello street 2")?;

//     assert_eq!("hello street", buffer.get::<&str>(&["nested", "street"])?.unwrap());
//     assert_eq!("hello street 2", buffer.get::<&str>(&["nested", "nested", "nested", "nested", "street"])?.unwrap());
//     assert_eq!(buffer.calc_bytes()?.current_buffer, buffer.calc_bytes()?.after_compaction);
//     buffer.del(&["nested", "street"])?;
//     buffer.compact(None)?;
//     assert_eq!("hello street 2", buffer.get::<&str>(&["nested", "nested", "nested", "nested", "street"])?.unwrap());
//     assert_eq!(None, buffer.get::<&str>(&["nested", "street"])?);


//     let schema = r#"{
//         "type": "struct",
//         "types": [
//             ["username", {"type": "string"}],
//             ["email"  , {"type": "string"}],
//             ["address", {"type": "struct", "types": [
//                 ["street", {"type": "string"}],
//                 ["city", {"type": "string"}],
//                 ["more", {"type": "portal", "to": "address"}]
//             ]}]
//         ]
//     }"#;
//     let factory = crate::NP_Factory::new_json(schema)?;
//     let mut buffer = factory.new_buffer(None);

//     buffer.set(&["address", "more", "more","more", "more","more", "more","more", "more", "street"], "hello")?;

//     assert_eq!("hello", buffer.get::<&str>(&["address", "more", "more","more", "more","more", "more","more", "more", "street"])?.unwrap());

//     Ok(())
// }