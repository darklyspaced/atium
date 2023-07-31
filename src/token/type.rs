use super::value::Value;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    String,
    Integer,
    Float,
    Boolean,
    Null,
}

impl From<Value> for Type {
    fn from(value: Value) -> Self {
        match value {
            Value::String(_) => Self::String,
            Value::Integer(_) => Self::Integer,
            Value::Float(_) => Self::Float,
            Value::Boolean(_) => Self::Boolean,
            Value::Null => Self::Null,
        }
    }
}
