use std::cell::RefCell;
use std::rc::Rc;

use super::binary_expression::BinaryExpressionSyntax;
use super::literal_expression::LiteralExpressionSyntax;
use super::parenthesized_expression::ParenthesizedExpressionSyntax;
use super::unary_expression::UnaryExpressionSyntax;
use crate::lexical_analyzer::lexer::Lexer;
use crate::reports::diagnostics::Diagnostics;
use crate::reports::text_place::TextPlace;
use crate::reports::text_span::TextSpan;
use crate::reports::text_type::TextType;
use crate::util::expression::Expression;
use crate::util::literals::LiteralValue;
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
            LiteralValue::String("Error".to_string()),
            self.current().position(),
            SyntaxKind::BadToken,
            1,
        )
    }

    pub fn parse(&mut self) -> Box<dyn Expression> {
        let expression = self.parse_logical_expression();
        self.equals(&[SyntaxKind::Eof]);
        expression
    }

    pub fn parse_expression(&mut self) -> Box<dyn Expression> {
        let expression = self.parse_logical_expression();
        expression
    }

    fn parse_logical_expression(&mut self) -> Box<dyn Expression> {
        let mut expr = self.parse_relational_expression();

        while *self.current().get_kind() == SyntaxKind::PipePipe {
            let operator = self.next_token();
            let right = self.parse_relational_expression();
            expr = Box::new(BinaryExpressionSyntax::new(
                expr,
                operator,
                right,
                SyntaxKind::None,
            )) as Box<dyn Expression>;
        }

        expr
    }

    fn parse_relational_expression(&mut self) -> Box<dyn Expression> {
        let mut expr = self.parse_arhithmetic_expression();

        while *self.current().get_kind() == SyntaxKind::AmpersandAmpersand
            || *self.current().get_kind() == SyntaxKind::EqualsEquals
            || *self.current().get_kind() == SyntaxKind::BangEquals
        {
            let operator = self.next_token();
            let right = self.parse_arhithmetic_expression();
            expr = Box::new(BinaryExpressionSyntax::new(
                expr,
                operator,
                right,
                SyntaxKind::None,
            )) as Box<dyn Expression>;
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
            expr = Box::new(BinaryExpressionSyntax::new(
                expr,
                operator,
                right,
                SyntaxKind::None,
            )) as Box<dyn Expression>;
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
            expr = Box::new(BinaryExpressionSyntax::new(
                expr,
                operator,
                right,
                SyntaxKind::None,
            )) as Box<dyn Expression>;
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
        let literal_token = self.equals(&[SyntaxKind::True, SyntaxKind::False, SyntaxKind::Number]);
        let value = literal_token.get_value();
        Box::new(LiteralExpressionSyntax::new(literal_token, value)) as Box<dyn Expression>
    }
}
