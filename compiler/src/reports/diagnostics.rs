use crate::util::{literals::LiteralType, syntax_kind::SyntaxKind};

use super::{
    diagnostic::Diagnostic, text_place::TextPlace, text_span::TextSpan, text_type::TextType,
};

pub struct Diagnostics {
    diagnostics: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn print(&self) {
        for diagnostic in &self.diagnostics {
            println!("{:?}", diagnostic)
        }
    }

    fn report(&mut self, message: String, span: TextSpan, place: TextPlace, kind: TextType) {
        self.diagnostics
            .push(Diagnostic::new(message, span, place, kind))
    }

    pub fn info_message(
        &mut self,
        message: String,
        span: TextSpan,
        place: TextPlace,
        kind: TextType,
    ) {
        self.report(message, span, place, kind);
    }

    pub fn report_invalid_number(
        &mut self,
        text: String,
        span: TextSpan,
        place: TextPlace,
        kind: TextType,
    ) {
        self.report(
            format!("The number {} isn't valid.", text),
            span,
            place,
            kind,
        );
    }

    pub fn report_invalid_character(
        &mut self,
        ch: char,
        position: usize,
        place: TextPlace,
        kind: TextType,
    ) {
        self.report(
            format!("The character {} isn't valid.", ch),
            TextSpan::new(position, 1),
            place,
            kind,
        );
    }

    pub fn report_unexpected_token(
        &mut self,
        span: TextSpan,
        token_kind: SyntaxKind,
        place: TextPlace,
        kind: TextType,
    ) {
        self.report(
            format!("Unexpected token {:?}.", token_kind),
            span,
            place,
            kind,
        );
    }

    pub fn report_undefined_binary_operator(
        &mut self,
        operator: SyntaxKind,
        left: LiteralType,
        right: LiteralType,
        span: TextSpan,
        place: TextPlace,
        kind: TextType,
    ) {
        self.report(
            format!(
                "Binary operator {:?} is not defined for types {:?} and {:?}.",
                operator, left, right
            ),
            span,
            place,
            kind,
        );
    }

    pub fn report_undefined_unary_operator(
        &mut self,
        operator: SyntaxKind,
        operand_type: LiteralType,
        span: TextSpan,
        place: TextPlace,
        kind: TextType,
    ) {
        self.report(
            format!(
                "Unary operator {:?} is not defined for type {:?}.",
                operator, operand_type
            ),
            span,
            place,
            kind,
        )
    }
}
