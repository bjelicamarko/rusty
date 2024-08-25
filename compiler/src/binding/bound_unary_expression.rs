use std::any::Any;

use crate::util::literals::LiteralType;

use super::{
    bound_expression::BoundExpression, bound_kind::BoundKind,
    bound_unary_operator::BoundUnaryOperator,
};

#[derive(Debug)]
pub struct BoundUnaryExpression {
    operator: BoundUnaryOperator,
    operand: Box<dyn BoundExpression>,
}

impl Clone for BoundUnaryExpression {
    fn clone(&self) -> Self {
        BoundUnaryExpression {
            operator: self.operator.clone(),
            operand: self.operand.clone(),
        }
    }
}

impl BoundUnaryExpression {
    pub fn new(operator: BoundUnaryOperator, operand: Box<dyn BoundExpression>) -> Self {
        Self { operator, operand }
    }

    pub fn get_operand(&self) -> Box<dyn BoundExpression> {
        self.operand.clone()
    }

    pub fn get_operator(&self) -> &BoundUnaryOperator {
        &self.operator
    }
}

impl BoundExpression for BoundUnaryExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type(&self) -> &LiteralType {
        self.operator.get_type()
    }

    fn get_type_of_bound(&self) -> &BoundKind {
        &BoundKind::BoundUnaryExpression
    }
}
