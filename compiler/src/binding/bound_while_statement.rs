use std::any::Any;

use super::{
    bound_expression::BoundExpression, bound_kind::BoundKind, bound_statement::BoundStatement,
};

#[derive(Debug)]
pub struct BoundWhileStatement {
    condition: Box<dyn BoundExpression>,
    body: Box<dyn BoundStatement>,
}

impl Clone for BoundWhileStatement {
    fn clone(&self) -> Self {
        BoundWhileStatement {
            condition: self.condition.clone(),
            body: self.body.clone(),
        }
    }
}

impl BoundWhileStatement {
    pub fn new(condition: Box<dyn BoundExpression>, body: Box<dyn BoundStatement>) -> Self {
        Self { condition, body }
    }

    pub fn get_condition(&self) -> Box<dyn BoundExpression> {
        self.condition.clone()
    }

    pub fn get_body(&self) -> Box<dyn BoundStatement> {
        self.body.clone()
    }
}

impl BoundStatement for BoundWhileStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_of_bound(&self) -> &BoundKind {
        &BoundKind::BoundWhileStatement
    }
}
