use std::any::Any;

use super::{bound_kind::BoundKind, bound_statement::BoundStatement};

#[derive(Debug)]
pub struct BoundStatementList {
    statements: Vec<Box<dyn BoundStatement>>,
}

impl Clone for BoundStatementList {
    fn clone(&self) -> Self {
        BoundStatementList {
            statements: self.statements.clone(),
        }
    }
}

impl BoundStatementList {
    pub fn new(statements: Vec<Box<dyn BoundStatement>>) -> Self {
        Self { statements }
    }

    pub fn get_statements(&self) -> Vec<Box<dyn BoundStatement>> {
        self.statements.clone()
    }
}

impl BoundStatement for BoundStatementList {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_of_bound(&self) -> &BoundKind {
        &BoundKind::BoundStatementList
    }
}
