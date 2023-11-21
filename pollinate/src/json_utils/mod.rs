use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io;

use crate::default_values::Values;
//add error handling?
/// Dumps a single JSON Struct to a file
pub fn dump_json(object: &impl Serialize, file: &str) -> io::Result<()> {
    let json_string = serde_json::to_string(&object).unwrap();
    fs::write(file, json_string)
}

pub fn dump_json_array(objects: &Vec<Value>, file: &str) -> io::Result<()> {
    let json_string = serde_json::to_string(&objects).unwrap();
    fs::write(file, json_string)
}

pub fn dump_value(value: Value, file: &str) -> io::Result<()> {
    let json_string = serde_json::to_string(&value).unwrap();
    fs::write(file, json_string)
}

///Takes a schema and generates a JSON object
pub fn create_json_from_schema<'a>(schema: &HashMap<String, Box<dyn Values + 'a>>) -> Value {
    let mut val = json!({});
    for key in schema.keys() {
        val[key] = schema.get(key).unwrap().get_value();
    }
    val
}

///Takes a schema and generates a vec of JSON objects
pub fn create_json_vec_from_schema<'a>(
    schema: &HashMap<String, Box<dyn Values + 'a>>,
    number_of_objects: u32,
) -> Vec<Value> {
    let capacity = usize::try_from(number_of_objects).unwrap();
    let mut json_objects = Vec::<Value>::with_capacity(capacity);
    for _ in 0..capacity {
        json_objects.push(create_json_from_schema(schema))
    }
    json_objects
}

//pub fn generate_json(object: &impl Serialize) -> String {}
