use std::{any::Any, fmt::Debug};

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
}

impl Clone for Box<dyn BoundStatement> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
