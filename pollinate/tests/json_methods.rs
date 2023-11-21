
use std::collections::HashMap;

use pollinate::{
    default_values::{DiscreteValues, ObjectValues, RangedValues, Values},
    json_utils::*,
};
use serde::Serialize;
use serde_json::json;

#[test]
fn write_to_file() {
    #[derive(Debug, Serialize)]
    struct Person {
        first_name: String,
        last_name: String,
        zip: u32,
    }
    let person = Person {
        first_name: "Adam".to_string(),
        last_name: "Hiatt".to_string(),
        zip: 97056,
    };
    match dump_json(&person, "./test/test.json") {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}

#[test]
fn return_json_object() {
    let possible_first_names = vec![json!("Adam"), json!("John"), json!("Ted")];
    let possible_last_names = vec![json!("Hiatt"), json!("Johnson"), json!("Tedson")];
    let possible_zips = vec![json!(12333), json!(97012), json!(21312)];
    let first_names = Box::new(DiscreteValues::new(&possible_first_names)) as Box<dyn Values>;
    let last_names = Box::new(DiscreteValues::new(&possible_last_names)) as Box<dyn Values>;
    let zips = Box::new(DiscreteValues::new(&possible_zips)) as Box<dyn Values>;
    let mut schema = HashMap::new();
    schema.insert("first_name".to_owned(), first_names);
    schema.insert("last_name".to_owned(), last_names);
    schema.insert("zip_code".to_owned(), zips);

    let val = create_json_from_schema(&schema);
    println!("{:?}", val);
}

#[test]
fn return_json_object_array() {
    let possible_first_names = vec![json!("Adam"), json!("John"), json!("Ted")];
    let possible_last_names = vec![json!("Hiatt"), json!("Johnson"), json!("Tedson")];
    let possible_zips = vec![json!(12333), json!(97012), json!(21312)];
    let first_names = Box::new(DiscreteValues::new(&possible_first_names)) as Box<dyn Values>;
    let last_names = Box::new(DiscreteValues::new(&possible_last_names)) as Box<dyn Values>;
    let zips = Box::new(DiscreteValues::new(&possible_zips)) as Box<dyn Values>;
    let mut schema = HashMap::new();
    schema.insert("first_name".to_owned(), first_names);
    schema.insert("last_name".to_owned(), last_names);
    schema.insert("zip_code".to_owned(), zips);

    let val = create_json_vec_from_schema(&schema, 100);
    println!("{:?}", val);
}

#[test]
fn dump_json_object_from_schema() {
    let possible_first_names = vec![json!("Adam"), json!("John"), json!("Ted")];
    let possible_last_names = vec![json!("Hiatt"), json!("Johnson"), json!("Tedson")];
    let possible_zips = vec![json!(12333), json!(97012), json!(21312)];
    let first_names = Box::new(DiscreteValues::new(&possible_first_names)) as Box<dyn Values>;
    let last_names = Box::new(DiscreteValues::new(&possible_last_names)) as Box<dyn Values>;
    let zips = Box::new(DiscreteValues::new(&possible_zips)) as Box<dyn Values>;
    let mut schema = HashMap::new();
    schema.insert("first_name".to_owned(), first_names);
    schema.insert("last_name".to_owned(), last_names);
    schema.insert("zip_code".to_owned(), zips);

    let val = create_json_from_schema(&schema);
    let _ = dump_json(&val, "temp.json");
    println!("{:?}", val);
}

#[test]
fn dump_json_vec_from_schema() {
    let possible_first_names = vec![json!("Adam"), json!("John"), json!("Ted")];
    let possible_last_names = vec![json!("Hiatt"), json!("Johnson"), json!("Tedson")];
    let possible_zips = vec![json!(12333), json!(97012), json!(21312)];
    let first_names = Box::new(DiscreteValues::new(&possible_first_names)) as Box<dyn Values>;
    let last_names = Box::new(DiscreteValues::new(&possible_last_names)) as Box<dyn Values>;
    let zips = Box::new(DiscreteValues::new(&possible_zips)) as Box<dyn Values>;
    let mut schema = HashMap::new();
    schema.insert("first_name".to_owned(), first_names);
    schema.insert("last_name".to_owned(), last_names);
    schema.insert("zip_code".to_owned(), zips);

    let val = create_json_vec_from_schema(&schema, 5000);
    let _ = dump_json_array(&val, "temp.json");
    println!("{:?}", val);
}

#[test]
fn test_nested_object() {
    let possible_first_names = vec![json!("Adam"), json!("John"), json!("Ted")];
    let possible_last_names = vec![json!("Hiatt"), json!("Johnson"), json!("Tedson")];
    let possible_zips = vec![json!(12333), json!(97012), json!(21312)];
    let first_names = Box::new(DiscreteValues::new(&possible_first_names)) as Box<dyn Values>;
    let last_names = Box::new(DiscreteValues::new(&possible_last_names)) as Box<dyn Values>;
    let zips = Box::new(DiscreteValues::new(&possible_zips)) as Box<dyn Values>;
    let mut nested_schema_structure = HashMap::new();
    nested_schema_structure.insert("zip_code".to_owned(), zips);
    let nested_schema = Box::new(ObjectValues::new(nested_schema_structure)) as Box<dyn Values>;
    let mut outer_schema = HashMap::new();
    outer_schema.insert("first_name".to_owned(), first_names);
    outer_schema.insert("last_name".to_owned(), last_names);
    outer_schema.insert("address".to_owned(), nested_schema);
    let val = create_json_vec_from_schema(&outer_schema, 10);
    _ = dump_json_array(&val, "nested.json");
}

#[test]
fn test_all_three_types() {
    let possible_first_names = vec![json!("Adam"), json!("John"), json!("Ted")];
    let possible_last_names = vec![json!("Hiatt"), json!("Johnson"), json!("Tedson")];
    let possible_zips = vec![json!(12333), json!(97012), json!(21312)];
    let first_names = Box::new(DiscreteValues::new(&possible_first_names)) as Box<dyn Values>;
    let last_names = Box::new(DiscreteValues::new(&possible_last_names)) as Box<dyn Values>;
    let zips = Box::new(DiscreteValues::new(&possible_zips)) as Box<dyn Values>;
    let ages = Box::new(RangedValues::new(12, 37)) as Box<dyn Values>;
    let mut nested_schema_structure = HashMap::new();
    nested_schema_structure.insert("zip_code".to_owned(), zips);
    let nested_schema = Box::new(ObjectValues::new(nested_schema_structure)) as Box<dyn Values>;
    let mut outer_schema = HashMap::new();
    outer_schema.insert("first_name".to_owned(), first_names);
    outer_schema.insert("last_name".to_owned(), last_names);
    outer_schema.insert("age".to_owned(), ages);
    outer_schema.insert("address".to_owned(), nested_schema);
    let val = create_json_vec_from_schema(&outer_schema, 10);
    _ = dump_json_array(&val, "nested.json");
}
