use no_proto::{error::NP_Error, NP_Factory};

fn main() -> Result<(), NP_Error> {

    // JSON is used to describe schema for the factory
    // Each factory represents a single schema
    // One factory can be used to serialize/deserialize any number of buffers
    let user_factory = NP_Factory::new(r#"{
        "type": "table",
        "columns": [
            ["name",   {"type": "string"}],
            ["age",    {"type": "u16", "default": 0}],
            ["tags",   {"type": "list", "of": {
                "type": "string"
            }}]
        ]
    }"#)?;

    println!("\n= Quick Example =\n");

    // create a new empty buffer
    let user_buffer = user_factory
        // optional capacity, optional address size (u16 by default)
        .empty_buffer(None);


    // close buffer and get internal bytes
    let user_bytes: Vec<u8> = user_buffer.close();
    // show bytes (empty)
    println!("bytes: {:?}", user_bytes);
    // open the buffer again
    let mut user_buffer = user_factory.open_buffer(user_bytes);

    // set an internal value of the buffer, set the  "name" column
    user_buffer.set(&["name"], "Billy Joel")?;

    // get an internal value of the buffer from the "name" column
    let name = user_buffer.get::<&str>(&["name"])?;
    assert_eq!(name, Some("Billy Joel"));
    println!("\nname: {}", name.unwrap());
    

    // show bytes
    let user_bytes: Vec<u8> = user_buffer.close();
    println!("bytes: {:?}", user_bytes);
    let mut user_buffer = user_factory.open_buffer(user_bytes);

    user_buffer.compact(None)?;


    // assign nested internal values, sets the first tag element
    user_buffer.set(&["tags", "0"], "first tag")?;

    // get nested internal value, first tag from the tag list
    let tag = user_buffer.get::<&str>(&["tags", "0"])?;
    assert_eq!(tag, Some("first tag"));
    println!("\ntag: {}", tag.unwrap());


    // show bytes
    let user_bytes: Vec<u8> = user_buffer.close();
    println!("bytes: {:?}", user_bytes);
    let user_buffer = user_factory.open_buffer(user_bytes);


    // get nested internal value, the age field
    let age = user_buffer.get::<u16>(&["age"])?;
    // returns default value from schema
    assert_eq!(age, Some(0u16));
    println!("\nage: {}", age.unwrap());

    // close again
    let user_bytes: Vec<u8> = user_buffer.close();

    // we can now save user_bytes to disk,
    // send it over the network, or whatever else is needed with the data

    println!("bytes: {:?}", user_bytes);

    Ok(())
}
