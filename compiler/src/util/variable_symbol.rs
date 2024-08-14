use super::literals::LiteralType;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct VariableSymbol {
    id: String,
    kind: LiteralType,
}

impl VariableSymbol {
    pub fn new(id: String, kind: LiteralType) -> Self {
        Self { id, kind }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_type(&self) -> LiteralType {
        self.kind.clone()
    }
}
