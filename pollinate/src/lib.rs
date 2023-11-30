//! # Pollinate
//!
//! `pollinate` is a Rust crate for parsing and generating random values based on JSON schemas. It provides utilities for defining default values, parsing schema details, and generating templates for creating randomized data conforming to specified JSON schemas.
//!
//! ## Features
//!
//! - **Schema Parsing:** The crate supports parsing JSON schema details, including handling integer, string, array, and object types.
//! - **Random Data Generation:** Using the parsed schema, `pollinate` can generate templates containing random values for each specified field.
//! - **Extensibility:** Custom value types can be easily added by implementing the `Values` trait.
//!
//!
#![allow(dead_code)]
pub mod default_values;
pub mod json_utils;
pub mod schema;
