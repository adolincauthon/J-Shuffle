use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io;

use crate::default_values::Values;

//add error handling?
/// Dumps a single JSON Struct to a file.
///
/// # Examples
///
/// ```
/// use serde::Serialize;
/// use serde_json::json;
/// use std::io;
/// use pollinate::json_utils::*;
///
/// #[derive(Serialize)]
/// struct MyStruct {
///     field1: i32,
///     field2: String,
/// }
///
/// let my_object = MyStruct {
///     field1: 42,
///     field2: String::from("Hello, World!"),
/// };
///
/// let result = dump_json(&my_object, "output.json");
/// assert!(result.is_ok());
/// ```
pub fn dump_json(object: &impl Serialize, file: &str) -> io::Result<()> {
    let json_string = serde_json::to_string(&object).unwrap();
    fs::write(file, json_string)
}

/// Dumps a vector of JSON values to a file.
///
/// # Examples
///
/// ```
/// use serde_json::Value;
/// use std::io;
/// use pollinate::json_utils::*;
///
/// let json_objects = vec![
///     serde_json::json!({"key1": "value1"}),
///     serde_json::json!({"key2": "value2"}),
/// ];
///
/// let result = dump_json_array(&json_objects, "output.json");
/// assert!(result.is_ok());
/// ```
pub fn dump_json_array(objects: &Vec<Value>, file: &str) -> io::Result<()> {
    let json_string = serde_json::to_string(&objects).unwrap();
    fs::write(file, json_string)
}

/// Dumps a single JSON value to a file.
///
/// # Examples
///
/// ```
/// use serde_json::Value;
/// use std::io;
/// use pollinate::json_utils::*;
///
/// let json_value = serde_json::json!({"key": "value"});
///
/// let result = dump_value(json_value, "output.json");
/// assert!(result.is_ok());
/// ```
pub fn dump_value(value: Value, file: &str) -> io::Result<()> {
    let json_string = serde_json::to_string(&value).unwrap();
    fs::write(file, json_string)
}

/// Takes a schema represented as a hashmap and generates a JSON object.
///
/// # Examples
///
/// ```
/// use serde_json::json;
/// use std::collections::HashMap;
/// use pollinate::default_values::Values;
/// use pollinate::json_utils::*;
///
/// #[derive(Debug, Clone)]
/// struct MyValue;
/// impl Values for MyValue {
///     fn get_value(&self) -> serde_json::Value {
///         serde_json::json!("some_value")
///     }
/// }
///
/// let mut schema = HashMap::new();
/// schema.insert("key".to_string(), Box::new(MyValue) as Box<dyn Values>);
///
/// let result =  create_json_from_schema(&schema);
/// assert_eq!(result, json!({"key": "some_value"}));
/// ```
pub fn create_json_from_schema<'a>(schema: &HashMap<String, Box<dyn Values + 'a>>) -> Value {
    let mut val = json!({});
    for key in schema.keys() {
        val[key] = schema.get(key).unwrap().get_value();
    }
    val
}
/// Takes a schema represented as a hashmap and generates a vector of JSON objects.
///
/// # Examples
///
/// ```
/// use serde_json::json;
/// use std::collections::HashMap;
/// use pollinate::default_values::Values;
/// use pollinate::json_utils::*;
///
/// #[derive(Debug, Clone)]
/// struct MyValue;
///
/// impl Values for MyValue {
///     fn get_value(&self) -> serde_json::Value {
///         serde_json::json!("some_value")
///     }
/// }
///
/// let mut schema = HashMap::new();
/// schema.insert("key".to_string(), Box::new(MyValue) as Box<dyn Values>);
///
/// let result = create_json_vec_from_schema(&schema, 3);
/// assert_eq!(result, vec![json!({"key": "some_value"}); 3]);
/// ```
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
