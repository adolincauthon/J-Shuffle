use rand::seq::SliceRandom;
use rand::thread_rng;

pub mod json_utility {
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
    pub fn create_json_from_schema(schema: &HashMap<&str, &impl Values>) -> Value {
        let mut val = json!({});
        for key in schema.keys() {
            val[key] = json!(schema.get(key).unwrap().get_value());
        }
        val
    }

    ///Takes a schema and generates a vec of JSON objects
    pub fn create_json_vec_from_schema(
        schema: &HashMap<&str, &impl Values>,
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
}

///contains default values
pub mod default_values {
    use crate::*;
    use rand::Rng;
    use serde_json::{json, Value};

    pub trait Values {
        fn get_value(&self) -> Value;
    }

    pub struct DiscreteValues<'a> {
        possible: &'a Vec<Value>,
    }

    impl<'a> DiscreteValues<'a> {
        pub fn new(values: &'a Vec<Value>) -> Self {
            DiscreteValues { possible: values }
        }
    }

    impl<'a> Values for DiscreteValues<'a> {
        // need to get range of random numbers
        fn get_value(&self) -> Value {
            let mut rng = thread_rng();
            self.possible.choose(&mut rng).unwrap().clone()
        }
    }

    pub struct RangedValues<T: Into<u64> + Copy> {
        start: T,
        end: T,
    }

    impl<T: Into<u64> + Copy> RangedValues<T> {
        pub fn new(start: T, end: T) -> Self {
            RangedValues { start, end }
        }
    }

    impl<T: Into<u64> + Copy> Values for RangedValues<T> {
        // need to get range of random numbers
        fn get_value(&self) -> Value {
            let mut rng = thread_rng();
            let start_range = Into::<u64>::into(self.start);
            let end_range = Into::<u64>::into(self.end);
            let val = rng.gen_range(start_range..end_range);
            json!(val)
        }
    }
}
