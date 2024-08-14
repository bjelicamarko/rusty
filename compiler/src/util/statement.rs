use std::{any::Any, fmt::Debug};

use super::syntax_kind::SyntaxKind;

pub trait StatementClone: Debug {
    fn clone_box(&self) -> Box<dyn Statement>;
}

impl<T> StatementClone for T
where
    T: 'static + Statement + Clone,
{
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

pub(crate) trait Statement: StatementClone + Any {
    fn as_any(&self) -> &dyn Any;
    fn get_kind(&self) -> &SyntaxKind;
}

impl Clone for Box<dyn Statement> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
