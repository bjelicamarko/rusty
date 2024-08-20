use std::any::Any;

use crate::util::variable_symbol::VariableSymbol;

use super::{bound_expression::BoundExpression, bound_statement::BoundStatement};

#[derive(Debug)]
pub struct BoundVariableDeclaration {
    variable: VariableSymbol,
    expression: Box<dyn BoundExpression>,
}

impl Clone for BoundVariableDeclaration {
    fn clone(&self) -> Self {
        BoundVariableDeclaration {
            variable: self.variable.clone(),
            expression: self.expression.clone(),
        }
    }
}

impl BoundVariableDeclaration {
    pub fn new(variable: VariableSymbol, expression: Box<dyn BoundExpression>) -> Self {
        Self {
            variable,
            expression,
        }
    }
}

impl BoundStatement for BoundVariableDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
