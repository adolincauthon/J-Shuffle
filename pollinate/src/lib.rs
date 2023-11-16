use dyn_clone::DynClone;
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
    pub fn create_json_from_schema<'a>(schema: &HashMap<&str, Box<dyn Values + 'a>>) -> Value {
        let mut val = json!({});
        for key in schema.keys() {
            val[key] = schema.get(key).unwrap().get_value();
        }
        val
    }

    ///Takes a schema and generates a vec of JSON objects
    pub fn create_json_vec_from_schema<'a>(
        schema: &HashMap<&str, Box<dyn Values + 'a>>,
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
    use std::collections::HashMap;

    use crate::{json_utility::create_json_from_schema, *};
    use rand::Rng;
    use serde_json::{json, Value};

    ///Returns a random variable from the possible values
    pub trait Values: DynClone {
        fn get_value(&self) -> Value;
    }

    dyn_clone::clone_trait_object!(Values);

    #[derive(Debug, Clone)]
    pub struct DiscreteValues {
        possible: Vec<Value>,
    }

    ///Represents a set of possible values
    impl DiscreteValues {
        pub fn new<'a>(values: &'a Vec<Value>) -> Self {
            DiscreteValues {
                possible: values.clone(),
            }
        }
    }

    impl Values for DiscreteValues {
        // need to get range of random numbers
        fn get_value(&self) -> Value {
            let mut rng = thread_rng();
            self.possible.choose(&mut rng).unwrap().clone()
        }
    }

    ///Provides a range of values. Values must fit in an i64
    #[derive(Clone)]
    pub struct RangedValues<T: Into<i64> + Copy> {
        start: T,
        end: T,
    }

    impl<T: Into<i64> + Copy> RangedValues<T> {
        pub fn new(start: T, end: T) -> Self {
            RangedValues { start, end }
        }
    }

    impl<T: Into<i64> + Copy> Values for RangedValues<T> {
        // need to get range of random numbers
        fn get_value(&self) -> Value {
            let mut rng = thread_rng();
            let start_range = Into::<i64>::into(self.start);
            let end_range = Into::<i64>::into(self.end);
            let val = rng.gen_range(start_range..end_range);
            json!(val)
        }
    }

    ///Schema for an JSON object made up of dynamic values
    #[derive(Clone)]
    pub struct ObjectValues<'a> {
        schema: HashMap<&'a str, Box<dyn Values + 'a>>,
    }

    impl<'a> ObjectValues<'a> {
        pub fn new(schema: &'a HashMap<&str, Box<dyn Values + 'a>>) -> Self {
            ObjectValues {
                schema: schema.clone(),
            }
        }
    }

    impl<'a> Values for ObjectValues<'a> {
        // need to get range of random numbers
        fn get_value(&self) -> Value {
            create_json_from_schema(&self.schema)
        }
    }

    ///Possible Array Values/Size
    #[derive(Clone)]
    pub struct ArrayValues {
        min: u32,
        max: u32,
        types: Vec<Box<dyn Values>>,
    }

    impl ArrayValues {
        fn new<'a>(min: u32, max: u32, types: &'a Vec<Box<dyn Values>>) -> Self {
            ArrayValues {
                min,
                max,
                types: types.clone(),
            }
        }
    }

    impl Values for ArrayValues {
        fn get_value(&self) -> Value {
            let mut rng = thread_rng();
            let val = rng.gen_range(self.min..=self.max);
            let mut my_values = Vec::with_capacity(val as usize);
            for _ in 0..=val {
                let new_value = self.types.choose(&mut rng).unwrap().get_value();
                my_values.push(new_value);
            }
            json!(my_values)
        }
    }
}

mod schema_utility {
    use serde::{de::Error, de::Visitor, Deserialize};
    use serde_json::*;
    use std::collections::HashMap;

    use crate::default_values::{DiscreteValues, RangedValues, Values};
    enum SchemaType {
        object,
    }

    #[derive(Deserialize, Debug)]
    struct JsonSchema {
        title: String,
        #[serde(alias = "type")]
        schemaType: String,
        properties: Map<String, Value>,
    }

    //Gets possible values for type
    //TODO
    // fn get_value_from_item(item: Map<String, Value>) -> impl Values
    // where
    //     V: Values,
    // {
    //     match item.get("type").unwrap().as_str() {
    //         //only enums for now
    //         Some("string") => {
    //             //let possible_values: Vec<Value> = item.get("enum").unwrap().as_array().unwrap();
    //             //DiscreteValues::new(possible_values)
    //         }
    //         _ => panic!("Not a valid value"),
    //     }
    //     todo!()
    // }

    ///generate a template to create random values based on JSON Schema
    // TODO: OsStrings for filepath
    fn generate_template_from_schema<'a>(schema_path: &str) -> HashMap<&str, Box<dyn Values + 'a>> {
        let schema_string =
            std::fs::read_to_string(schema_path).expect("Should have been able to read the file");
        let parsed_schema: JsonSchema = serde_json::from_str(&schema_string).unwrap();
        println!("{:?}", parsed_schema);

        //temp return value for
        let mut map = HashMap::new();
        let val = Box::new(RangedValues::<u32>::new(0, 100)) as Box<dyn Values>;
        _ = map.insert("temp", val);
        map
    }

    // #[test]
    // fn test_template() {
    //     generate_template_from_schema(
    //         "/home/adam/repos/J-Shuffle/pollinate/test_data/person_schema.json",
    //     );
    // }
}
