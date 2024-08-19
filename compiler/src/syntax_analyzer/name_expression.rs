use std::any::Any;

use crate::util::expression::Expression;
use crate::util::literals::{LiteralType, LiteralValue};
use crate::util::syntax_kind::SyntaxKind;
use crate::util::syntax_token::SyntaxToken;

#[derive(Debug)]
pub struct NameExpressionSyntax {
    token: SyntaxToken,
}

impl NameExpressionSyntax {
    pub fn new(token: SyntaxToken) -> Self {
        Self { token }
    }

    pub fn get_token(&self) -> SyntaxToken {
        self.token.clone()
    }
}

impl Clone for NameExpressionSyntax {
    fn clone(&self) -> Self {
        NameExpressionSyntax {
            token: self.token.clone(),
        }
    }
}

impl Expression for NameExpressionSyntax {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_kind(&self) -> &SyntaxKind {
        &SyntaxKind::NameExpression
    }

    fn get_children(&self) -> Vec<Box<dyn Expression>> {
        vec![Box::new(self.token.clone()) as Box<dyn Expression>]
    }

    fn get_value(&self) -> LiteralValue {
        self.token.get_value().clone()
    }

    fn get_type(&self) -> &LiteralType {
        self.token.get_type()
    }
}
