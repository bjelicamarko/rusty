use std::cell::RefCell;
use std::rc::Rc;

use super::assignment::Assignment;
use super::binary_expression::BinaryExpressionSyntax;
use super::constant_declaration::ConstantDeclaration;
use super::else_statement::ElseStatement;
use super::if_statement::IfStatement;
use super::literal_expression::LiteralExpressionSyntax;
use super::name_expression::NameExpressionSyntax;
use super::parenthesized_expression::ParenthesizedExpressionSyntax;
use super::statement_list::StatementList;
use super::unary_expression::UnaryExpressionSyntax;
use super::variable_declaration::VariableDeclaration;
use crate::lexical_analyzer::lexer::Lexer;
use crate::reports::diagnostics::Diagnostics;
use crate::reports::text_place::TextPlace;
use crate::reports::text_span::TextSpan;
use crate::reports::text_type::TextType;
use crate::util::expression::Expression;
use crate::util::literals::LiteralValue;
use crate::util::statement::Statement;
use crate::util::syntax_kind::SyntaxKind;
use crate::util::syntax_token::SyntaxToken;

pub struct Parser {
    tokens: Vec<SyntaxToken>,
    position: usize,
    diagnostics: Rc<RefCell<Diagnostics>>,
}

impl Parser {
    pub fn new(diagnostics: Rc<RefCell<Diagnostics>>) -> Self {
        Self {
            tokens: Vec::new(),
            position: 0,
            diagnostics,
        }
    }

    pub fn create(&mut self, lexer: &mut Lexer) {
        loop {
            let token = lexer.next_token();

            self.diagnostics.borrow_mut().info_message(
                format!(
                "Creating token (kind: {:?}, position: {}, value: '{:?}', type of value: '{:?}')",
                token.kind(),
                token.position(),
                token.value(),
                token.get_type()
            ),
                TextSpan::new(token.position(), token.length()),
                TextPlace::Syntax,
                TextType::Info,
            );

            if *token.kind() != SyntaxKind::BadToken && *token.kind() != SyntaxKind::WhiteSpace {
                self.tokens.push(token.clone());
            }
            if *token.kind() == SyntaxKind::Eof {
                break;
            }
            if *token.kind() == SyntaxKind::BadToken {
                break;
            }
        }
    }

    fn current(&self) -> SyntaxToken {
        self.tokens[self.position].clone()
    }

    fn next_token(&mut self) -> SyntaxToken {
        let current = self.current();
        self.position += 1;
        current
    }

    fn equals(&mut self, kinds: &[SyntaxKind]) -> SyntaxToken {
        if kinds.contains(self.current().kind()) {
            return self.next_token();
        }

        self.diagnostics.borrow_mut().report_unexpected_token(
            TextSpan::new(self.current().position(), self.current().length()),
            *self.current().kind(),
            TextPlace::Syntax,
            TextType::Error,
        );

        SyntaxToken::new(
            "Error".to_string(),
            LiteralValue::String("Error".to_string()),
            self.current().position(),
            SyntaxKind::BadToken,
            1,
        )
    }

    pub fn parse(&mut self) -> Box<dyn Statement> {
        let program = self.parse_statement_list();
        self.equals(&[SyntaxKind::Eof]);
        println!("{:#?}", program);
        program
    }

    fn parse_statement(&mut self) -> Box<dyn Statement> {
        if *self.current().get_kind() == SyntaxKind::OpenBrace {
            return self.parse_statement_list();
        } else if *self.current().get_kind() == SyntaxKind::If {
            return self.parse_if_statement();
        } else if *self.current().get_kind() == SyntaxKind::Let {
            return self.parse_variable_declaration();
        } else if *self.current().get_kind() == SyntaxKind::Const {
            return self.parse_constant_declaration();
        } else {
            return self.parse_assignment();
        }
    }

    fn parse_if_statement(&mut self) -> Box<dyn Statement> {
        let if_token = self.equals(&[SyntaxKind::If]);
        let open_parenthesis = self.equals(&[SyntaxKind::OpenParenthesis]);
        let expression = self.parse_expression();
        let close_parenthesis = self.equals(&[SyntaxKind::CloseParenthesis]);
        let statement_list = self.parse_statement_list();
        let else_statement = self.parse_else_statement();

        Box::new(IfStatement::new(
            if_token,
            open_parenthesis,
            expression,
            close_parenthesis,
            statement_list,
            else_statement,
        )) as Box<dyn Statement>
    }

    fn parse_else_statement(&mut self) -> Option<Box<dyn Statement>> {
        if *self.current().get_kind() == SyntaxKind::Else {
            let else_token = self.equals(&[SyntaxKind::Else]);
            let statement_list = self.parse_statement_list();
            return Some(
                Box::new(ElseStatement::new(else_token, statement_list)) as Box<dyn Statement>
            );
        }

        None
    }

    fn parse_statement_list(&mut self) -> Box<dyn Statement> {
        let mut statements = Vec::new();

        let open_brace = self.equals(&[SyntaxKind::OpenBrace]);
        while *self.current().get_kind() != SyntaxKind::CloseBrace {
            statements.push(self.parse_statement());
        }
        let close_brace = self.equals(&[SyntaxKind::CloseBrace]);

        Box::new(StatementList::new(open_brace, statements, close_brace)) as Box<dyn Statement>
    }

    fn parse_assignment(&mut self) -> Box<dyn Statement> {
        let variable = self.equals(&[SyntaxKind::IdentifierToken]);
        let equals = self.equals(&[SyntaxKind::Equals]);
        let expression = self.parse_expression();
        let semi_colon = self.equals(&[SyntaxKind::Semicolon]);
        Box::new(Assignment::new(variable, equals, expression, semi_colon)) as Box<dyn Statement>
    }

    fn parse_variable_declaration(&mut self) -> Box<dyn Statement> {
        let let_token = self.equals(&[SyntaxKind::Let]);
        let variable = self.equals(&[SyntaxKind::IdentifierToken]);
        let equals = self.equals(&[SyntaxKind::Equals]);
        let expression = self.parse_expression();
        let semi_colon = self.equals(&[SyntaxKind::Semicolon]);
        Box::new(VariableDeclaration::new(
            let_token, variable, equals, expression, semi_colon,
        )) as Box<dyn Statement>
    }

    fn parse_constant_declaration(&mut self) -> Box<dyn Statement> {
        let const_token = self.equals(&[SyntaxKind::Const]);
        let variable = self.equals(&[SyntaxKind::IdentifierToken]);
        let equals = self.equals(&[SyntaxKind::Equals]);
        let expression = self.parse_expression();
        let semi_colon = self.equals(&[SyntaxKind::Semicolon]);
        Box::new(ConstantDeclaration::new(
            const_token,
            variable,
            equals,
            expression,
            semi_colon,
        )) as Box<dyn Statement>
    }

    fn parse_expression(&mut self) -> Box<dyn Expression> {
        let expression = self.parse_logical_expression();
        expression
    }

    fn parse_logical_expression(&mut self) -> Box<dyn Expression> {
        let mut expr = self.parse_relational_expression();

        while *self.current().get_kind() == SyntaxKind::PipePipe {
            let operator = self.next_token();
            let right = self.parse_relational_expression();
            expr =
                Box::new(BinaryExpressionSyntax::new(expr, operator, right)) as Box<dyn Expression>;
        }

        expr
    }

    fn parse_relational_expression(&mut self) -> Box<dyn Expression> {
        let mut expr = self.parse_arhithmetic_expression();

        while *self.current().get_kind() == SyntaxKind::AmpersandAmpersand
            || *self.current().get_kind() == SyntaxKind::EqualsEquals
            || *self.current().get_kind() == SyntaxKind::BangEquals
            || *self.current().get_kind() == SyntaxKind::Less
            || *self.current().get_kind() == SyntaxKind::LessOrEquals
            || *self.current().get_kind() == SyntaxKind::Greater
            || *self.current().get_kind() == SyntaxKind::GreaterOrEquals
        {
            let operator = self.next_token();
            let right = self.parse_arhithmetic_expression();
            expr =
                Box::new(BinaryExpressionSyntax::new(expr, operator, right)) as Box<dyn Expression>;
        }

        expr
    }

    fn parse_arhithmetic_expression(&mut self) -> Box<dyn Expression> {
        let mut expr = self.parse_term();

        while *self.current().get_kind() == SyntaxKind::Plus
            || *self.current().get_kind() == SyntaxKind::Minus
        {
            let operator = self.next_token();
            let right = self.parse_term();
            expr =
                Box::new(BinaryExpressionSyntax::new(expr, operator, right)) as Box<dyn Expression>;
        }

        expr
    }

    fn parse_term(&mut self) -> Box<dyn Expression> {
        let mut expr = self.parse_factor();

        while *self.current().get_kind() == SyntaxKind::Mul
            || *self.current().get_kind() == SyntaxKind::Div
        {
            let operator = self.next_token();
            let right = self.parse_factor();
            expr =
                Box::new(BinaryExpressionSyntax::new(expr, operator, right)) as Box<dyn Expression>;
        }

        expr
    }

    fn parse_factor(&mut self) -> Box<dyn Expression> {
        if *self.current().get_kind() == SyntaxKind::Bang
            || *self.current().get_kind() == SyntaxKind::Minus
        {
            let operator = self.next_token();
            let operand: Box<dyn Expression> = self.parse_factor();
            return Box::new(UnaryExpressionSyntax::new(operator, operand)) as Box<dyn Expression>;
        }

        if *self.current().get_kind() == SyntaxKind::OpenParenthesis {
            let open_parenthesis_token = self.next_token();
            let expr = self.parse_expression();
            let close_parenthesis_token = self.equals(&[SyntaxKind::CloseParenthesis]);

            return Box::new(ParenthesizedExpressionSyntax::new(
                open_parenthesis_token,
                expr,
                close_parenthesis_token,
            )) as Box<dyn Expression>;
        }

        self.parse_literal()
    }

    fn parse_literal(&mut self) -> Box<dyn Expression> {
        let literal_token = self.equals(&[
            SyntaxKind::True,
            SyntaxKind::False,
            SyntaxKind::Number,
            SyntaxKind::IdentifierToken,
        ]);
        if *literal_token.get_kind() == SyntaxKind::IdentifierToken {
            return Box::new(NameExpressionSyntax::new(literal_token)) as Box<dyn Expression>;
        }
        Box::new(LiteralExpressionSyntax::new(literal_token)) as Box<dyn Expression>
    }
}
