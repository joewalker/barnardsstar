
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
    List(Vec<Value>),
}
