use std::any::Any;

use crate::util::{
    expression::Expression, statement::Statement, syntax_kind::SyntaxKind,
    syntax_token::SyntaxToken,
};

#[derive(Debug)]
pub struct IfStatement {
    if_token: SyntaxToken,
    open_parenthesis: SyntaxToken,
    condition: Box<dyn Expression>,
    close_parenthesis: SyntaxToken,
    then_statement: Box<dyn Statement>,
    else_clause: Option<Box<dyn Statement>>,
}

impl Clone for IfStatement {
    fn clone(&self) -> Self {
        IfStatement {
            if_token: self.if_token.clone(),
            open_parenthesis: self.open_parenthesis.clone(),
            condition: self.condition.clone(),
            close_parenthesis: self.close_parenthesis.clone(),
            then_statement: self.then_statement.clone(),
            else_clause: self.else_clause.clone(),
        }
    }
}

impl IfStatement {
    pub fn new(
        if_token: SyntaxToken,
        open_parenthesis: SyntaxToken,
        condition: Box<dyn Expression>,
        close_parenthesis: SyntaxToken,
        then_statement: Box<dyn Statement>,
        else_clause: Option<Box<dyn Statement>>,
    ) -> Self {
        Self {
            if_token,
            open_parenthesis,
            condition,
            close_parenthesis,
            then_statement,
            else_clause,
        }
    }

    pub fn get_condition(&self) -> Box<dyn Expression> {
        self.condition.clone()
    }

    pub fn get_then_statement(&self) -> Box<dyn Statement> {
        self.then_statement.clone()
    }

    pub fn get_else_clause(&self) -> Option<Box<dyn Statement>> {
        self.else_clause.clone()
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
