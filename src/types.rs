
use std::collections::{BTreeSet, BTreeMap, LinkedList};
use std::cmp::{Ordering, Ord, PartialOrd};
use ordered_float::OrderedFloat;

/// We're using BTree{Set, Map} rather than Hash{Set, Map} because the BTree variants implement Hash
/// (unlike the Hash variants which don't in order to preserve O(n) hashing time which is hard given
/// recurrsive data structures)
/// See https://internals.rust-lang.org/t/implementing-hash-for-hashset-hashmap/3817/1
/// TODO: We should probably Box the collection types
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Value {
    Nil,
    Boolean(bool),
    Integer(i32),
    // https://users.rust-lang.org/t/hashmap-key-cant-be-float-number-type-why/7892
    Float(OrderedFloat<f64>),
    Text(String),
    Symbol(String),
    Keyword(String),
    Vector(Vec<Value>),
    List(LinkedList<Value>),
    Set(BTreeSet<Value>),
    Map(BTreeMap<Value, Value>),
}

use self::Value::*;

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// TODO: Check we follow the equality rules at the bottom of https://github.com/edn-format/edn
impl Ord for Value {
    fn cmp(&self, other: &Value) -> Ordering {

        let ord_order = to_ord(self).cmp(&to_ord(other));
        match *self {
            Nil             => match *other { Nil             => Ordering::Equal, _ => ord_order },
            Boolean(bs)     => match *other { Boolean(bo)     => bo.cmp(&bs), _ => ord_order },
            Integer(is)     => match *other { Integer(io)     => io.cmp(&is), _ => ord_order },
            Float(ref fs)   => match *other { Float(ref fo)   => fo.cmp(&fs), _ => ord_order },
            Text(ref ts)    => match *other { Text(ref to)    => to.cmp(&ts), _ => ord_order },
            Symbol(ref ss)  => match *other { Symbol(ref so)  => so.cmp(&ss), _ => ord_order },
            Keyword(ref ks) => match *other { Keyword(ref ko) => ko.cmp(&ks), _ => ord_order },
            Vector(ref vs)  => match *other { Vector(ref vo)  => vo.cmp(&vs), _ => ord_order },
            List(ref ls)    => match *other { List(ref lo)    => lo.cmp(&ls), _ => ord_order },
            Set(ref ss)     => match *other { Set(ref so)     => so.cmp(&ss), _ => ord_order },
            Map(ref ms)     => match *other { Map(ref mo)     => mo.cmp(&ms), _ => ord_order },
        }
    }
}

// TODO: There has to be a better way to do `as i32` for Value
fn to_ord(value: &Value) -> i32 {
    match *value {
        Nil => 0,
        Boolean(_) => 1,
        Integer(_) => 2,
        Float(_) => 3,
        Text(_) => 4,
        Symbol(_) => 5,
        Keyword(_) => 6,
        Vector(_) => 7,
        List(_) => 8,
        Set(_) => 9,
        Map(_) => 10,
    }
}

pub struct Pair(Value, Value);
