#[derive(Debug)]
pub enum BoundKind {
    // statements
    BoundStatementList,
    BoundAssignment,
    BoundVariableDeclaration,
    BoundConstantDeclaration,
    BoundIfStatement,
    BoundWhileStatement,
    BoundForStatement,

    // expressions
    BoundLiteralExpression,
    BoundUnaryExpression,
    BoundBinaryExpression,
}
