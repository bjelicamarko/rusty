use std::any::Any;

use crate::util::{
    expression::Expression, statement::Statement, syntax_kind::SyntaxKind,
    syntax_token::SyntaxToken,
};

#[derive(Debug)]
pub struct VariableDeclaration {
    let_token: SyntaxToken,
    variable: SyntaxToken,
    equals: SyntaxToken,
    expression: Box<dyn Expression>,
    semi_colon: SyntaxToken,
}

impl Clone for VariableDeclaration {
    fn clone(&self) -> Self {
        VariableDeclaration {
            let_token: self.let_token.clone(),
            variable: self.variable.clone(),
            equals: self.equals.clone(),
            expression: self.expression.clone(),
            semi_colon: self.semi_colon.clone(),
        }
    }
}

impl VariableDeclaration {
    pub fn new(
        let_token: SyntaxToken,
        variable: SyntaxToken,
        equals: SyntaxToken,
        expression: Box<dyn Expression>,
        semi_colon: SyntaxToken,
    ) -> Self {
        Self {
            let_token,
            variable,
            equals,
            expression,
            semi_colon,
        }
    }
}

impl Statement for VariableDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_kind(&self) -> &SyntaxKind {
        &SyntaxKind::VariableDeclaration
    }
}
