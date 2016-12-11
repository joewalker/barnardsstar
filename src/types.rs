
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Value {
    Nil,
    Integer(i32),
    Boolean(bool),
    Text(String),
    Symbol(String),
    Keyword(String),
    List(Vec<Value>),
}
