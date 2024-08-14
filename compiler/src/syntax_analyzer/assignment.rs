use std::any::Any;

use crate::util::{
    expression::Expression, statement::Statement, syntax_kind::SyntaxKind,
    syntax_token::SyntaxToken,
};

#[derive(Debug)]
pub struct Assignment {
    variable: SyntaxToken,
    equals: SyntaxToken,
    expression: Box<dyn Expression>,
    semi_colon: SyntaxToken,
}

impl Clone for Assignment {
    fn clone(&self) -> Self {
        Assignment {
            variable: self.variable.clone(),
            equals: self.equals.clone(),
            expression: self.expression.clone(),
            semi_colon: self.semi_colon.clone(),
        }
    }
}

impl Assignment {
    pub fn new(
        variable: SyntaxToken,
        equals: SyntaxToken,
        expression: Box<dyn Expression>,
        semi_colon: SyntaxToken,
    ) -> Self {
        Self {
            variable,
            equals,
            expression,
            semi_colon,
        }
    }

    pub fn get_variable(&self) -> SyntaxToken {
        self.variable.clone()
    }

    pub fn get_expression(&self) -> Box<dyn Expression> {
        self.expression.clone()
    }
}

impl Statement for Assignment {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_kind(&self) -> &SyntaxKind {
        &SyntaxKind::Assignment
    }
}
