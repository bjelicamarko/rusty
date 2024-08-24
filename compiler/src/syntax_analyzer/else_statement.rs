use std::any::Any;

use crate::util::{statement::Statement, syntax_kind::SyntaxKind, syntax_token::SyntaxToken};

#[derive(Debug)]
pub struct ElseStatement {
    else_token: SyntaxToken,
    else_statement: Box<dyn Statement>,
}

impl Clone for ElseStatement {
    fn clone(&self) -> Self {
        ElseStatement {
            else_token: self.else_token.clone(),
            else_statement: self.else_statement.clone(),
        }
    }
}

impl ElseStatement {
    pub fn new(else_token: SyntaxToken, else_statement: Box<dyn Statement>) -> Self {
        Self {
            else_token,
            else_statement,
        }
    }

    pub fn get_else_statement(&self) -> Box<dyn Statement> {
        self.else_statement.clone()
    }
}

impl Statement for ElseStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_kind(&self) -> &SyntaxKind {
        &SyntaxKind::ElseStatement
    }
}
