// TODO: Can't we do this just for tests?
#![allow(unused_imports)]

mod edn {
    include!(concat!(env!("OUT_DIR"), "/edn.rs"));
}

use std::collections::{BTreeSet, BTreeMap, LinkedList};
use std::iter::FromIterator;
use self::edn::*;
use types::Value::*;
use types::Pair;
use ordered_float::OrderedFloat;

#[test]
fn test_nil() {
    assert_eq!(nil("nil").unwrap(), Nil);

    assert!(nil("true").is_err());
}

#[test]
fn test_boolean() {
    assert_eq!(boolean("true").unwrap(), Boolean(true));
    assert_eq!(boolean("false").unwrap(), Boolean(false));

    assert!(boolean("nil").is_err());
}

#[test]
fn test_integer() {
    assert_eq!(integer("0").unwrap(), Integer(0i32));
    assert_eq!(integer("1").unwrap(), Integer(1i32));
    assert_eq!(integer("999").unwrap(), Integer(999i32));
    assert_eq!(integer("-999").unwrap(), Integer(-999i32));

    assert!(integer("nil").is_err());
}

#[test]
fn test_float() {
    assert_eq!(float("111.222").unwrap(), Float(OrderedFloat(111.222f64)));
    assert_eq!(float("3e4").unwrap(), Float(OrderedFloat(3e4f64)));
    assert_eq!(float("-55e-66").unwrap(), Float(OrderedFloat(-55e-66f64)));
    // assert_eq!(float("77.88e99").unwrap(), Float(OrderedFloat(77.88e99f64)));
    // assert_eq!(float("-9.9E-9").unwrap(), Float(OrderedFloat(-9.9E-9f64)));

    assert!(float("nil").is_err());
}

#[test]
fn test_text() {
    assert_eq!(text("\"hello world\"").unwrap(), Text("hello world".to_string()));
    assert_eq!(text("\"\"").unwrap(), Text("".to_string()));

    assert!(text("\"").is_err());
    assert!(text("nil").is_err());
}

#[test]
fn test_symbol() {
    assert_eq!(symbol("r_r").unwrap(), Symbol("r_r".to_string()));
    assert_eq!(symbol("$symbol").unwrap(), Symbol("$symbol".to_string()));
    assert_eq!(symbol("hello").unwrap(), Symbol("hello".to_string()));
}

#[test]
fn test_keyword() {
    assert_eq!(keyword(":hello/world").unwrap(), Keyword(":hello/world".to_string()));
    assert_eq!(keyword(":symbol").unwrap(), Keyword(":symbol".to_string()));
    assert_eq!(keyword(":hello").unwrap(), Keyword(":hello".to_string()));
}

#[test]
fn test_value() {
    assert_eq!(value("nil").unwrap(), Nil);
    assert_eq!(value("true").unwrap(), Boolean(true));
    assert_eq!(value("1").unwrap(), Integer(1i32));
    assert_eq!(value("\"hello world\"").unwrap(), Text("hello world".to_string()));
    assert_eq!(value("$symbol").unwrap(), Symbol("$symbol".to_string()));
    assert_eq!(value(":hello").unwrap(), Keyword(":hello".to_string()));
    assert_eq!(value("[1]").unwrap(), Vector(vec![Integer(1)]));
    // TODO: Why is this a parse error from a value context but not from a float context?
    // assert_eq!(value("111.222").unwrap(), Float(OrderedFloat(111.222f64)));
}

#[test]
fn test_vector() {
    let test = "[]";
    let value = Vector(vec![
    ]);
    assert_eq!(vector(test).unwrap(), value);

    let test = "[1]";
    let value = Vector(vec![
        Integer(1),
    ]);
    assert_eq!(vector(test).unwrap(), value);

    let test = "[nil]";
    let value = Vector(vec![
        Nil,
    ]);
    assert_eq!(vector(test).unwrap(), value);

    let test = "[1 2]";
    let value = Vector(vec![
        Integer(1),
        Integer(2),
    ]);
    assert_eq!(vector(test).unwrap(), value);

    // let test = "[1 2 3.4]";
    // let value = Vector(vec![
    //     Integer(1),
    //     Integer(2),
    //     Float(3.4f64),
    // ]);
    // assert_eq!(vector(test).unwrap(), value);

    let test = "[1 0 nil \"nil\"]";
    let value = Vector(vec![
        Integer(1),
        Integer(0),
        Nil,
        Text("nil".to_string()),
    ]);
    assert_eq!(vector(test).unwrap(), value);

    let test = "[1 [0 nil] \"nil\"]";
    let value = Vector(vec![
        Integer(1),
        Vector(vec![
            Integer(0),
            Nil,
        ]),
        Text("nil".to_string()),
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
    let value = List(LinkedList::from_iter(vec![
    ]));
    assert_eq!(list(test).unwrap(), value);

    let test = "(1)";
    let value = List(LinkedList::from_iter(vec![
        Integer(1),
    ]));
    assert_eq!(list(test).unwrap(), value);

    let test = "(nil)";
    let value = List(LinkedList::from_iter(vec![
        Nil,
    ]));
    assert_eq!(list(test).unwrap(), value);

    let test = "(1 2)";
    let value = List(LinkedList::from_iter(vec![
        Integer(1),
        Integer(2),
    ]));
    assert_eq!(list(test).unwrap(), value);

    // let test = "(1 2 3.4)";
    // let value = List(LinkedList::from_iter(vec![
    //     Integer(1),
    //     Integer(2),
    //     Float(3.4f64),
    // ]));
    // assert_eq!(list(test).unwrap(), value);

    let test = "(1 0 nil \"nil\")";
    let value = List(LinkedList::from_iter(vec![
        Integer(1),
        Integer(0),
        Nil,
        Text("nil".to_string()),
    ]));
    assert_eq!(list(test).unwrap(), value);

    let test = "(1 (0 nil) \"nil\")";
    let value = List(LinkedList::from_iter(vec![
        Integer(1),
        List(LinkedList::from_iter(vec![
            Integer(0),
            Nil,
        ])),
        Text("nil".to_string()),
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
    let value = Set(BTreeSet::from_iter(vec![
    ]));
    assert_eq!(set(test).unwrap(), value);

    let test = "#{1}";
    let value = Set(BTreeSet::from_iter(vec![
        Integer(1),
    ]));
    assert_eq!(set(test).unwrap(), value);

    let test = "#{nil}";
    let value = Set(BTreeSet::from_iter(vec![
        Nil,
    ]));
    assert_eq!(set(test).unwrap(), value);

    let test = "#{2 1}";
    let value = Set(BTreeSet::from_iter(vec![
        Integer(1),
        Integer(2),
    ]));
    assert_eq!(set(test).unwrap(), value);

    // let test = "#{3.4 2 1}";
    // let value = Set(BTreeSet::from_iter(vec![
    //     Integer(1),
    //     Integer(2),
    //     Float(3.4f64),
    // ]));
    // assert_eq!(set(test).unwrap(), value);

    let test = "#{1 0 nil \"nil\"}";
    let value = Set(BTreeSet::from_iter(vec![
        Nil,
        Integer(0),
        Integer(1),
        Text("nil".to_string()),
    ]));
    assert_eq!(set(test).unwrap(), value);

    let test = "#{1 #{0 nil} \"nil\"}";
    let value = Set(BTreeSet::from_iter(vec![
        Integer(1),
        Set(BTreeSet::from_iter(vec![
            Nil,
            Integer(0),
        ])),
        Text("nil".to_string()),
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
    let value = Map(BTreeMap::from_iter(vec![
    ]));
    assert_eq!(map(test).unwrap(), value);

    let test = "{\"a\" 1}";
    let value = Map(BTreeMap::from_iter(vec![
        (Text("a".to_string()), Integer(1)),
    ]));
    assert_eq!(map(test).unwrap(), value);

    let test = "{nil 1, \"b\" 2}";
    let value = Map(BTreeMap::from_iter(vec![
        (Nil, Integer(1)),
        (Text("b".to_string()), Integer(2)),
    ]));
    assert_eq!(map(test).unwrap(), value);

    let test = "{nil 1, \"b\" 2, \"a\" 3}";
    let value = Map(BTreeMap::from_iter(vec![
        (Nil, Integer(1)),
        (Text("a".to_string()), Integer(3)),
        (Text("b".to_string()), Integer(2)),
    ]));
    assert_eq!(map(test).unwrap(), value);

    let test = "{:a 1, $b {:b/a nil, :b/b #{nil 5}}, c [1 2], d (3 4)}";
    let value = Map(BTreeMap::from_iter(vec![
        (Keyword(":a".to_string()), Integer(1)),
        (Symbol("$b".to_string()), Map(BTreeMap::from_iter(vec![
            (Keyword(":b/a".to_string()), Nil),
            (Keyword(":b/b".to_string()), Set(BTreeSet::from_iter(vec![
                Nil,
                Integer(5),
            ]))),
        ]))),
        (Symbol("c".to_string()), Vector(vec![
            Integer(1),
            Integer(2),
        ])),
        (Symbol("d".to_string()), List(LinkedList::from_iter(vec![
            Integer(3),
            Integer(4),
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
    let test = "[
        :find ?id ?reason ?ts
        :in $
        :where
            [?id :session/startReason ?reason ?tx]
            [?tx :db/txInstant ?ts]
            (not-join [?id] [?id :session/endReason _])
        ]";

    let reply = Vector(vec![
        Keyword(":find".to_string()),
        Symbol("?id".to_string()),
        Symbol("?reason".to_string()),
        Symbol("?ts".to_string()),
        Keyword(":in".to_string()),
        Symbol("$".to_string()),
        Keyword(":where".to_string()),
        Vector(vec![
            Symbol("?id".to_string()),
            Keyword(":session/startReason".to_string()),
            Symbol("?reason".to_string()),
            Symbol("?tx".to_string()),
        ]),
        Vector(vec![
            Symbol("?tx".to_string()),
            Keyword(":db/txInstant".to_string()),
            Symbol("?ts".to_string()),
        ]),
        List(LinkedList::from_iter(vec![
            Symbol("not-join".to_string()),
            Vector(vec![
                Symbol("?id".to_string()),
            ]),
            Vector(vec![
                Symbol("?id".to_string()),
                Keyword(":session/endReason".to_string()),
                Symbol("_".to_string()),
            ]),
        ])),
    ]);
    assert_eq!(value(test).unwrap(), reply);
}
