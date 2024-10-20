use std::str::FromStr;

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
    OpenBrace,
    CloseBrace,
    Semicolon,
    Equals,
    Less,
    LessOrEquals,
    Greater,
    GreaterOrEquals,

    // keywords
    True,
    False,
    IdentifierToken,
    If,
    Else,
    Let,
    Const,
    While,
    For,
    To,

    // expressions
    LiteralExpression,
    BinaryExpression,
    ParenthesizedExpression,
    UnaryExpression,
    NameExpression,

    // statements
    StatementList,
    Assignment,
    IfStatement,
    ElseStatement,
    VariableDeclaration,
    ConstantDeclaration,
    WhileStatement,
    ForStatement,

    //special
    Variable,
    None,
    Error,
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
            "if" => SyntaxKind::If,
            "else" => SyntaxKind::Else,
            "let" => SyntaxKind::Let,
            "const" => SyntaxKind::Const,
            "while" => SyntaxKind::While,
            "for" => SyntaxKind::For,
            "to" => SyntaxKind::To,
            _ => SyntaxKind::IdentifierToken,
        }
    }
}

impl FromStr for SyntaxKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "eof" => Ok(SyntaxKind::Eof),
            "badtoken" => Ok(SyntaxKind::BadToken),
            "number" => Ok(SyntaxKind::Number),
            "whitespace" => Ok(SyntaxKind::WhiteSpace),
            "plus" => Ok(SyntaxKind::Plus),
            "minus" => Ok(SyntaxKind::Minus),
            "mul" => Ok(SyntaxKind::Mul),
            "div" => Ok(SyntaxKind::Div),
            "openparenthesis" => Ok(SyntaxKind::OpenParenthesis),
            "closeparenthesis" => Ok(SyntaxKind::CloseParenthesis),
            "bang" => Ok(SyntaxKind::Bang),
            "ampersandampersand" => Ok(SyntaxKind::AmpersandAmpersand),
            "pipepipe" => Ok(SyntaxKind::PipePipe),
            "equalsequals" => Ok(SyntaxKind::EqualsEquals),
            "bangequals" => Ok(SyntaxKind::BangEquals),
            "true" => Ok(SyntaxKind::True),
            "false" => Ok(SyntaxKind::False),
            "identifiertoken" => Ok(SyntaxKind::IdentifierToken),
            "literalexpression" => Ok(SyntaxKind::LiteralExpression),
            "binaryexpression" => Ok(SyntaxKind::BinaryExpression),
            "parenthesizedexpression" => Ok(SyntaxKind::ParenthesizedExpression),
            "unaryexpression" => Ok(SyntaxKind::UnaryExpression),
            "greaterorequals" => Ok(SyntaxKind::GreaterOrEquals),
            "greater" => Ok(SyntaxKind::Greater),
            "lessorequals" => Ok(SyntaxKind::LessOrEquals),
            "less" => Ok(SyntaxKind::Less),
            "for" => Ok(SyntaxKind::For),
            "to" => Ok(SyntaxKind::To),
            "(" => Ok(SyntaxKind::OpenParenthesis),
            ")" => Ok(SyntaxKind::CloseParenthesis),
            "=" => Ok(SyntaxKind::Equals),
            "while" => Ok(SyntaxKind::While),
            "const" => Ok(SyntaxKind::Const),
            ";" => Ok(SyntaxKind::Semicolon),
            "if" => Ok(SyntaxKind::If),
            "else" => Ok(SyntaxKind::Else),
            "openbrace" => Ok(SyntaxKind::OpenBrace),
            "closebrace" => Ok(SyntaxKind::CloseBrace),
            "{" => Ok(SyntaxKind::OpenBrace),
            "}" => Ok(SyntaxKind::CloseBrace),
            "let" => Ok(SyntaxKind::Let),
            "none" => Ok(SyntaxKind::None),
            _ => Err(format!("'{}' is not a valid SyntaxKind", s)),
        }
    }
}
