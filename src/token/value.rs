use std::{fmt, fmt::Display};

macro_rules! impl_integer {
    ($($type:ty),+) => {
        $(impl From<$type> for Value {
            fn from(value: $type) -> Self {
                Self::Integer(i128::from(value))
            }
        })+
    };
}

// parse the enum and generate Value::T(ty) -> ty
// macro_rules! and_back_again {
//
// }

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Integer(i128),
    Float(f64),
    Boolean(bool),
    Null,
}

impl_integer!(u8, u16, u32, u64, i8, i16, i32, i64, i128);

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(a) => write!(f, "{a}"),
            Self::Integer(a) => write!(f, "{a}"),
            Self::Float(a) => write!(f, "{a}"),
            Self::Boolean(a) => write!(f, "{a}"),
            Self::Null => write!(f, "Null"),
        }
    }
}
