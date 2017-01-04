
use std::collections::LinkedList;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Integer(i32),
    Float(f64),
    Text(String),
    Symbol(String),
    Keyword(String),
    Vector(Vec<Value>),
    List(LinkedList<Value>),
}
