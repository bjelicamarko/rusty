use std::any::Any;

use super::{
    bound_expression::BoundExpression, bound_kind::BoundKind, bound_statement::BoundStatement,
};

#[derive(Debug)]
pub struct BoundIfStatement {
    condition: Box<dyn BoundExpression>,
    then_statement: Box<dyn BoundStatement>,
    else_statement: Option<Box<dyn BoundStatement>>,
}

impl Clone for BoundIfStatement {
    fn clone(&self) -> Self {
        BoundIfStatement {
            condition: self.condition.clone(),
            then_statement: self.then_statement.clone(),
            else_statement: self.else_statement.clone(),
        }
    }
}

impl BoundIfStatement {
    pub fn new(
        condition: Box<dyn BoundExpression>,
        then_statement: Box<dyn BoundStatement>,
        else_statement: Option<Box<dyn BoundStatement>>,
    ) -> Self {
        Self {
            condition,
            then_statement,
            else_statement,
        }
    }

    pub fn get_condition(&self) -> Box<dyn BoundExpression> {
        self.condition.clone()
    }

    pub fn get_then_statement(&self) -> Box<dyn BoundStatement> {
        self.then_statement.clone()
    }

    pub fn get_else_statement(&self) -> Option<Box<dyn BoundStatement>> {
        self.else_statement.clone()
    }
}

impl BoundStatement for BoundIfStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_of_bound(&self) -> &BoundKind {
        &BoundKind::BoundIfStatement
    }
}
