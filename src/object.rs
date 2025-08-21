use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Number(f64),
    Boolean(bool),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Number(n) => write!(f, "{}", n),
            Object::Boolean(b) => write!(f, "{}", b),
        }
    }
}