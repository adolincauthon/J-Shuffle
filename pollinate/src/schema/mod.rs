use serde_json::*;
use std::collections::HashMap;

use crate::default_values::{ArrayValues, DiscreteValues, ObjectValues, RangedValues, Values};

fn parse_integer(details: &Value) -> Box<dyn Values> {
    if let Some(x) = details.get("enum") {
        return Box::new(DiscreteValues::new(x.as_array().unwrap())) as Box<dyn Values>;
    }
    if let Some(min) = details.get("minimum").unwrap().as_i64() {
        if let Some(max) = details.get("maximum").unwrap().as_i64() {
            return Box::new(RangedValues::new(min, max)) as Box<dyn Values>;
        }
        Box::new(RangedValues::new(min, i64::MAX)) as Box<dyn Values>
    } else {
        let min = i64::MIN;
        if let Some(max) = details.get("maximum").unwrap().as_i64() {
            return Box::new(RangedValues::new(min, max)) as Box<dyn Values>;
        }
        Box::new(RangedValues::new(min, i64::MAX)) as Box<dyn Values>
    }
}

fn parse_string(details: &Value) -> Box<dyn Values> {
    let enum_values = details.get("enum").unwrap().as_array().unwrap();
    return Box::new(DiscreteValues::new(enum_values)) as Box<dyn Values>;
}

fn parse_array(details: &Value) -> Box<dyn Values> {
    if let Some(x) = details.get("maximum") {
        let mut min = 0;
        let max = x.as_u64().unwrap() as u32;
        if let Some(n) = details.get("minimum") {
            min = n.as_u64().unwrap() as u32;
        }
        let types = parse_type(&details.get("items").unwrap()).unwrap();
        return Box::new(ArrayValues::new(min, max, types)) as Box<dyn Values>;
    } else {
        panic!("Arrays must have max value")
    }
}

fn parse_object(details: &Value) -> Box<dyn Values> {
    let properties = details.get("properties").unwrap();
    let properties = properties.as_object().unwrap();
    let schema = populate_schema(properties);
    Box::new(ObjectValues::new(schema)) as Box<dyn Values>
}

fn parse_type(details: &Value) -> Option<Box<dyn Values>> {
    match details.get("type").unwrap().as_str() {
        Some("string") => return Some(parse_string(details)),
        Some("integer") => return Some(parse_integer(details)),
        Some("array") => return Some(parse_array(details)),
        Some("object") => return Some(parse_object(details)),
        Some(_) => todo!(),
        None => todo!(),
    };
}

///Gets possible values for type
fn populate_schema<'a>(properties: &Map<String, Value>) -> HashMap<String, Box<dyn Values + 'a>> {
    let mut enumerated_schema = HashMap::<String, Box<dyn Values>>::new();
    for (property, details) in properties.iter() {
        if let Some(val) = parse_type(details) {
            enumerated_schema.insert(property.to_owned(), val);
        }
    }
    enumerated_schema
}

///generate a template to create random values based on JSON Schema
// TODO: OsStrings for filepath
pub fn generate_template_from_schema<'a>(
    schema_path: &str,
) -> HashMap<String, Box<dyn Values + 'a>> {
    let schema_string =
        std::fs::read_to_string(schema_path).expect("Should have been able to read the file");
    let parsed_schema: Map<String, Value> = serde_json::from_str(&schema_string).unwrap();
    let enumerated_schema = populate_schema(
        &parsed_schema
            .get("properties")
            .unwrap()
            .as_object()
            .unwrap(),
    );
    //temp return value for
    enumerated_schema
}
