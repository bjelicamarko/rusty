use std::{cell::RefCell, rc::Rc};

use crate::util::variable_symbol::VariableSymbol;

#[derive(Clone)]
pub struct BoundScope {
    pub variables: Vec<VariableSymbol>,
    parent: Option<Rc<RefCell<BoundScope>>>,
}

impl BoundScope {
    pub fn new(parent: Option<Rc<RefCell<BoundScope>>>) -> Self {
        Self {
            variables: Vec::new(),
            parent,
        }
    }

    pub fn get_parent(&mut self) -> Option<Rc<RefCell<BoundScope>>> {
        self.parent.clone()
    }
}
