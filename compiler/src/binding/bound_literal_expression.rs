use std::any::Any;

use crate::util::{
    literals::{LiteralType, LiteralValue},
    syntax_kind::SyntaxKind,
};

use super::{bound_expression::BoundExpression, bound_kind::BoundKind};

#[derive(Debug)]
pub struct BoundLiteralExpression {
    value: LiteralValue,
    type_of_value: LiteralType,
    kind: SyntaxKind,
}

impl Clone for BoundLiteralExpression {
    fn clone(&self) -> Self {
        BoundLiteralExpression {
            value: self.value.clone(),
            type_of_value: self.type_of_value.clone(),
            kind: self.kind.clone(),
        }
    }
}

impl BoundLiteralExpression {
    pub fn new(value: LiteralValue, type_of_value: LiteralType, kind: SyntaxKind) -> Self {
        Self {
            value,
            type_of_value,
            kind,
        }
    }

    pub fn get_value(&self) -> LiteralValue {
        self.value.clone()
    }

    pub fn get_kind(&self) -> &SyntaxKind {
        &self.kind
    }
}

impl BoundExpression for BoundLiteralExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type(&self) -> &LiteralType {
        &self.type_of_value
    }

    fn get_type_of_bound(&self) -> &BoundKind {
        &BoundKind::BoundLiteralExpression
    }
}
