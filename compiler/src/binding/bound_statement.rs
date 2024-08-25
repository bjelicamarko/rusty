use std::{any::Any, fmt::Debug};

use super::bound_kind::BoundKind;

pub trait BoundStatementClone: Debug {
    fn clone_box(&self) -> Box<dyn BoundStatement>;
}

impl<T> BoundStatementClone for T
where
    T: 'static + BoundStatement + Clone,
{
    fn clone_box(&self) -> Box<dyn BoundStatement> {
        Box::new(self.clone())
    }
}

pub(crate) trait BoundStatement: BoundStatementClone + Any {
    fn as_any(&self) -> &dyn Any;
    fn get_type_of_bound(&self) -> &BoundKind;
}

impl Clone for Box<dyn BoundStatement> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
