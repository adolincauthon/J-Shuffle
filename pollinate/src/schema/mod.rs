//! # Pollinate Schema Module
//!
//! The `schema` module within the `pollinate` crate provides functionality for parsing JSON schemas and generating templates containing randomized data based on these schemas.
//!
//! ## Overview
//!
//! This module includes functions for parsing different types of JSON schema details, such as integers, strings, arrays, and objects. It also supports generating templates that include random values for each specified field, allowing users to create realistic data structures for testing or other purposes.
//!
//! ## Functions
//!
//! - `parse_integer`: Parses integer details from a JSON schema and returns a corresponding `Values` trait object.
//! - `parse_string`: Parses string details from a JSON schema and returns a corresponding `Values` trait object.
//! - `parse_array`: Parses array details from a JSON schema and returns a corresponding `Values` trait object.
//! - `parse_object`: Parses object details from a JSON schema and returns a corresponding `Values` trait object.
//! - `parse_type`: Parses the given details based on the type and returns a corresponding `Values` trait object.
//! - `populate_schema`: Generates a template of possible values for different types based on a JSON schema.
//! - `generate_template_from_schema`: Generates a template to create random values based on a JSON schema file.
//!

use serde_json::*;
use std::collections::HashMap;

use crate::default_values::{ArrayValues, DiscreteValues, ObjectValues, RangedValues, Values};

/// Parses integer details and returns a corresponding Values trait object.
///
/// # Examples
///
/// ```
/// use pollinate::default_values::*;
/// use pollinate::schema::*;
/// use serde_json::json;
///
/// let details = json!({"enum": [1, 2, 3]});
/// let values = parse_integer(&details);
/// let result = values.get_value();
/// let result = result.as_i64().unwrap();
/// assert!(result == 1 || result == 2 || result == 3);
/// ```
pub fn parse_integer(details: &Value) -> Box<dyn Values> {
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

/// Parses string details and returns a corresponding Values trait object.
///
/// # Examples
///
/// ```
/// use pollinate::default_values::{Values, DiscreteValues};
/// use serde_json::json;
///
/// let details = json!({"enum": ["A", "B", "C"]});
/// let values = pollinate::schema::parse_string(&details);
/// let result = values.get_value();
/// let result = result.as_str().unwrap();
/// assert!(result == "A" || result == "B" || result == "C");
/// ```
pub fn parse_string(details: &Value) -> Box<dyn Values> {
    let enum_values = details.get("enum").unwrap().as_array().unwrap();
    Box::new(DiscreteValues::new(enum_values)) as Box<dyn Values>
}

/// Parses array details and returns a corresponding Values trait object.
///
/// # Examples
///
/// ```
/// use pollinate::default_values::{Values, ArrayValues, DiscreteValues};
/// use serde_json::json;
///
/// let details = json!({"type": "array", "maximum": 5, "minimum": 2, "items": {"type": "string", "enum": ["X", "Y"]}});
/// let values = pollinate::schema::parse_array(&details);
/// let result = values.get_value();
/// let result = result.as_array().unwrap();
/// let len = result.len();
/// assert!(len <= 5 && len >= 2);
/// for item in result {
///   let item = item.as_str().unwrap();
///   assert!(item == "X" || item == "Y");
/// }
/// ```
pub fn parse_array(details: &Value) -> Box<dyn Values> {
    if let Some(x) = details.get("maximum") {
        let mut min = 0;
        let max = x.as_u64().unwrap() as u32;
        if let Some(n) = details.get("minimum") {
            min = n.as_u64().unwrap() as u32;
        }
        let types = parse_type(details.get("items").unwrap()).unwrap();
        Box::new(ArrayValues::new(min, max, types)) as Box<dyn Values>
    } else {
        panic!("Arrays must have max value")
    }
}

/// Parses object details and returns a corresponding Values trait object.
pub fn parse_object(details: &Value) -> Box<dyn Values> {
    let properties = details.get("properties").unwrap();
    let properties = properties.as_object().unwrap();
    let schema = populate_schema(properties);
    Box::new(ObjectValues::new(schema)) as Box<dyn Values>
}

/// Parses the given details based on the type and returns a corresponding Values trait object.
pub fn parse_type(details: &Value) -> Option<Box<dyn Values>> {
    match details.get("type").unwrap().as_str() {
        Some("string") => return Some(parse_string(details)),
        Some("integer") => return Some(parse_integer(details)),
        Some("array") => return Some(parse_array(details)),
        Some("object") => return Some(parse_object(details)),
        Some(_) => todo!(),
        None => todo!(),
    };
}

/// Gets possible values for a type based on a JSON schema.
/// Returns a HashMap schema for generating a JSON template
/// ```
pub fn populate_schema<'a>(
    properties: &Map<String, Value>,
) -> HashMap<String, Box<dyn Values + 'a>> {
    let mut enumerated_schema = HashMap::<String, Box<dyn Values>>::new();
    for (property, details) in properties.iter() {
        if let Some(val) = parse_type(details) {
            enumerated_schema.insert(property.to_owned(), val);
        }
    }
    enumerated_schema
}

/// Generates a template for creating JSON objects.
/// Takes path to JSON Schema as argument
/// See main.rs for example
pub fn generate_template_from_schema<'a>(
    schema_path: &str,
) -> HashMap<String, Box<dyn Values + 'a>> {
    let schema_string =
        std::fs::read_to_string(schema_path).expect("Should have been able to read the file");
    let parsed_schema: Map<String, Value> = serde_json::from_str(&schema_string).unwrap();
    let enumerated_schema = populate_schema(
        parsed_schema
            .get("properties")
            .unwrap()
            .as_object()
            .unwrap(),
    );
    //temp return value for
    enumerated_schema
}
