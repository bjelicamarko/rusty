use std::any::Any;

use crate::util::literals::LiteralType;

use super::{
    bound_binary_operator::BoundBinaryOperator, bound_expression::BoundExpression,
    bound_kind::BoundKind,
};

#[derive(Debug)]
pub struct BoundBinaryExpression {
    left: Box<dyn BoundExpression>,
    operator: BoundBinaryOperator,
    right: Box<dyn BoundExpression>,
}

impl Clone for BoundBinaryExpression {
    fn clone(&self) -> Self {
        BoundBinaryExpression {
            left: self.left.clone(),
            operator: self.operator.clone(),
            right: self.right.clone(),
        }
    }
}

impl BoundBinaryExpression {
    pub fn new(
        left: Box<dyn BoundExpression>,
        operator: BoundBinaryOperator,
        right: Box<dyn BoundExpression>,
    ) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }

    pub fn get_left(&self) -> Box<dyn BoundExpression> {
        self.left.clone()
    }

    pub fn get_right(&self) -> Box<dyn BoundExpression> {
        self.right.clone()
    }

    pub fn get_operator(&self) -> &BoundBinaryOperator {
        &self.operator
    }
}

impl BoundExpression for BoundBinaryExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type(&self) -> &LiteralType {
        self.operator.get_type()
    }

    fn get_type_of_bound(&self) -> &BoundKind {
        &BoundKind::BoundBinaryExpression
    }
}
