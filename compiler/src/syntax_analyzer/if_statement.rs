use std::any::Any;

use crate::util::{
    expression::Expression, statement::Statement, syntax_kind::SyntaxKind,
    syntax_token::SyntaxToken,
};

#[derive(Debug)]
pub struct IfStatement {
    if_token: SyntaxToken,
    open_parenthesis: SyntaxToken,
    expression: Box<dyn Expression>,
    close_parenthesis: SyntaxToken,
    statement_list: Box<dyn Statement>,
    else_statement: Option<Box<dyn Statement>>,
}

impl Clone for IfStatement {
    fn clone(&self) -> Self {
        IfStatement {
            if_token: self.if_token.clone(),
            open_parenthesis: self.open_parenthesis.clone(),
            expression: self.expression.clone(),
            close_parenthesis: self.close_parenthesis.clone(),
            statement_list: self.statement_list.clone(),
            else_statement: if self.else_statement.is_some() {
                Some(self.else_statement.as_ref().unwrap().clone())
            } else {
                None
            },
        }
    }
}

impl IfStatement {
    pub fn new(
        if_token: SyntaxToken,
        open_parenthesis: SyntaxToken,
        expression: Box<dyn Expression>,
        close_parenthesis: SyntaxToken,
        statement_list: Box<dyn Statement>,
        else_statement: Option<Box<dyn Statement>>,
    ) -> Self {
        Self {
            if_token,
            open_parenthesis,
            expression,
            close_parenthesis,
            statement_list,
            else_statement,
        }
    }
}

impl Statement for IfStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_kind(&self) -> &SyntaxKind {
        &SyntaxKind::IfStatement
    }
}
