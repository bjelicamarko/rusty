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
        let expression = self.parse_expression(0);
        self.equals(&[SyntaxKind::Eof]);
        expression
    }

    fn parse_expression(&mut self, parent_precedence: usize) -> Box<dyn Expression> {
        let mut _left: Box<dyn Expression> = Box::new(SyntaxToken::new(
            LiteralValue::String("None".to_string()),
            0,
            SyntaxKind::None,
            1,
        )) as Box<dyn Expression>;

        let unary_operator_precedence = self.current().kind().get_unary_operator_precendence();
        if unary_operator_precedence != 0 && unary_operator_precedence >= parent_precedence {
            let operator = self.next_token();
            let operand = self.parse_expression(unary_operator_precedence);

            _left = Box::new(UnaryExpressionSyntax::new(operator, operand)) as Box<dyn Expression>;
        } else {
            _left = self.parse_primary_expression();
        }

        loop {
            let precendence = self.current().kind().get_binary_operator_precendence();
            if precendence == 0 || precendence <= parent_precedence {
                break;
            }
            let operator_token = self.next_token();
            let right = self.parse_expression(precendence);

            _left = Box::new(BinaryExpressionSyntax::new(_left, operator_token, right))
                as Box<dyn Expression>;
        }

        _left
    }

    fn parse_primary_expression(&mut self) -> Box<dyn Expression> {
        if *self.current().get_kind() == SyntaxKind::OpenParenthesis {
            let open_parenthesis_token = self.next_token();
            let expr = self.parse_expression(0);
            let close_parenthesis_token = self.equals(&[SyntaxKind::CloseParenthesis]);

            Box::new(ParenthesizedExpressionSyntax::new(
                open_parenthesis_token,
                expr,
                close_parenthesis_token,
            )) as Box<dyn Expression>
        } else {
            let literal_token =
                self.equals(&[SyntaxKind::True, SyntaxKind::False, SyntaxKind::Number]);
            let value = literal_token.get_value();
            Box::new(LiteralExpressionSyntax::new(literal_token, value)) as Box<dyn Expression>
        }
    }
}
