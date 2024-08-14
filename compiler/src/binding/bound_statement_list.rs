use super::bound_statement::BoundStatement;

#[derive(Debug)]
pub struct BoundStatementList {
    statements: Vec<Box<dyn BoundStatement>>,
}

impl Clone for BoundStatementList {
    fn clone(&self) -> Self {
        BoundStatementList {
            statements: self.statements.clone(),
        }
    }
}

impl BoundStatementList {
    pub fn new(statements: Vec<Box<dyn BoundStatement>>) -> Self {
        Self { statements }
    }
}

impl BoundStatement for BoundStatementList {}
