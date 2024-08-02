use std::{any::Any, fmt::Debug};

use super::{
    literals::{LiteralType, LiteralValue},
    syntax_kind::SyntaxKind,
};

pub trait ExpressionClone: Debug {
    fn clone_box(&self) -> Box<dyn Expression>;
}

impl<T> ExpressionClone for T
where
    T: 'static + Expression + Clone,
{
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

pub(crate) trait Expression: ExpressionClone + Any {
    fn as_any(&self) -> &dyn Any;
    fn get_kind(&self) -> &SyntaxKind;
    fn get_children(&self) -> Vec<Box<dyn Expression>>;
    fn get_value(&self) -> LiteralValue;
    fn get_type(&self) -> &LiteralType;
}

impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
