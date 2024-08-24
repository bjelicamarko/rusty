use super::literals::LiteralType;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct VariableSymbol {
    id: String,
    kind: LiteralType,
    read_only: bool,
    is_global: bool,
}

impl PartialEq for VariableSymbol {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for VariableSymbol {}

impl Hash for VariableSymbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl VariableSymbol {
    pub fn new(id: String, kind: LiteralType, read_only: bool, is_global: bool) -> Self {
        Self {
            id,
            kind,
            read_only,
            is_global,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_type(&self) -> LiteralType {
        self.kind.clone()
    }

    pub fn is_read_only(&self) -> bool {
        self.read_only.clone()
    }

    pub fn is_global(&self) -> bool {
        self.is_global.clone()
    }
}
