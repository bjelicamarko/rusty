use super::literals::LiteralType;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct VariableSymbol {
    id: String,
    kind: LiteralType,
    read_only: bool,
}

impl VariableSymbol {
    pub fn new(id: String, kind: LiteralType, read_only: bool) -> Self {
        Self {
            id,
            kind,
            read_only,
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
}
