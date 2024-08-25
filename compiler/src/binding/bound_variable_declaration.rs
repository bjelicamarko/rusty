use std::any::Any;

use crate::util::variable_symbol::VariableSymbol;

use super::{
    bound_expression::BoundExpression, bound_kind::BoundKind, bound_statement::BoundStatement,
};

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

    pub fn get_variable(&self) -> VariableSymbol {
        self.variable.clone()
    }

    pub fn get_bound_expression(&self) -> Box<dyn BoundExpression> {
        self.expression.clone()
    }
}

impl BoundStatement for BoundVariableDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_of_bound(&self) -> &BoundKind {
        &BoundKind::BoundVariableDeclaration
    }
}
