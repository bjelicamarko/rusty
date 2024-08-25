use std::any::Any;

use crate::util::{
    expression::Expression,
    literals::{LiteralType, LiteralValue},
    syntax_kind::SyntaxKind,
    syntax_token::SyntaxToken,
};

#[derive(Debug)]
pub struct BinaryExpressionSyntax {
    left: Box<dyn Expression>,
    operator: SyntaxToken,
    right: Box<dyn Expression>,
}

impl Clone for BinaryExpressionSyntax {
    fn clone(&self) -> Self {
        BinaryExpressionSyntax {
            left: self.left.clone(),
            operator: self.operator.clone(),
            right: self.right.clone(),
        }
    }
}

impl BinaryExpressionSyntax {
    pub fn get_operator(&self) -> &SyntaxToken {
        &self.operator
    }

    pub fn get_left(&self) -> Box<dyn Expression> {
        self.left.clone()
    }

    pub fn get_right(&self) -> Box<dyn Expression> {
        self.right.clone()
    }

    pub fn new(
        token_left: Box<dyn Expression>,
        operator: SyntaxToken,
        token_right: Box<dyn Expression>,
    ) -> Self {
        Self {
            left: token_left,
            operator,
            right: token_right,
        }
    }
}

impl Expression for BinaryExpressionSyntax {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_kind(&self) -> &SyntaxKind {
        &SyntaxKind::BinaryExpression
    }

    fn get_children(&self) -> Vec<Box<dyn Expression>> {
        vec![
            self.left.clone(),
            Box::new(self.operator.clone()) as Box<dyn Expression>,
            self.right.clone(),
        ]
    }

    fn get_value(&self) -> LiteralValue {
        todo!()
    }

    fn get_type(&self) -> &LiteralType {
        todo!()
    }
}
