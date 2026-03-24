use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Str(String),
    Array(Vec<i64>),
    Vector(Vec<Value>),
    Stack(Vec<i64>),
    Map(HashMap<String, String>),
    Set(HashSet<String>),
    Option(Option<Box<Value>>),
    Result(Result<Box<Value>, String>),
}
