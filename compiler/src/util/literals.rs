#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
pub enum LiteralType {
    Integer,
    String,
    Boolean,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LiteralValue {
    Integer(i32),
    String(String),
    Boolean(bool),
}

impl LiteralValue {
    pub fn get_type(&self) -> &LiteralType {
        match *self {
            LiteralValue::Boolean(_) => &LiteralType::Boolean,
            LiteralValue::Integer(_) => &LiteralType::Integer,
            LiteralValue::String(_) => &LiteralType::String,
        }
    }

    pub fn as_integer(&self) -> Option<i32> {
        if let LiteralValue::Integer(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        if let LiteralValue::Boolean(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<String> {
        if let LiteralValue::String(value) = self {
            Some(value.to_string())
        } else {
            None
        }
    }
}
