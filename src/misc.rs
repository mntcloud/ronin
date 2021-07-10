use std::collections::HashMap;

pub type Object<T> = HashMap<String, T>;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;