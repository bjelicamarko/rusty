use std::any::Any;

use super::{
    expression::Expression,
    literals::{LiteralType, LiteralValue},
    syntax_kind::SyntaxKind,
};

#[derive(Debug)]
pub struct SyntaxToken {
    value: LiteralValue,
    position: usize,
    kind: SyntaxKind,
    length: usize,
}

impl SyntaxToken {
    pub fn new(value: LiteralValue, position: usize, kind: SyntaxKind, length: usize) -> Self {
        Self {
            value,
            position,
            kind,
            length,
        }
    }

    pub fn value(&self) -> &LiteralValue {
        &self.value
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn kind(&self) -> &SyntaxKind {
        &self.kind
    }
}

impl Clone for SyntaxToken {
    fn clone(&self) -> Self {
        SyntaxToken {
            value: self.value.clone(),
            position: self.position,
            kind: self.kind,
            length: self.length,
        }
    }
}

impl Expression for SyntaxToken {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_kind(&self) -> &SyntaxKind {
        &self.kind
    }

    fn get_children(&self) -> Vec<Box<dyn Expression>> {
        Vec::new()
    }

    fn get_value(&self) -> LiteralValue {
        self.value.clone()
    }

    fn get_type(&self) -> &LiteralType {
        self.value.get_type()
    }
}
