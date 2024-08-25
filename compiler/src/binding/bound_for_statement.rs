use std::any::Any;

use crate::util::variable_symbol::VariableSymbol;

use super::{
    bound_expression::BoundExpression, bound_kind::BoundKind, bound_statement::BoundStatement,
};

#[derive(Debug)]
pub struct BoundForStatement {
    variable: VariableSymbol,
    lower_bound: Box<dyn BoundExpression>,
    upper_bound: Box<dyn BoundExpression>,
    body: Box<dyn BoundStatement>,
}

impl Clone for BoundForStatement {
    fn clone(&self) -> Self {
        BoundForStatement {
            variable: self.variable.clone(),
            lower_bound: self.lower_bound.clone(),
            upper_bound: self.upper_bound.clone(),
            body: self.body.clone(),
        }
    }
}

impl BoundForStatement {
    pub fn new(
        variable: VariableSymbol,
        lower_bound: Box<dyn BoundExpression>,
        upper_bound: Box<dyn BoundExpression>,
        body: Box<dyn BoundStatement>,
    ) -> Self {
        Self {
            variable,
            lower_bound,
            upper_bound,
            body,
        }
    }

    pub fn get_variable(&self) -> &VariableSymbol {
        &self.variable
    }

    pub fn get_lower_bound(&self) -> Box<dyn BoundExpression> {
        self.lower_bound.clone()
    }

    pub fn get_upper_bound(&self) -> Box<dyn BoundExpression> {
        self.upper_bound.clone()
    }

    pub fn get_body(&self) -> Box<dyn BoundStatement> {
        self.body.clone()
    }
}

impl BoundStatement for BoundForStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_of_bound(&self) -> &BoundKind {
        &BoundKind::BoundForStatement
    }
}
