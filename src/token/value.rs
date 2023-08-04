use serde::{Deserialize, Serialize};
use std::{fmt, fmt::Display};

macro_rules! impl_from {
    ($wrapper:path; $inner_type:ty; $($from:ty),+) => {
        $(impl From<$from> for Value {
            fn from(value: $from) -> Self {
                $wrapper(<$inner_type>::from(value))
            }
        })+
    };
}

// parse the enum and generate Value::T(ty) -> ty
// macro_rules! and_back_again {
//
// }
//

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Integer(i128),
    Float(f64),
    Boolean(bool),
    Null,
}

impl_from!(Value::Integer; i128; u8, u16, u32, u64, i8, i16, i32, i64, i128);
impl_from!(Value::Float; f64; f32, f64);
impl_from!(Value::String; String; String);
impl_from!(Value::Boolean; bool; bool);

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
