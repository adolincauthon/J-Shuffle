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
    let count = u32::from(args.count);
    let schema = generate_template_from_schema(args.input_schema.as_str());
    if count == 1 {
        let json = create_json_from_schema(&schema);
        _ = dump_value(json, &args.output_schema);
    } else {
        let json = create_json_vec_from_schema(&schema, count);
        _ = dump_json_array(&json, &args.output_schema);
    }
}
