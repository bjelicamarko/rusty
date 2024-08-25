use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::util::literals::LiteralValue;
use crate::util::variable_symbol::VariableSymbol;

lazy_static! {
    pub static ref SYMBOL_TABLE: Mutex<HashMap<VariableSymbol, Option<LiteralValue>>> =
        Mutex::new(HashMap::new());
}

pub fn insert_into_symbol_table(variable: &VariableSymbol, value: Option<LiteralValue>) {
    SYMBOL_TABLE.lock().unwrap().insert(variable.clone(), value);
}
