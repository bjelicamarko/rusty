#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SyntaxKind {
    // tokens
    Eof,
    BadToken,
    Number,
    WhiteSpace,
    Plus,
    Minus,
    Mul,
    Div,
    OpenParenthesis,
    CloseParenthesis,
    Bang,
    AmpersandAmpersand,
    PipePipe,
    EqualsEquals,
    BangEquals,

    // keywords
    True,
    False,
    IdentifierToken,

    // expressions
    LiteralExpression,
    BinaryExpression,
    ParenthesizedExpression,
    UnaryExpression,

    // special
    None,
}

impl SyntaxKind {
    pub fn get_binary_operator_precendence(&self) -> usize {
        match *self {
            SyntaxKind::Div | SyntaxKind::Mul => 5,
            SyntaxKind::Plus | SyntaxKind::Minus => 4,
            SyntaxKind::EqualsEquals | SyntaxKind::BangEquals => 3,
            SyntaxKind::AmpersandAmpersand => 2,
            SyntaxKind::PipePipe => 1,
            _ => 0,
        }
    }

    pub fn get_unary_operator_precendence(&self) -> usize {
        match *self {
            SyntaxKind::Plus | SyntaxKind::Minus | SyntaxKind::Bang => 6,
            _ => 0,
        }
    }

    pub fn get_keyword_kind(text: &str) -> SyntaxKind {
        match text {
            "true" => SyntaxKind::True,
            "false" => SyntaxKind::False,
            _ => SyntaxKind::IdentifierToken,
        }
    }
}
