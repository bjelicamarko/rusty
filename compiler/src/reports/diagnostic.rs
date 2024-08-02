use std::fmt::{Debug, Formatter, Result};

use super::{text_place::TextPlace, text_span::TextSpan, text_type::TextType};

pub struct Diagnostic {
    message: String,
    span: TextSpan,
    place: TextPlace,
    kind: TextType,
}

impl Diagnostic {
    pub fn new(message: String, span: TextSpan, place: TextPlace, kind: TextType) -> Self {
        Self {
            message,
            span,
            place,
            kind,
        }
    }
}

impl Debug for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{:?}-{:?}-Position:{:?}: {:?}",
            self.kind, self.place, self.span, self.message
        )
    }
}
