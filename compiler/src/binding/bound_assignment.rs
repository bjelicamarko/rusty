use std::any::Any;

use crate::util::variable_symbol::VariableSymbol;

use super::{
    bound_expression::BoundExpression, bound_kind::BoundKind, bound_statement::BoundStatement,
};

#[derive(Debug)]
pub struct BoundAssignment {
    variable: VariableSymbol,
    expression: Box<dyn BoundExpression>,
}

impl Clone for BoundAssignment {
    fn clone(&self) -> Self {
        BoundAssignment {
            variable: self.variable.clone(),
            expression: self.expression.clone(),
        }
    }
}

impl BoundAssignment {
    pub fn new(variable: VariableSymbol, expression: Box<dyn BoundExpression>) -> Self {
        Self {
            variable,
            expression,
        }
    }

    pub fn get_variable(&self) -> VariableSymbol {
        self.variable.clone()
    }

    pub fn get_bound_expression(&self) -> Box<dyn BoundExpression> {
        self.expression.clone()
    }
}

impl BoundStatement for BoundAssignment {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_of_bound(&self) -> &BoundKind {
        &BoundKind::BoundAssignment
    }
}
