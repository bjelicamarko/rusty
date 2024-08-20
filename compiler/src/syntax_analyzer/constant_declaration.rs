use std::any::Any;

use crate::util::{
    expression::Expression, statement::Statement, syntax_kind::SyntaxKind,
    syntax_token::SyntaxToken,
};

#[derive(Debug)]
pub struct ConstantDeclaration {
    const_token: SyntaxToken,
    variable: SyntaxToken,
    equals: SyntaxToken,
    expression: Box<dyn Expression>,
    semi_colon: SyntaxToken,
}

impl Clone for ConstantDeclaration {
    fn clone(&self) -> Self {
        ConstantDeclaration {
            const_token: self.const_token.clone(),
            variable: self.variable.clone(),
            equals: self.equals.clone(),
            expression: self.expression.clone(),
            semi_colon: self.semi_colon.clone(),
        }
    }
}

impl ConstantDeclaration {
    pub fn new(
        const_token: SyntaxToken,
        variable: SyntaxToken,
        equals: SyntaxToken,
        expression: Box<dyn Expression>,
        semi_colon: SyntaxToken,
    ) -> Self {
        Self {
            const_token,
            variable,
            equals,
            expression,
            semi_colon,
        }
    }
}

impl Statement for ConstantDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_kind(&self) -> &SyntaxKind {
        &SyntaxKind::ConstantDeclaration
    }
}
