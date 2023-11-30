//! Default Values
//! Contains default values for dynamic JSON object generation.
//!
//! # Examples
//!
//! ```
//! use pollinate::default_values::*;
//! use serde_json::*;
//! use std::collections::HashMap;
//! // Define discrete values
//! let discrete_values = DiscreteValues::new(&[json!("A"), json!("B"), json!("C")]);
//!
//! // Get a random value from discrete values
//! let random_discrete_value = discrete_values.get_value();
//!
//! // Define ranged values
//! let ranged_values = RangedValues::new(1, 100);
//!
//! // Get a random value within the range
//! let random_ranged_value = ranged_values.get_value();
//!
//! // Define an object schema
//! let mut object_schema = HashMap::new();
//! object_schema.insert("field1".to_string(), Box::new(DiscreteValues::new(&[json!("X"), json!("Y")])) as Box<dyn Values>);
//! object_schema.insert("field2".to_string(), Box::new(RangedValues::new(10, 50) )as Box<dyn Values>);
//! let object_values = ObjectValues::new(object_schema);
//!
//! // Generate a JSON object based on the schema
//! let generated_object = object_values.get_value();
//!
//! // Define array values with a minimum of 1 and a maximum of 5 elements, each being a discrete value
//! let array_values = ArrayValues::new(1, 5, Box::new(DiscreteValues::new(&[json!("P"), json!("Q")])));
//!
//! // Generate a JSON array based on the array values
//! let generated_array = array_values.get_value();
//! ```
use crate::json_utils::create_json_from_schema;
use dyn_clone::DynClone;
use rand::{seq::SliceRandom, thread_rng, Rng};
use serde_json::{json, Value};
use std::collections::HashMap;

///Returns a random variable from the possible values
pub trait Values: DynClone {
    fn get_value(&self) -> Value;
}

dyn_clone::clone_trait_object!(Values);

/// Instance of DiscreteValues with the given possible values.
///
/// # Examples
///
/// ```
/// use pollinate::default_values::*;
/// use serde_json::*;
///
/// let values = DiscreteValues::new(&[json!("A"), json!("B"), json!("C")]);
/// let value: Value = values.get_value();
/// assert!(value == json!("A") || value == json!("B") || value == json!("C"));
/// ```
#[derive(Debug, Clone)]
pub struct DiscreteValues {
    possible: Vec<Value>,
}

///Represents a set of discrete possible values
impl DiscreteValues {
    pub fn new(values: &[Value]) -> Self {
        DiscreteValues {
            possible: values.to_owned().clone(),
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

/// Creates a new instance of RangedValues with the given start and end values.
/// Range must start and end with a valid i64 integer
///
/// # Examples
///
/// ```
/// use pollinate::default_values::*;
/// use serde_json::*;
///
/// let values = RangedValues::new(1, 100);
/// let value = values.get_value();
/// assert!(1 <= value.as_i64().unwrap() && value.as_i64().unwrap() <= 100);
/// ```
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
    schema: HashMap<String, Box<dyn Values + 'a>>,
}

impl<'a> ObjectValues<'a> {
    pub fn new(schema: HashMap<String, Box<dyn Values + 'a>>) -> Self {
        ObjectValues { schema }
    }
}

impl<'a> Values for ObjectValues<'a> {
    fn get_value(&self) -> Value {
        create_json_from_schema(&self.schema)
    }
}

/// Creates new instances of array values with a random number of
/// Elements between min and max inclusive.
/// Consists of a singular type of value (types)
#[derive(Clone)]
pub struct ArrayValues {
    min: u32,
    max: u32,
    types: Box<dyn Values>,
}

impl ArrayValues {
    pub fn new(min: u32, max: u32, types: Box<dyn Values>) -> Self {
        ArrayValues { min, max, types }
    }
}

impl Values for ArrayValues {
    fn get_value(&self) -> Value {
        let mut rng = thread_rng();
        let val = rng.gen_range(self.min..=self.max);
        let mut my_values = Vec::with_capacity(val as usize);
        for _ in 0..=val {
            let new_value = self.types.get_value();
            my_values.push(new_value);
        }
        json!(my_values)
    }
}
