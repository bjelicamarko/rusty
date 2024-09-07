use std::fmt::{Debug, Formatter, Result};

use rocket::serde::{Deserialize, Serialize};

use super::{text_place::TextPlace, text_span::TextSpan, text_type::TextType};

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
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

    pub fn get_type(&self) -> &TextType {
        &self.kind
    }

    pub fn get_place(&self) -> &TextPlace {
        &self.place
    }

    pub fn get_message(&self) -> &str {
        &self.message
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
