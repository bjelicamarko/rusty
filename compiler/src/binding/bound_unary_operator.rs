use crate::util::{literals::LiteralType, syntax_kind::SyntaxKind};

use super::bound_unary_operator_kind::BoundUnaryOperatorKind;

#[derive(Clone, Debug)]
pub struct BoundUnaryOperator {
    syntax_kind: SyntaxKind,
    kind: BoundUnaryOperatorKind,
    operand_type: LiteralType,
    result_type: LiteralType,
}

impl BoundUnaryOperator {
    pub fn get_type(&self) -> &LiteralType {
        &self.result_type
    }

    pub fn get_kind(&self) -> BoundUnaryOperatorKind {
        self.kind.clone()
    }

    fn new(
        syntax_kind: SyntaxKind,
        kind: BoundUnaryOperatorKind,
        operand_type: LiteralType,
        result_type: LiteralType,
    ) -> Self {
        Self {
            syntax_kind,
            kind,
            operand_type,
            result_type,
        }
    }

    fn build_unary(
        syntax_kind: SyntaxKind,
        kind: BoundUnaryOperatorKind,
        operand_type: LiteralType,
    ) -> Self {
        BoundUnaryOperator::new(syntax_kind, kind, operand_type, operand_type)
    }

    pub fn bind(syntax_kind: SyntaxKind, operand_type: LiteralType) -> Option<Self> {
        let operators = [
            BoundUnaryOperator::build_unary(
                SyntaxKind::Bang,
                BoundUnaryOperatorKind::LogicalNegation,
                LiteralType::Boolean,
            ),
            BoundUnaryOperator::build_unary(
                SyntaxKind::Plus,
                BoundUnaryOperatorKind::Identity,
                LiteralType::Integer,
            ),
            BoundUnaryOperator::build_unary(
                SyntaxKind::Minus,
                BoundUnaryOperatorKind::Negation,
                LiteralType::Integer,
            ),
        ];

        for operator in &operators {
            if operator.syntax_kind == syntax_kind && operator.operand_type == operand_type {
                return Some(operator.clone());
            }
        }
        None
    }
}
