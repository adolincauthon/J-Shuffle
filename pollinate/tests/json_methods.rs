#[cfg(test)]
mod json_tests {
    use std::collections::HashMap;

    use pollinate::{default_values::DiscreteValues, json_utility::*};
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
        let first_names = DiscreteValues::new(&possible_first_names);
        let last_names = DiscreteValues::new(&possible_last_names);
        let zips = DiscreteValues::new(&possible_zips);
        let mut schema = HashMap::new();
        schema.insert("first_name", &first_names);
        schema.insert("last_name", &last_names);
        schema.insert("zip_code", &zips);

        let val = create_json_from_schema(&schema);
        println!("{:?}", val);
    }

    #[test]
    fn return_json_object_array() {
        let possible_first_names = vec![json!("Adam"), json!("John"), json!("Ted")];
        let possible_last_names = vec![json!("Hiatt"), json!("Johnson"), json!("Tedson")];
        let possible_zips = vec![json!(12333), json!(97012), json!(21312)];
        let first_names = DiscreteValues::new(&possible_first_names);
        let last_names = DiscreteValues::new(&possible_last_names);
        let zips = DiscreteValues::new(&possible_zips);
        let mut schema = HashMap::new();
        schema.insert("first_name", &first_names);
        schema.insert("last_name", &last_names);
        schema.insert("zip_code", &zips);

        let val = create_json_vec_from_schema(&schema, 100);
        println!("{:?}", val);
    }

    #[test]
    fn dump_json_object_from_schema() {
        let possible_first_names = vec![json!("Adam"), json!("John"), json!("Ted")];
        let possible_last_names = vec![json!("Hiatt"), json!("Johnson"), json!("Tedson")];
        let possible_zips = vec![json!(12333), json!(97012), json!(21312)];
        let first_names = DiscreteValues::new(&possible_first_names);
        let last_names = DiscreteValues::new(&possible_last_names);
        let zips = DiscreteValues::new(&possible_zips);
        let mut schema = HashMap::new();
        schema.insert("first_name", &first_names);
        schema.insert("last_name", &last_names);
        schema.insert("zip_code", &zips);

        let val = create_json_from_schema(&schema);
        let _ = dump_json(&val, "temp.json");
        println!("{:?}", val);
    }

    #[test]
    fn dump_json_vec_from_schema() {
        let possible_first_names = vec![json!("Adam"), json!("John"), json!("Ted")];
        let possible_last_names = vec![json!("Hiatt"), json!("Johnson"), json!("Tedson")];
        let possible_zips = vec![json!(12333), json!(97012), json!(21312)];
        let first_names = DiscreteValues::new(&possible_first_names);
        let last_names = DiscreteValues::new(&possible_last_names);
        let zips = DiscreteValues::new(&possible_zips);
        let mut schema = HashMap::new();
        schema.insert("first_name", &first_names);
        schema.insert("last_name", &last_names);
        schema.insert("zip_code", &zips);

        let val = create_json_vec_from_schema(&schema, 5000);
        let _ = dump_json_array(&val, "temp.json");
        println!("{:?}", val);
    }
}
