use std::any::Any;

use crate::util::{
    expression::Expression,
    literals::{LiteralType, LiteralValue},
    syntax_kind::SyntaxKind,
    syntax_token::SyntaxToken,
};

#[derive(Debug)]
pub struct UnaryExpressionSyntax {
    operator: SyntaxToken,
    operand: Box<dyn Expression>,
}

impl Clone for UnaryExpressionSyntax {
    fn clone(&self) -> Self {
        UnaryExpressionSyntax {
            operator: self.operator.clone(),
            operand: self.operand.clone(),
        }
    }
}

impl UnaryExpressionSyntax {
    pub fn new(operator: SyntaxToken, operand: Box<dyn Expression>) -> Self {
        Self { operator, operand }
    }

    pub fn operand(&self) -> Box<dyn Expression> {
        self.operand.clone()
    }

    pub fn operator(&self) -> &SyntaxToken {
        &self.operator
    }
}

impl Expression for UnaryExpressionSyntax {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_kind(&self) -> &SyntaxKind {
        &SyntaxKind::UnaryExpression
    }

    fn get_children(&self) -> Vec<Box<dyn Expression>> {
        vec![
            Box::new(self.operator.clone()) as Box<dyn Expression>,
            self.operand.clone(),
        ]
    }

    fn get_value(&self) -> LiteralValue {
        todo!()
    }

    fn get_type(&self) -> &LiteralType {
        todo!()
    }
}
