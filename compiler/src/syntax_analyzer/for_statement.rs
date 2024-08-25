use std::any::Any;

use crate::util::{
    expression::Expression, statement::Statement, syntax_kind::SyntaxKind,
    syntax_token::SyntaxToken,
};

#[derive(Debug)]
pub struct ForStatement {
    for_token: SyntaxToken,
    open_parenthesis: SyntaxToken,
    identifier: SyntaxToken,
    equals: SyntaxToken,
    lower_bound: Box<dyn Expression>,
    to_token: SyntaxToken,
    upper_bound: Box<dyn Expression>,
    close_parenthesis: SyntaxToken,
    body: Box<dyn Statement>,
}

impl Clone for ForStatement {
    fn clone(&self) -> Self {
        ForStatement {
            for_token: self.for_token.clone(),
            open_parenthesis: self.open_parenthesis.clone(),
            identifier: self.identifier.clone(),
            equals: self.equals.clone(),
            lower_bound: self.lower_bound.clone(),
            to_token: self.to_token.clone(),
            upper_bound: self.upper_bound.clone(),
            close_parenthesis: self.close_parenthesis.clone(),
            body: self.body.clone(),
        }
    }
}

impl ForStatement {
    pub fn new(
        for_token: SyntaxToken,
        open_parenthesis: SyntaxToken,
        identifier: SyntaxToken,
        equals: SyntaxToken,
        lower_bound: Box<dyn Expression>,
        to_token: SyntaxToken,
        upper_bound: Box<dyn Expression>,
        close_parenthesis: SyntaxToken,
        body: Box<dyn Statement>,
    ) -> Self {
        Self {
            for_token,
            open_parenthesis,
            identifier,
            equals,
            lower_bound,
            to_token,
            upper_bound,
            close_parenthesis,
            body,
        }
    }

    pub fn get_identifier(&self) -> SyntaxToken {
        self.identifier.clone()
    }

    pub fn get_lower_bound(&self) -> Box<dyn Expression> {
        self.lower_bound.clone()
    }

    pub fn get_upper_bound(&self) -> Box<dyn Expression> {
        self.upper_bound.clone()
    }

    pub fn get_body(&self) -> Box<dyn Statement> {
        self.body.clone()
    }
}

impl Statement for ForStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_kind(&self) -> &SyntaxKind {
        &SyntaxKind::ForStatement
    }
}
