use std::any::Any;

use crate::util::{statement::Statement, syntax_kind::SyntaxKind, syntax_token::SyntaxToken};

#[derive(Debug)]
pub struct StatementList {
    open_brace: SyntaxToken,
    statements: Vec<Box<dyn Statement>>,
    close_brace: SyntaxToken,
}

impl Clone for StatementList {
    fn clone(&self) -> Self {
        StatementList {
            open_brace: self.open_brace.clone(),
            statements: self.statements.clone(),
            close_brace: self.close_brace.clone(),
        }
    }
}

impl StatementList {
    pub fn new(
        open_brace: SyntaxToken,
        statements: Vec<Box<dyn Statement>>,
        close_brace: SyntaxToken,
    ) -> Self {
        Self {
            open_brace,
            statements,
            close_brace,
        }
    }

    pub fn get_statements(&self) -> Vec<Box<dyn Statement>> {
        self.statements.clone()
    }
}

impl Statement for StatementList {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_kind(&self) -> &SyntaxKind {
        &SyntaxKind::StatementList
    }
}
