use std::any::Any;

use crate::util::{statement::Statement, syntax_kind::SyntaxKind, syntax_token::SyntaxToken};

#[derive(Debug)]
pub struct ElseStatement {
    else_token: SyntaxToken,
    statement_list: Box<dyn Statement>,
}

impl Clone for ElseStatement {
    fn clone(&self) -> Self {
        ElseStatement {
            else_token: self.else_token.clone(),
            statement_list: self.statement_list.clone(),
        }
    }
}

impl ElseStatement {
    pub fn new(else_token: SyntaxToken, statement_list: Box<dyn Statement>) -> Self {
        Self {
            else_token,
            statement_list,
        }
    }

    pub fn get_statement_list(&self) -> Box<dyn Statement> {
        self.statement_list.clone()
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
