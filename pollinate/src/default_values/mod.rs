///contains default values
use std::collections::HashMap;

use crate::json_utils::create_json_from_schema;
use dyn_clone::DynClone;
use rand::{seq::SliceRandom, thread_rng, Rng};
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
    schema: HashMap<String, Box<dyn Values + 'a>>,
}

impl<'a> ObjectValues<'a> {
    pub fn new(schema: HashMap<String, Box<dyn Values + 'a>>) -> Self {
        ObjectValues { schema }
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
