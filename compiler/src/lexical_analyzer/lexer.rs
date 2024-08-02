use std::cell::RefCell;
use std::rc::Rc;

use crate::reports::diagnostics::Diagnostics;
use crate::reports::text_place::TextPlace;
use crate::reports::text_span::TextSpan;
use crate::reports::text_type::TextType;
use crate::util::literals::LiteralValue;
use crate::util::{syntax_kind::SyntaxKind, syntax_token::SyntaxToken};

pub struct Lexer {
    text: String,
    position: usize,
    size: usize,
    diagnostics: Rc<RefCell<Diagnostics>>,
}

impl Lexer {
    pub fn in_memory_reader(text: &str, diagnostics: Rc<RefCell<Diagnostics>>) -> Self {
        Self {
            text: text.to_string(),
            position: 0,
            size: text.chars().count(),
            diagnostics,
        }
    }

    fn peek(&self, offset: usize) -> char {
        let index = self.position + offset;

        if index >= self.size {
            return '\0';
        }
        self.text.chars().nth(index).unwrap()
    }

    fn current(&self) -> char {
        self.peek(0)
    }

    fn lookahead(&self) -> char {
        self.peek(1)
    }

    fn next(&mut self) {
        self.position += 1;
    }

    fn create_operator(&mut self) -> SyntaxToken {
        let mut double_jump = false;
        let kind: SyntaxKind = if self.current() == '+' {
            SyntaxKind::Plus
        } else if self.current() == '-' {
            SyntaxKind::Minus
        } else if self.current() == '*' {
            SyntaxKind::Mul
        } else if self.current() == '/' {
            SyntaxKind::Div
        } else if self.current() == '&' && self.lookahead() == '&' {
            double_jump = true;
            SyntaxKind::AmpersandAmpersand
        } else if self.current() == '|' && self.lookahead() == '|' {
            double_jump = true;
            SyntaxKind::PipePipe
        } else if self.current() == '=' && self.lookahead() == '=' {
            double_jump = true;
            SyntaxKind::EqualsEquals
        } else if self.current() == '!' && self.lookahead() == '=' {
            double_jump = true;
            SyntaxKind::BangEquals
        } else if self.current() == '!' && self.lookahead() != '=' {
            SyntaxKind::Bang
        } else {
            self.diagnostics.borrow_mut().report_invalid_character(
                self.current(),
                self.position,
                TextPlace::Lexical,
                TextType::Error,
            );
            SyntaxKind::BadToken
        };
        let start = self.position;
        let mut value = String::from(self.current());
        self.next();
        if double_jump {
            value.push(self.current());
            self.next();
        }
        SyntaxToken::new(
            LiteralValue::String(value.clone()),
            start,
            kind,
            value.len(),
        )
    }

    fn create_number_token(&mut self) -> SyntaxToken {
        let start = self.position;

        while self.current().is_ascii_digit() {
            self.next();
        }
        let value: String = self
            .text
            .chars()
            .skip(start)
            .take(self.position - start)
            .collect();

        let result: Result<i32, _> = value.parse();

        match result {
            Ok(number) => SyntaxToken::new(
                LiteralValue::Integer(number),
                start,
                SyntaxKind::Number,
                value.len(),
            ),
            Err(_) => {
                self.diagnostics.borrow_mut().report_invalid_number(
                    value.clone(),
                    TextSpan::new(start, value.len()),
                    TextPlace::Lexical,
                    TextType::Error,
                );
                SyntaxToken::new(
                    LiteralValue::String(value.clone()),
                    start,
                    SyntaxKind::BadToken,
                    value.len(),
                )
            }
        }
    }

    fn create_white_space_token(&mut self) -> SyntaxToken {
        let start: usize = self.position;

        while self.current().is_whitespace() {
            self.next();
        }

        SyntaxToken::new(
            LiteralValue::String("WhiteSpace".to_string()),
            start,
            SyntaxKind::WhiteSpace,
            1,
        )
    }

    fn create_keyword_identifier(&mut self) -> SyntaxToken {
        let start: usize = self.position;

        while self.current().is_alphabetic() {
            self.next();
        }

        let value: String = self
            .text
            .chars()
            .skip(start)
            .take(self.position - start)
            .collect();

        let kind = SyntaxKind::get_keyword_kind(&value);
        if kind == SyntaxKind::True || kind == SyntaxKind::False {
            let bool_value: Result<bool, _> = value.parse();
            return SyntaxToken::new(
                LiteralValue::Boolean(bool_value.unwrap()),
                start,
                kind,
                value.len(),
            );
        }
        SyntaxToken::new(
            LiteralValue::String(value.to_string()),
            start,
            SyntaxKind::get_keyword_kind(&value),
            value.len(),
        )
    }

    fn create_parenthesis_token(&mut self, kind: SyntaxKind) -> SyntaxToken {
        let start: usize = self.position;
        let value = self.current().to_string();
        self.next();

        SyntaxToken::new(
            LiteralValue::String(value.clone()),
            start,
            kind,
            value.len(),
        )
    }

    pub fn next_token(&mut self) -> SyntaxToken {
        if self.current() == '\0' {
            return SyntaxToken::new(
                LiteralValue::String("EOF".to_string()),
                self.position,
                SyntaxKind::Eof,
                1,
            );
        }

        if self.current().is_ascii_digit() {
            return self.create_number_token();
        }

        if self.current().is_whitespace() {
            return self.create_white_space_token();
        }

        if self.current() == '(' || self.current() == ')' {
            return self.create_parenthesis_token(if self.current() == '(' {
                SyntaxKind::OpenParenthesis
            } else {
                SyntaxKind::CloseParenthesis
            });
        }

        if self.current().is_alphabetic() {
            return self.create_keyword_identifier();
        }

        self.create_operator()
    }
}
