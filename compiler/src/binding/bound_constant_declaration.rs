use std::any::Any;

use crate::util::variable_symbol::VariableSymbol;

use super::{bound_expression::BoundExpression, bound_statement::BoundStatement};

#[derive(Debug)]
pub struct BoundConstantDeclaration {
    variable: VariableSymbol,
    expression: Box<dyn BoundExpression>,
}

impl Clone for BoundConstantDeclaration {
    fn clone(&self) -> Self {
        BoundConstantDeclaration {
            variable: self.variable.clone(),
            expression: self.expression.clone(),
        }
    }
}

impl BoundConstantDeclaration {
    pub fn new(variable: VariableSymbol, expression: Box<dyn BoundExpression>) -> Self {
        Self {
            variable,
            expression,
        }
    }
}

impl BoundStatement for BoundConstantDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
