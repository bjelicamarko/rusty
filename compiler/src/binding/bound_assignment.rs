use crate::util::variable_symbol::VariableSymbol;

use super::{bound_expression::BoundExpression, bound_statement::BoundStatement};

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
}

impl BoundStatement for BoundAssignment {}
