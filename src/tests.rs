
mod edn {
    include!(concat!(env!("OUT_DIR"), "/edn.rs"));
}

use self::edn::*;
use types::Value;

/*
#[test]
fn test_query() {
    assert!(variable("?r").is_ok());

    assert!(find_spec(":find ?r").is_ok());
    assert!(query(":fund").is_err());
}

#[test]
fn test_active_sessions() {
    const ACTIVE_SESSIONS: &'static str = "
:find ?id ?reason ?ts
:in $
:where
[?id :session/startReason ?reason ?tx]
[?tx :db/txInstant ?ts]
    ";
    // (not-join [?id]
    //     [?id :session/endReason _])

    let reply = find_spec(ACTIVE_SESSIONS);
    println!("reply = {:?}", reply);
    assert!(reply.is_ok());
}
*/

#[test]
fn test_nil() {
    assert_eq!(nil("nil").unwrap(), Value::Nil);

    assert!(nil("true").is_err());
}

#[test]
fn test_boolean() {
    assert_eq!(boolean("true").unwrap(), Value::Boolean(true));
    assert_eq!(boolean("false").unwrap(), Value::Boolean(false));

    assert!(boolean("nil").is_err());
}

#[test]
fn test_integer() {
    assert_eq!(integer("0").unwrap(), Value::Integer(0i32));
    assert_eq!(integer("1").unwrap(), Value::Integer(1i32));
    assert_eq!(integer("999").unwrap(), Value::Integer(999i32));
    assert_eq!(integer("-999").unwrap(), Value::Integer(-999i32));

    assert!(integer("nil").is_err());
}

#[test]
fn test_text() {
    assert_eq!(text("\"hello world\"").unwrap(), Value::Text("hello world".to_string()));
    assert_eq!(text("\"\"").unwrap(), Value::Text("".to_string()));

    assert!(text("\"").is_err());
    assert!(text("nil").is_err());
}

#[test]
fn test_symbol() {
    assert_eq!(symbol("r_-r").unwrap(), Value::Symbol("r_-r".to_string()));
    assert_eq!(symbol("$symbol").unwrap(), Value::Symbol("$symbol".to_string()));
    assert_eq!(symbol("hello").unwrap(), Value::Symbol("hello".to_string()));
}

#[test]
fn test_keyword() {
    assert_eq!(keyword(":hello/world").unwrap(), Value::Keyword(":hello/world".to_string()));
    assert_eq!(keyword(":symbol").unwrap(), Value::Keyword(":symbol".to_string()));
    assert_eq!(keyword(":hello").unwrap(), Value::Keyword(":hello".to_string()));
}

#[test]
fn test_value() {
    assert_eq!(value("nil").unwrap(), Value::Nil);
    assert_eq!(value("true").unwrap(), Value::Boolean(true));
    assert_eq!(value("1").unwrap(), Value::Integer(1i32));
    assert_eq!(value("\"hello world\"").unwrap(), Value::Text("hello world".to_string()));
    assert_eq!(value("$symbol").unwrap(), Value::Symbol("$symbol".to_string()));
    assert_eq!(value(":hello").unwrap(), Value::Keyword(":hello".to_string()));
}

#[test]
fn test_list() {
    let test = "()";
    let value = Value::List(vec![
    ]);
    assert_eq!(list(test).unwrap(), value);

    let test = "(1)";
    let value = Value::List(vec![
        Value::Integer(1),
    ]);
    assert_eq!(list(test).unwrap(), value);

    let test = "(nil)";
    let value = Value::List(vec![
        Value::Nil,
    ]);
    assert_eq!(list(test).unwrap(), value);

    let test = "(1 2)";
    let value = Value::List(vec![
        Value::Integer(1),
        Value::Integer(2),
    ]);
    assert_eq!(list(test).unwrap(), value);

    let test = "(1 2 3)";
    let value = Value::List(vec![
        Value::Integer(1),
        Value::Integer(2),
        Value::Integer(3),
    ]);
    assert_eq!(list(test).unwrap(), value);

    let test = "(1 0 nil \"nil\")";
    let value = Value::List(vec![
        Value::Integer(1),
        Value::Integer(0),
        Value::Nil,
        Value::Text("nil".to_string()),
    ]);
    assert_eq!(list(test).unwrap(), value);

    let test = "(1 (0 nil) \"nil\")";
    let value = Value::List(vec![
        Value::Integer(1),
        Value::List(vec![
            Value::Integer(0),
            Value::Nil,
        ]),
        Value::Text("nil".to_string()),
    ]);
    assert_eq!(list(test).unwrap(), value);

    assert!(list("(").is_err());
    assert!(list("1)").is_err());
    assert!(list("(1 (2 nil) \"hi\"").is_err());
}
