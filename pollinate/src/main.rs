//! # Pollinate
//!
//! `pollinate` is a Rust crate for parsing and generating random values based on JSON schemas. It provides utilities for defining default values, parsing schema details, and generating templates for creating randomized data conforming to specified JSON schemas.
//!
//! ## Features
//!
//! - **Schema Parsing:** The crate supports parsing JSON schema details, including handling integer, string, array, and object types.
//! - **Random Data Generation:** Using the parsed schema, `pollinate` can generate templates containing random values for each specified field.
//! - **Extensibility:** Custom value types can be easily added by implementing the `Values` trait.
//! Usage: pollinate [OPTIONS] --input-schema <INPUT_SCHEMA> --output-schema <OUTPUT_SCHEMA>
//! Options:
//! -i, --input-schema <INPUT_SCHEMA>    Path to input schema
//! -o, --output-schema <OUTPUT_SCHEMA>  Path to output schema
//! -c, --count <COUNT>                  Number of objects to create [default: 1]
//! -h, --help                           Print help
//! -V, --version                        Print version

use clap::Parser;
use pollinate::{
    json_utils::{
        create_json_from_schema, create_json_vec_from_schema, dump_json_array, dump_value,
    },
    schema::generate_template_from_schema,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to input schema
    #[arg(short, long)]
    input_schema: String,

    /// Path to output schema
    #[arg(short, long)]
    output_schema: String,

    /// Number of objects to create
    #[arg(short, long, default_value_t = 1)]
    count: u32,
}

fn main() {
    let args = Args::parse();
    let count = args.count;
    let schema = generate_template_from_schema(args.input_schema.as_str());
    if count == 1 {
        let json = create_json_from_schema(&schema);
        _ = dump_value(json, &args.output_schema);
    } else {
        let json = create_json_vec_from_schema(&schema, count);
        _ = dump_json_array(&json, &args.output_schema);
    }
}
