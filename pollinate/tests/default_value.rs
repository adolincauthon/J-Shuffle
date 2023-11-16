#[cfg(test)]
mod default_values_test {
    use std::collections::HashMap;

    use pollinate::{
        default_values::{DiscreteValues, RangedValues, Values},
        json_utility::create_json_from_schema,
    };
    use serde::Serialize;
    use serde_json::{json, Value};

    #[test]
    fn get_random_value_string() {
        let possible_names = vec![json!("Adam"), json!("John"), json!("Ted")];
        let first_names = DiscreteValues::new(&possible_names);
        let name = first_names.get_value();
        println!("{:?}", name);
    }

    #[test]
    fn get_more_values_than_string_vec() {
        let possible_names = vec![json!("Adam"), json!("John"), json!("Ted")];
        let first_names = DiscreteValues::new(&possible_names);
        for _ in 1..5 {
            println!("{:?}", first_names.get_value());
        }
    }

    #[test]
    fn get_more_values_than_string_struct() {
        #[derive(Debug, Serialize)]
        struct Person<'a> {
            first_name: &'a str,
            last_name: &'a str,
        }

        let possible_people = vec![
            json!(Person {
                first_name: "Adam",
                last_name: "Hiatt",
            }),
            json!(Person {
                first_name: "John",
                last_name: "Diddly",
            }),
            json!(Person {
                first_name: "Jingle",
                last_name: "Schmidt",
            }),
        ];

        let people = DiscreteValues::new(&possible_people);
        for _ in 1..5 {
            println!("{:?}", people.get_value());
        }
    }

    #[test]
    fn get_range_of_values() {
        let zero_to_100 = Box::new(RangedValues::<u32>::new(0, 100)) as Box<dyn Values>;
        let mut schema = HashMap::new();
        schema.insert("value", zero_to_100);
        let mut objects = Vec::<Value>::new();
        for _ in 0..100 {
            objects.push(create_json_from_schema(&schema));
        }
        for val in objects {
            let _ = match val.get("value") {
                Some(Value::Number(x)) => {
                    assert!(x.as_u64() <= Some(100));
                    assert!(x.as_u64() >= Some(0));
                }
                Some(Value::Bool(_)) => panic!("Bool returned"),
                Some(Value::String(_)) => panic!("String returned"),
                Some(Value::Array(_)) => panic!("Array returned"),
                Some(Value::Object(x)) => {
                    println!("{:?}", x);
                    panic!("Object returned")
                }
                Some(Value::Null) => panic!("Null returned"),
                _ => panic!("None returned"),
            };
        }
    }
}
