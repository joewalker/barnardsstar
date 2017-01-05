// TODO: Can't we do this just for tests?
#![allow(unused_imports)]

mod edn {
    include!(concat!(env!("OUT_DIR"), "/edn.rs"));
}

use std::collections::{BTreeSet, BTreeMap, LinkedList};
use std::iter::FromIterator;
use self::edn::*;
use types::{Value, Pair};

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

/*
https://users.rust-lang.org/t/hashmap-key-cant-be-float-number-type-why/7892
#[test]
fn test_float() {
    assert_eq!(float("0").unwrap(), Value::Float(0f64));
    assert_eq!(float("1.1").unwrap(), Value::Float(1.1f64));
    assert_eq!(float("9.9e9").unwrap(), Value::Float(9.9e9f64));
    assert_eq!(float("-9.9E-9").unwrap(), Value::Float(-9.9E-9f64));

    assert!(float("nil").is_err());
}
*/

#[test]
fn test_text() {
    assert_eq!(text("\"hello world\"").unwrap(), Value::Text("hello world".to_string()));
    assert_eq!(text("\"\"").unwrap(), Value::Text("".to_string()));

    assert!(text("\"").is_err());
    assert!(text("nil").is_err());
}

#[test]
fn test_symbol() {
    assert_eq!(symbol("r_r").unwrap(), Value::Symbol("r_r".to_string()));
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
    assert_eq!(value("[1]").unwrap(), Value::Vector(vec![Value::Integer(1)]));
    //assert_eq!(value("9.9").unwrap(), Value::Float(9.9f64));
}

#[test]
fn test_vector() {
    let test = "[]";
    let value = Value::Vector(vec![
    ]);
    assert_eq!(vector(test).unwrap(), value);

    let test = "[1]";
    let value = Value::Vector(vec![
        Value::Integer(1),
    ]);
    assert_eq!(vector(test).unwrap(), value);

    let test = "[nil]";
    let value = Value::Vector(vec![
        Value::Nil,
    ]);
    assert_eq!(vector(test).unwrap(), value);

    let test = "[1 2]";
    let value = Value::Vector(vec![
        Value::Integer(1),
        Value::Integer(2),
    ]);
    assert_eq!(vector(test).unwrap(), value);

    // let test = "[1 2 3.4]";
    // let value = Value::Vector(vec![
    //     Value::Integer(1),
    //     Value::Integer(2),
    //     Value::Float(3.4f64),
    // ]);
    // assert_eq!(vector(test).unwrap(), value);

    let test = "[1 0 nil \"nil\"]";
    let value = Value::Vector(vec![
        Value::Integer(1),
        Value::Integer(0),
        Value::Nil,
        Value::Text("nil".to_string()),
    ]);
    assert_eq!(vector(test).unwrap(), value);

    let test = "[1 [0 nil] \"nil\"]";
    let value = Value::Vector(vec![
        Value::Integer(1),
        Value::Vector(vec![
            Value::Integer(0),
            Value::Nil,
        ]),
        Value::Text("nil".to_string()),
    ]);
    assert_eq!(vector(test).unwrap(), value);

    assert!(vector("[").is_err());
    assert!(vector("(").is_err());
    assert!(vector("1)").is_err());
    assert!(vector("(1 (2 nil) \"hi\"").is_err());
}

#[test]
fn test_list() {
    let test = "()";
    let value = Value::List(LinkedList::from_iter(vec![
    ]));
    assert_eq!(list(test).unwrap(), value);

    let test = "(1)";
    let value = Value::List(LinkedList::from_iter(vec![
        Value::Integer(1),
    ]));
    assert_eq!(list(test).unwrap(), value);

    let test = "(nil)";
    let value = Value::List(LinkedList::from_iter(vec![
        Value::Nil,
    ]));
    assert_eq!(list(test).unwrap(), value);

    let test = "(1 2)";
    let value = Value::List(LinkedList::from_iter(vec![
        Value::Integer(1),
        Value::Integer(2),
    ]));
    assert_eq!(list(test).unwrap(), value);

    // let test = "(1 2 3.4)";
    // let value = Value::List(LinkedList::from_iter(vec![
    //     Value::Integer(1),
    //     Value::Integer(2),
    //     Value::Float(3.4f64),
    // ]));
    // assert_eq!(list(test).unwrap(), value);

    let test = "(1 0 nil \"nil\")";
    let value = Value::List(LinkedList::from_iter(vec![
        Value::Integer(1),
        Value::Integer(0),
        Value::Nil,
        Value::Text("nil".to_string()),
    ]));
    assert_eq!(list(test).unwrap(), value);

    let test = "(1 (0 nil) \"nil\")";
    let value = Value::List(LinkedList::from_iter(vec![
        Value::Integer(1),
        Value::List(LinkedList::from_iter(vec![
            Value::Integer(0),
            Value::Nil,
        ])),
        Value::Text("nil".to_string()),
    ]));
    assert_eq!(list(test).unwrap(), value);

    assert!(list("[").is_err());
    assert!(list("(").is_err());
    assert!(list("1)").is_err());
    assert!(list("(1 (2 nil) \"hi\"").is_err());
}

#[test]
fn test_set() {
    let test = "#{}";
    let value = Value::Set(BTreeSet::from_iter(vec![
    ]));
    assert_eq!(set(test).unwrap(), value);

    let test = "#{1}";
    let value = Value::Set(BTreeSet::from_iter(vec![
        Value::Integer(1),
    ]));
    assert_eq!(set(test).unwrap(), value);

    let test = "#{nil}";
    let value = Value::Set(BTreeSet::from_iter(vec![
        Value::Nil,
    ]));
    assert_eq!(set(test).unwrap(), value);

    let test = "#{2 1}";
    let value = Value::Set(BTreeSet::from_iter(vec![
        Value::Integer(1),
        Value::Integer(2),
    ]));
    assert_eq!(set(test).unwrap(), value);

    // let test = "#{3.4 2 1}";
    // let value = Value::Set(BTreeSet::from_iter(vec![
    //     Value::Integer(1),
    //     Value::Integer(2),
    //     Value::Float(3.4f64),
    // ]));
    // assert_eq!(set(test).unwrap(), value);

    let test = "#{1 0 nil \"nil\"}";
    let value = Value::Set(BTreeSet::from_iter(vec![
        Value::Nil,
        Value::Integer(0),
        Value::Integer(1),
        Value::Text("nil".to_string()),
    ]));
    assert_eq!(set(test).unwrap(), value);

    let test = "#{1 #{0 nil} \"nil\"}";
    let value = Value::Set(BTreeSet::from_iter(vec![
        Value::Integer(1),
        Value::Set(BTreeSet::from_iter(vec![
            Value::Nil,
            Value::Integer(0),
        ])),
        Value::Text("nil".to_string()),
    ]));
    assert_eq!(set(test).unwrap(), value);

    assert!(set("#{").is_err());
    assert!(set("}").is_err());
    assert!(set("1}").is_err());
    assert!(set("#{1 #{2 nil} \"hi\"").is_err());
}

#[test]
fn test_map() {
    let test = "{}";
    let value = Value::Map(BTreeMap::from_iter(vec![
    ]));
    assert_eq!(map(test).unwrap(), value);

    let test = "{\"a\" 1}";
    let value = Value::Map(BTreeMap::from_iter(vec![
        (Value::Text("a".to_string()), Value::Integer(1)),
    ]));
    assert_eq!(map(test).unwrap(), value);

    let test = "{nil 1, \"b\" 2}";
    let value = Value::Map(BTreeMap::from_iter(vec![
        (Value::Nil, Value::Integer(1)),
        (Value::Text("b".to_string()), Value::Integer(2)),
    ]));
    assert_eq!(map(test).unwrap(), value);

    let test = "{nil 1, \"b\" 2, \"a\" 3}";
    let value = Value::Map(BTreeMap::from_iter(vec![
        (Value::Nil, Value::Integer(1)),
        (Value::Text("a".to_string()), Value::Integer(3)),
        (Value::Text("b".to_string()), Value::Integer(2)),
    ]));
    assert_eq!(map(test).unwrap(), value);

    let test = "{:a 1, $b {:b/a nil, :b/b #{nil 5}}, c [1 2], d (3 4)}";
    let value = Value::Map(BTreeMap::from_iter(vec![
        (Value::Keyword(":a".to_string()), Value::Integer(1)),
        (Value::Symbol("$b".to_string()), Value::Map(BTreeMap::from_iter(vec![
            (Value::Keyword(":b/a".to_string()), Value::Nil),
            (Value::Keyword(":b/b".to_string()), Value::Set(BTreeSet::from_iter(vec![
                Value::Nil,
                Value::Integer(5),
            ]))),
        ]))),
        (Value::Symbol("c".to_string()), Value::Vector(vec![
            Value::Integer(1),
            Value::Integer(2),
        ])),
        (Value::Symbol("d".to_string()), Value::List(LinkedList::from_iter(vec![
            Value::Integer(3),
            Value::Integer(4),
        ]))),
    ]));
    assert_eq!(map(test).unwrap(), value);

    assert!(map("#{").is_err());
    assert!(map("}").is_err());
    assert!(map("1}").is_err());
    assert!(map("#{1 #{2 nil} \"hi\"").is_err());
}

#[test]
fn test_query() {
    let test = "[:find ?id ?reason ?ts :in $ :where [?id :session/startReason ?reason ?tx] [?tx :db/txInstant ?ts]]";
        // (not-join [?id]
        //     [?id :session/endReason _])

    let reply = Value::Vector(vec![
        Value::Keyword(":find".to_string()),
        Value::Symbol("?id".to_string()),
        Value::Symbol("?reason".to_string()),
        Value::Symbol("?ts".to_string()),
        Value::Keyword(":in".to_string()),
        Value::Symbol("$".to_string()),
        Value::Keyword(":where".to_string()),
        Value::Vector(vec![
            Value::Symbol("?id".to_string()),
            Value::Keyword(":session/startReason".to_string()),
            Value::Symbol("?reason".to_string()),
            Value::Symbol("?tx".to_string()),
        ]),
        Value::Vector(vec![
            Value::Symbol("?tx".to_string()),
            Value::Keyword(":db/txInstant".to_string()),
            Value::Symbol("?ts".to_string()),
        ]),
    ]);
    assert_eq!(value(test).unwrap(), reply);
}
