use std::any::Any;

use crate::util::expression::Expression;
use crate::util::literals::{LiteralType, LiteralValue};
use crate::util::syntax_kind::SyntaxKind;
use crate::util::syntax_token::SyntaxToken;

#[derive(Debug)]
pub struct LiteralExpressionSyntax {
    literal_token: SyntaxToken,
    value: LiteralValue,
}

impl LiteralExpressionSyntax {
    pub fn new(token: SyntaxToken, value: LiteralValue) -> Self {
        Self {
            literal_token: token,
            value,
        }
    }
}

impl Clone for LiteralExpressionSyntax {
    fn clone(&self) -> Self {
        LiteralExpressionSyntax {
            literal_token: self.literal_token.clone(),
            value: self.value.clone(),
        }
    }
}

impl Expression for LiteralExpressionSyntax {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_kind(&self) -> &SyntaxKind {
        &SyntaxKind::LiteralExpression
    }

    fn get_children(&self) -> Vec<Box<dyn Expression>> {
        vec![Box::new(self.literal_token.clone()) as Box<dyn Expression>]
    }

    fn get_value(&self) -> LiteralValue {
        self.value.clone()
    }

    fn get_type(&self) -> &LiteralType {
        self.value.get_type()
    }
}
