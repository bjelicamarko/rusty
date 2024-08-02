use std::any::Any;

use crate::util::literals::{LiteralType, LiteralValue};

use super::bound_expression::BoundExpression;

#[derive(Debug)]
pub struct BoundLiteralExpression {
    value: LiteralValue,
}

impl Clone for BoundLiteralExpression {
    fn clone(&self) -> Self {
        BoundLiteralExpression {
            value: self.value.clone(),
        }
    }
}

impl BoundLiteralExpression {
    pub fn new(value: LiteralValue) -> Self {
        Self { value }
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
        self.value.get_type()
    }
}
