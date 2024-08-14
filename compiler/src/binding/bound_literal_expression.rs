use std::any::Any;

use crate::util::literals::{LiteralType, LiteralValue};

use super::bound_expression::BoundExpression;

#[derive(Debug)]
pub struct BoundLiteralExpression {
    value: LiteralValue,
    type_of_value: LiteralType,
}

impl Clone for BoundLiteralExpression {
    fn clone(&self) -> Self {
        BoundLiteralExpression {
            value: self.value.clone(),
            type_of_value: self.type_of_value.clone(),
        }
    }
}

impl BoundLiteralExpression {
    pub fn new(value: LiteralValue, type_of_value: LiteralType) -> Self {
        Self {
            value,
            type_of_value,
        }
    }

    pub fn get_value(&self) -> LiteralValue {
        self.value.clone()
    }
}

impl BoundExpression for BoundLiteralExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type(&self) -> &LiteralType {
        &self.type_of_value
    }
}
