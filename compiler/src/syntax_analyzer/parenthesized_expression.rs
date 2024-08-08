use std::any::Any;

use crate::util::{
    expression::Expression,
    literals::{LiteralType, LiteralValue},
    syntax_kind::SyntaxKind,
    syntax_token::SyntaxToken,
};

#[derive(Debug)]
pub struct ParenthesizedExpressionSyntax {
    open_parenthesis_token: SyntaxToken,
    expression: Box<dyn Expression>,
    close_parenthesis_token: SyntaxToken,
}

impl Clone for ParenthesizedExpressionSyntax {
    fn clone(&self) -> Self {
        ParenthesizedExpressionSyntax {
            open_parenthesis_token: self.open_parenthesis_token.clone(),
            expression: self.expression.clone(),
            close_parenthesis_token: self.close_parenthesis_token.clone(),
        }
    }
}

impl ParenthesizedExpressionSyntax {
    pub fn new(
        open_parenthesis_token: SyntaxToken,
        expression: Box<dyn Expression>,
        close_parenthesis_token: SyntaxToken,
    ) -> Self {
        Self {
            open_parenthesis_token,
            expression,
            close_parenthesis_token,
        }
    }

    pub fn get_expression(&self) -> Box<dyn Expression> {
        self.expression.clone()
    }
}

impl Expression for ParenthesizedExpressionSyntax {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_kind(&self) -> &SyntaxKind {
        &SyntaxKind::ParenthesizedExpression
    }

    fn get_children(&self) -> Vec<Box<dyn Expression>> {
        vec![
            Box::new(self.open_parenthesis_token.clone()) as Box<dyn Expression>,
            self.expression.clone(),
            Box::new(self.close_parenthesis_token.clone()) as Box<dyn Expression>,
        ]
    }

    fn get_value(&self) -> LiteralValue {
        todo!()
    }

    fn get_type(&self) -> &LiteralType {
        todo!()
    }
}
