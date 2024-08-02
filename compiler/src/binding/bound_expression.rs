use std::{any::Any, fmt::Debug};

use crate::util::literals::LiteralType;

pub trait BoundExpressionClone: Debug {
    fn clone_box(&self) -> Box<dyn BoundExpression>;
}

impl<T> BoundExpressionClone for T
where
    T: 'static + BoundExpression + Clone,
{
    fn clone_box(&self) -> Box<dyn BoundExpression> {
        Box::new(self.clone())
    }
}

pub(crate) trait BoundExpression: BoundExpressionClone + Any {
    fn get_type(&self) -> &LiteralType;
    fn as_any(&self) -> &dyn Any;
}

impl Clone for Box<dyn BoundExpression> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
