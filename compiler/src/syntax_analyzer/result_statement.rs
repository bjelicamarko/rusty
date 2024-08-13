use crate::util::{expression::Expression, statement::Statement, syntax_token::SyntaxToken};

#[derive(Debug)]
pub struct ResultStatement {
    result: SyntaxToken,
    equals: SyntaxToken,
    expression: Box<dyn Expression>,
}

impl Clone for ResultStatement {
    fn clone(&self) -> Self {
        ResultStatement {
            result: self.result.clone(),
            equals: self.equals.clone(),
            expression: self.expression.clone(),
        }
    }
}

impl ResultStatement {
    pub fn new(result: SyntaxToken, equals: SyntaxToken, expression: Box<dyn Expression>) -> Self {
        Self {
            result,
            equals,
            expression,
        }
    }
}

impl Statement for ResultStatement {}
