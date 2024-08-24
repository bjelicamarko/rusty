use crate::util::{
    expression::Expression, statement::Statement, syntax_kind::SyntaxKind,
    syntax_token::SyntaxToken,
};

#[derive(Debug)]
pub struct WhileStatement {
    while_token: SyntaxToken,
    open_parenthesis: SyntaxToken,
    condition: Box<dyn Expression>,
    close_parenthesis: SyntaxToken,
    body: Box<dyn Statement>,
}

impl Clone for WhileStatement {
    fn clone(&self) -> Self {
        WhileStatement {
            while_token: self.while_token.clone(),
            open_parenthesis: self.open_parenthesis.clone(),
            condition: self.condition.clone(),
            close_parenthesis: self.close_parenthesis.clone(),
            body: self.body.clone(),
        }
    }
}

impl WhileStatement {
    pub fn new(
        while_token: SyntaxToken,
        open_parenthesis: SyntaxToken,
        condition: Box<dyn Expression>,
        close_parenthesis: SyntaxToken,
        body: Box<dyn Statement>,
    ) -> Self {
        Self {
            while_token,
            open_parenthesis,
            condition,
            close_parenthesis,
            body,
        }
    }

    pub fn get_condition(&self) -> Box<dyn Expression> {
        self.condition.clone()
    }

    pub fn get_body(&self) -> Box<dyn Statement> {
        self.body.clone()
    }
}

impl Statement for WhileStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_kind(&self) -> &SyntaxKind {
        &SyntaxKind::WhileStatement
    }
}
