#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
//!
//! This library extends serde_json::Value by adding a recursively digging function.
//! 
//! Allows the caller to dig deeply in a given JSON value.
//! Example:
//! ```
//! use serde_json::json;
//! use serde_dig::{Dig};
//! fn main() {
//!     // The type of `john` is `serde_json::Value`
//!     let john = json!({
//!         "name": "John Doe",
//!         "age": 43,
//!         "phones": [
//!             "+44 1234567",
//!             "+44 2345678"
//!         ]
//!     });
//!     println!("first phone number: {}", john.get_deep(&["phones".into(), 0.into()]).unwrap());
//! }
//! ```
use serde_json::{Value};

/// This is a union of the two indexing types when exploring JSON
#[derive(Debug)]
pub enum DigIndex<'a> {
    /// represents string keys for serde_json::Value::Object  
    String(&'a str),
    /// for vector access for serde_json::Value::Array
    Index(usize)
}

impl<'a> From<&'a str> for DigIndex<'a> {
    fn from(v: &'a str) -> Self {
        DigIndex::String(v)
    }
}

impl From<usize> for DigIndex<'_> {
    fn from(v: usize) -> Self {
        DigIndex::Index(v)
    }
}

/// trait that allows for recursively getting until the slice is exhausted
pub trait Dig<T> {
    /// Attempt to find the serde_json::Value object stored in the path represented by the slice
    fn get_deep(&self, key: &[T]) -> Option<&Value>;
}

impl Dig<&str> for Value {
    fn get_deep(&self, key: &[&str]) -> Option<&Value> {
        if key.len() == 0 {
            return Some(&self);
        }
        let k = &key[0];
        return match self {
            Value::Object(obj) => {
                obj.get(*k)?.get_deep(&key[1..])
            }
            _ => None
        };   
    }
}

impl Dig<usize> for Value {
    fn get_deep(&self, key: &[usize]) -> Option<&Value> {
        if key.len() == 0 {
            return Some(&self);
        }
        let k = &key[0];
        return match self {
            Value::Array(arr) => {
                arr.get(*k)?.get_deep(&key[1..])
            },
            _ => None
        };
    }
}

impl<'a> Dig<DigIndex<'a>> for Value {
    fn get_deep(&self, key: &[DigIndex<'a>]) -> Option<&Self> {
        if key.len() == 0 {
            return Some(&self);
        }
        let k = &key[0];
        return match self {
            Value::Array(arr) => {
                match k {
                    DigIndex::Index(i) => arr.get(*i)?.get_deep(&key[1..]),
                    _ => None
                }
            },
            Value::Object(obj) => {
                match k {
                    DigIndex::String(i) => obj.get(*i)?.get_deep(&key[1..]),
                    _ => None
                }
            }
            _ => None
        };
    }
}


#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::Dig;
    use crate::DigIndex;

    #[test]
    fn dig_test() {
        let object = json!({ "A": 65, "B": [1, 5], "C": {"foo": "bar", "baz": [4]} });
        assert_eq!(object.get_deep(&["A"]).unwrap().as_i64().unwrap(), 65);
        assert_eq!(object.get_deep(&[DigIndex::String("B"), DigIndex::Index(1)]).unwrap().as_i64().unwrap(), 5);
    }

    #[test]
    fn dig_str_test() {
        let object = json!({ "A": 65, "B": [1, 5], "C": {"foo": "bar", "baz": [4]} });
        assert_eq!(object.get_deep(&["A"]).unwrap().as_i64().unwrap(), 65);
    }

    #[test]
    fn dig_vec_test() {
        let object = json!([[1,2], [5]]);
        assert_eq!(object.get_deep(&[0, 1]).unwrap().as_i64().unwrap(), 2);
    }
}
