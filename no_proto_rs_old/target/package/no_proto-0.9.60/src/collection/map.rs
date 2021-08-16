use alloc::{string::String, sync::Arc};
use crate::{idl::{JS_AST, JS_Schema}, pointer::NP_Cursor, schema::{NP_Map_List_Data, NP_Value_Kind}};
use crate::{json_flex::JSMAP};
use crate::pointer::{NP_Value};
use crate::{memory::{NP_Memory}, schema::{NP_Schema, NP_TypeKeys, NP_Parsed_Schema}, error::NP_Error, json_flex::NP_JSON};

use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::borrow::ToOwned;

#[repr(C)]
#[derive(Debug)]
#[doc(hidden)]
#[allow(missing_docs)]
pub struct NP_Map_Bytes {
    head: [u8; 4]
}

#[allow(missing_docs)]
impl NP_Map_Bytes {
    #[inline(always)]
    pub fn set_head(&mut self, head: u32) {
        self.head = head.to_be_bytes();
    }
    #[inline(always)]
    pub fn get_head(&self) -> u32 {
        u32::from_be_bytes(self.head)
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
struct Map_Item<'item> {
    key: &'item str,
    buff_addr: usize
}

impl<'item> Map_Item<'item> {
    pub fn new(key: &'item str, buff_addr: usize) -> Self {
        Self { key, buff_addr}
    }
}

/// The map type.
/// 
#[doc(hidden)]
#[derive(Debug)]
pub struct NP_Map<'map> { 
    count: usize,
    current: Option<Map_Item<'map>>,
    head: Option<Map_Item<'map>>,
    map: NP_Cursor,
    value_of: usize
}

#[allow(missing_docs)]
impl<'map> NP_Map<'map> {

    #[inline(always)]
    pub fn select(map_cursor: NP_Cursor, key: &str, make_path: bool, schema_query: bool, memory: &'map NP_Memory) -> Result<Option<NP_Cursor>, NP_Error> {

        let data = unsafe { &*(*memory.get_schema(map_cursor.schema_addr).data as *const NP_Map_List_Data) };

        if schema_query {
            let value_of = data.child;

            return Ok(Some(NP_Cursor::new(0, value_of, map_cursor.schema_addr)));
        }

        let mut map_iter = Self::new_iter(&map_cursor, memory);

        // key is maybe in map
        while let Some((ikey, item)) = map_iter.step_iter(memory) {
            if ikey == key {
                return Ok(Some(item.clone()))
            }
        }

        // key is not in map
        if make_path {
            Ok(Some(Self::insert(&map_cursor, memory, key)?))
        } else {
            Ok(None)
        }
    }

    #[inline(always)]
    pub fn get_map<'get>(map_buff_addr: usize, memory: &'get NP_Memory) -> &'get mut NP_Map_Bytes {
        if map_buff_addr > memory.read_bytes().len() { // attack
            unsafe { &mut *(memory.write_bytes().as_ptr() as *mut NP_Map_Bytes) }
        } else { // normal operation
            unsafe { &mut *(memory.write_bytes().as_ptr().add(map_buff_addr as usize) as *mut NP_Map_Bytes) }
        }
    }

    #[inline(always)]
    pub fn new_iter(map_cursor: &NP_Cursor, memory: &'map NP_Memory) -> Self {

        let data = unsafe { &*(*memory.get_schema(map_cursor.schema_addr).data as *const NP_Map_List_Data) };

        let value_of = data.child;

        if map_cursor.get_value(memory).get_addr_value() == 0 {
            return Self {
                current: None,
                count: 0,
                head: None,
                map: map_cursor.clone(),
                value_of
            }
        }

        let head_addr = Self::get_map(map_cursor.buff_addr, memory).get_head();

        let head_cursor = NP_Cursor::new(head_addr as usize, value_of, map_cursor.schema_addr);
        let head_cursor_value = head_cursor.get_value(memory);

        Self {
            current: None,
            count: 0,
            head: Some(Map_Item::new(head_cursor_value.get_key(memory), head_cursor.buff_addr )),
            map: map_cursor.clone(),
            value_of
        }
    }

    #[inline(always)]
    pub fn step_iter(&mut self, memory: &'map NP_Memory) -> Option<(&'map str, NP_Cursor)> {

        if self.count > u16::MAX as usize {
            return None;
        }
        
        match self.head {
            Some(head) => {

                self.count += 1;

                match self.current {
                    Some(current) => { // subsequent iterations
                        let current_item = NP_Cursor::new(current.buff_addr, self.value_of, self.map.schema_addr);
                        let current_value = current_item.get_value(memory);
                        let next_value = current_value.get_next_addr() as usize;
                        if next_value == 0 { //nothing left to step
                            return None;
                        } else {
                            let next_value_cursor = NP_Cursor::new(next_value, self.value_of, self.map.schema_addr);
                            let next_value_value = next_value_cursor.get_value(memory);
                            let key = next_value_value.get_key(memory);
                            self.current = Some(Map_Item { buff_addr: next_value, key: key });
                            return Some((key, next_value_cursor))
                        }
                    },
                    None => { // first iteration, get head
                        self.current = Some(head.clone());
                        return Some((head.key, NP_Cursor::new(head.buff_addr, self.value_of, self.map.schema_addr)))
                    }
                }
            },
            None => return None
        }


    }

    #[inline(always)]
    pub fn insert(map_cursor: &NP_Cursor, memory: &NP_Memory, key: &str) -> Result<NP_Cursor, NP_Error> {

        let data = unsafe { &*(*memory.get_schema(map_cursor.schema_addr).data as *const NP_Map_List_Data) };

        let value_of = data.child;

        if key.len() >= 255 {
            return Err(NP_Error::new("Key length cannot be larger than 255 charecters!"));
        }

        let map_value = || { map_cursor.get_value(memory) };

        let new_cursor_addr = memory.malloc_borrow(&[0u8; 12])?;
        let new_cursor = NP_Cursor::new(new_cursor_addr, value_of, map_cursor.schema_addr);

        // set key
        let key_item_addr = memory.malloc_borrow(&[key.len() as u8])?;
        memory.malloc_borrow(key.as_bytes())?;
        new_cursor.get_value_mut(memory).set_key_addr(key_item_addr as u32);

        let head = map_value().get_addr_value() as usize;

        // Set head of map to new cursor
        map_cursor.get_value_mut(memory).set_addr_value(new_cursor_addr as u32);

        if head != 0 { // set new cursors NEXT to old HEAD
            new_cursor.get_value_mut(memory).set_next_addr(head as u32);
        }

        Ok(new_cursor)
    }

}

impl<'value> NP_Value<'value> for NP_Map<'value> {

    fn to_json(depth:usize, cursor: &NP_Cursor, memory: &'value NP_Memory) -> NP_JSON {
        let c_value = || { cursor.get_value(memory) };

        if c_value().get_addr_value() == 0 {
            return NP_JSON::Null
        }

        let mut json_map = JSMAP::new();

        let mut map_iter = NP_Map::new_iter(&cursor, memory);

        while let Some((key, item)) = NP_Map::step_iter(&mut map_iter, memory) {
            json_map.insert(String::from(key), NP_Cursor::json_encode(depth + 1, &item, memory));     
        }

        NP_JSON::Dictionary(json_map)
    }

    fn set_from_json<'set>(depth: usize, apply_null: bool, cursor: NP_Cursor, memory: &'set NP_Memory, value: &Box<NP_JSON>) -> Result<(), NP_Error> where Self: 'set + Sized {
        
        match &**value {
            NP_JSON::Dictionary(json_map) => {
                for js_item in json_map.values.iter() {
                    match NP_Map::select(cursor, &js_item.0, true, false, memory)? {
                        Some(value) => {
                            NP_Cursor::set_from_json(depth + 1, apply_null, value, memory, &Box::new(js_item.1.clone()))?;
                        },
                        None => { }
                    }
                }
            },
            _ => { }
        }
    
        Ok(())
    }

    fn type_idx() -> (&'value str, NP_TypeKeys) { ("map", NP_TypeKeys::Map) }
    fn self_type_idx(&self) -> (&'value str, NP_TypeKeys) { ("map", NP_TypeKeys::Map) }
    
    fn schema_to_json(schema: &Vec<NP_Parsed_Schema>, address: usize)-> Result<NP_JSON, NP_Error> {
        let mut schema_json = JSMAP::new();
        schema_json.insert("type".to_owned(), NP_JSON::String(Self::type_idx().0.to_string()));

        let data = unsafe { &*(*schema[address].data as *const NP_Map_List_Data) };

        let value_of = data.child;

        schema_json.insert("value".to_owned(), NP_Schema::_type_to_json(schema, value_of)?);

        Ok(NP_JSON::Dictionary(schema_json))
    }

    fn get_size(depth:usize, cursor: &NP_Cursor, memory: &'value NP_Memory) -> Result<usize, NP_Error> {

        let c_value = || { cursor.get_value(memory) };

        if c_value().get_addr_value() == 0 {
            return Ok(0) 
        }

        let mut acc_size = 0usize;

        let mut map_iter = Self::new_iter(&cursor, memory);

        while let Some((_index, item)) = Self::step_iter(&mut map_iter, memory) {
            let key_size = item.get_value(memory).get_key_size(memory);
            acc_size += 1; // length byte
            acc_size += key_size;
            acc_size += NP_Cursor::calc_size(depth + 1, &item, memory)?;
        }


        Ok(acc_size)
   
    }



    fn do_compact(depth:usize, from_cursor: NP_Cursor, from_memory: &'value NP_Memory, to_cursor: NP_Cursor, to_memory: &'value NP_Memory) -> Result<NP_Cursor, NP_Error> where Self: 'value + Sized {

        let from_value = from_cursor.get_value(from_memory);

        if from_value.get_addr_value() == 0 {
            return Ok(to_cursor) 
        }

        let mut map_iter = Self::new_iter(&from_cursor, from_memory);

        while let Some((key, item)) = Self::step_iter(&mut map_iter, from_memory) {
            let new_item = Self::insert(&to_cursor, to_memory, key)?;
            NP_Cursor::compact(depth + 1, item.clone(), from_memory, new_item, to_memory)?;    
        }


        Ok(to_cursor)
    }

    fn schema_to_idl(schema: &Vec<NP_Parsed_Schema>, address: usize)-> Result<String, NP_Error> {
        let data = unsafe { &*(*schema[address].data as *const NP_Map_List_Data) };

        let mut result = String::from("map({value: ");
        result.push_str(NP_Schema::_type_to_idl(&schema, data.child)?.as_str());
        result.push_str("})");
        Ok(result)
         
    }

    fn from_idl_to_schema(mut schema: Vec<NP_Parsed_Schema>, _name: &str, idl: &JS_Schema, args: &Vec<JS_AST>) -> Result<(bool, Vec<u8>, Vec<NP_Parsed_Schema>), NP_Error> {
        let mut schema_data: Vec<u8> = Vec::new();
        schema_data.push(NP_TypeKeys::Map as u8);
        

        let value_addr = schema.len();
        schema.push(NP_Parsed_Schema {
            val: NP_Value_Kind::Pointer,
            i: NP_TypeKeys::Map,
            sortable: false,
            data: Arc::new(Box::into_raw(Box::new(NP_Map_List_Data { child: value_addr + 1 })) as *const u8)
        });

        let mut value_jst: Option<&JS_AST> = None;

        if args.len() > 0 {
            match &args[0] {
                JS_AST::object { properties } => {
                    for (key, value) in properties {
                        if idl.get_str(key).trim() == "value" {
                            value_jst = Some(value);
                        }
                    }
                },
                _ => { }
            }
        };

        if let Some(x) = value_jst {
            // let of_addr = schema.len();
            let (_sortable, child_bytes, schema) = NP_Schema::from_idl(schema, idl, x)?;
            
            schema_data.extend(child_bytes);

            Ok((false, schema_data, schema))
        } else {
            Err(NP_Error::new("lists require an 'of' property!"))
        }
    }

    fn from_json_to_schema(mut schema: Vec<NP_Parsed_Schema>, json_schema: &Box<NP_JSON>) -> Result<(bool, Vec<u8>, Vec<NP_Parsed_Schema>), NP_Error> {
      
        let mut schema_data: Vec<u8> = Vec::new();
        schema_data.push(NP_TypeKeys::Map as u8);

        let value_addr = schema.len();
        schema.push(NP_Parsed_Schema {
            val: NP_Value_Kind::Pointer,
            i: NP_TypeKeys::Map,
            data: Arc::new(Box::into_raw(Box::new(NP_Map_List_Data { child: value_addr + 1 })) as *const u8),
            sortable: false
        });

        match json_schema["value"] {
            NP_JSON::Null => {
                return Err(NP_Error::new("Maps require a 'value' property that is a schema type!"))
            },
            _ => { }
        }

        
        let (_sortable, child_bytes, schema) = NP_Schema::from_json(schema, &Box::new(json_schema["value"].clone()))?;
        
        schema_data.extend(child_bytes);

        return Ok((false, schema_data, schema))

    }

    fn default_value(_depth: usize, _addr: usize, _schema: &Vec<NP_Parsed_Schema>) -> Option<Self> {
        None
    }

    fn from_bytes_to_schema(mut schema: Vec<NP_Parsed_Schema>, address: usize, bytes: &[u8]) -> (bool, Vec<NP_Parsed_Schema>) {
        let of_addr = schema.len();
        schema.push(NP_Parsed_Schema {
            val: NP_Value_Kind::Pointer,
            i: NP_TypeKeys::Map,
            sortable: false,
            data: Arc::new(Box::into_raw(Box::new(NP_Map_List_Data { child: of_addr + 1 })) as *const u8)
        });
        let (_sortable, schema) = NP_Schema::from_bytes(schema, address + 1, bytes);
        (false, schema)
    }
}


#[test]
fn schema_parsing_works_idl() -> Result<(), NP_Error> {
    let schema = r#"map({value: string()})"#;
    let factory = crate::NP_Factory::new(schema)?;
    assert_eq!(schema, factory.schema.to_idl()?);
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_idl()?);
    Ok(())
}

#[test]
fn schema_parsing_works() -> Result<(), NP_Error> {
    let schema = r#"{"type":"map","value":{"type":"string"}}"#;
    let factory = crate::NP_Factory::new_json(schema)?;
    assert_eq!(schema, factory.schema.to_json()?.stringify());
    let factory2 = crate::NP_Factory::new_bytes(factory.export_schema_bytes())?;
    assert_eq!(schema, factory2.schema.to_json()?.stringify());
    Ok(())
}

#[test]
fn set_clear_value_and_compaction_works() -> Result<(), NP_Error> {
    let schema = r#"{"type":"map","value":{"type":"string"}}"#;
    let factory = crate::NP_Factory::new_json(schema)?;

    // compaction works
    let mut buffer = factory.new_buffer(None);
    buffer.set(&["name"], "hello, world")?;
    assert_eq!(buffer.get::<&str>(&["name"])?, Some("hello, world"));
    assert_eq!(buffer.calc_bytes()?.after_compaction, buffer.calc_bytes()?.current_buffer);
    assert_eq!(buffer.calc_bytes()?.current_buffer, 39usize);
    buffer.del(&[])?;
    buffer.compact(None)?;
    assert_eq!(buffer.calc_bytes()?.current_buffer, 6usize);

    // values are preserved through compaction
    let mut buffer = factory.new_buffer(None);
    buffer.set(&["name"], "hello, world")?;
    buffer.set(&["name2"], "hello, world2")?;
    assert_eq!(buffer.get::<&str>(&["name"])?, Some("hello, world"));
    assert_eq!(buffer.get::<&str>(&["name2"])?, Some("hello, world2"));
    assert_eq!(buffer.calc_bytes()?.current_buffer, 74usize);
    buffer.compact(None)?;
    assert_eq!(buffer.get::<&str>(&["name"])?, Some("hello, world"));
    assert_eq!(buffer.get::<&str>(&["name2"])?, Some("hello, world2"));
    assert_eq!(buffer.calc_bytes()?.current_buffer, 74usize);

    buffer.set_with_json(&[], r#"{"value": {"foo": "bar", "foo2": "bar2"}}"#)?;
    assert_eq!(buffer.get::<&str>(&["foo"])?, Some("bar"));
    assert_eq!(buffer.get::<&str>(&["foo2"])?, Some("bar2"));

    Ok(())
}