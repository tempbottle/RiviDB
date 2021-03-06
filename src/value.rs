use heapsize::HeapSizeOf;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ValueType {
    Null,
    Bool(bool),
    Timestamp(u64),
    Integer(i64),
    Str(Rc<String>),
    Set(Rc<Vec<String>>),
}

pub type RecordType = Vec<(String, ValueType)>;

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ValueType::Null => write!(f, "null"),
            &ValueType::Bool(b) => write!(f, "{}", b),
            &ValueType::Timestamp(t) => write!(f, "t{}", t),
            &ValueType::Integer(i) => write!(f, "{}", i),
            &ValueType::Str(ref s) => write!(f, "\"{}\"", s),
            &ValueType::Set(ref vec) => write!(f, "{:?}", vec),
        }
    }
}

impl HeapSizeOf for ValueType {
    fn heap_size_of_children(&self) -> usize {
        use ValueType::*;
        match self {
            &Null | &Bool(_) | &Timestamp(_) | &Integer(_) => 0,
            &Str(ref r) => r.heap_size_of_children(),
            &Set(ref r) => r.heap_size_of_children(),
        }
    }
}
