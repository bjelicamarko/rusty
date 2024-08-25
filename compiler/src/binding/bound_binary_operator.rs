use crate::util::{literals::LiteralType, syntax_kind::SyntaxKind};

use super::bound_binary_operator_kind::BoundBinaryOperatorKind;

#[derive(Clone, Debug)]
pub struct BoundBinaryOperator {
    syntax_kind: SyntaxKind,
    kind: BoundBinaryOperatorKind,
    left_type: LiteralType,
    right_type: LiteralType,
    result_type: LiteralType,
}

impl BoundBinaryOperator {
    pub fn get_type(&self) -> &LiteralType {
        &self.result_type
    }

    pub fn get_kind(&self) -> &BoundBinaryOperatorKind {
        &self.kind
    }

    fn new(
        syntax_kind: SyntaxKind,
        kind: BoundBinaryOperatorKind,
        left_type: LiteralType,
        right_type: LiteralType,
        result_type: LiteralType,
    ) -> Self {
        Self {
            syntax_kind,
            kind,
            left_type,
            right_type,
            result_type,
        }
    }

    fn build_unary(
        syntax_kind: SyntaxKind,
        kind: BoundBinaryOperatorKind,
        operand_type: LiteralType,
        result_type: LiteralType,
    ) -> Self {
        BoundBinaryOperator::new(syntax_kind, kind, operand_type, operand_type, result_type)
    }

    fn build_binary(
        syntax_kind: SyntaxKind,
        kind: BoundBinaryOperatorKind,
        result_type: LiteralType,
    ) -> Self {
        BoundBinaryOperator::new(syntax_kind, kind, result_type, result_type, result_type)
    }

    pub fn bind(
        syntax_kind: SyntaxKind,
        left_type: LiteralType,
        right_type: LiteralType,
    ) -> Option<Self> {
        let operators = [
            BoundBinaryOperator::build_binary(
                SyntaxKind::Plus,
                BoundBinaryOperatorKind::Addition,
                LiteralType::Integer,
            ),
            BoundBinaryOperator::build_binary(
                SyntaxKind::Minus,
                BoundBinaryOperatorKind::Subtraction,
                LiteralType::Integer,
            ),
            BoundBinaryOperator::build_binary(
                SyntaxKind::Mul,
                BoundBinaryOperatorKind::Multiplication,
                LiteralType::Integer,
            ),
            BoundBinaryOperator::build_binary(
                SyntaxKind::Div,
                BoundBinaryOperatorKind::Division,
                LiteralType::Integer,
            ),
            BoundBinaryOperator::build_unary(
                SyntaxKind::EqualsEquals,
                BoundBinaryOperatorKind::Equals,
                LiteralType::Integer,
                LiteralType::Boolean,
            ),
            BoundBinaryOperator::build_unary(
                SyntaxKind::BangEquals,
                BoundBinaryOperatorKind::NotEquals,
                LiteralType::Integer,
                LiteralType::Boolean,
            ),
            BoundBinaryOperator::build_binary(
                SyntaxKind::AmpersandAmpersand,
                BoundBinaryOperatorKind::LogicalAnd,
                LiteralType::Boolean,
            ),
            BoundBinaryOperator::build_binary(
                SyntaxKind::PipePipe,
                BoundBinaryOperatorKind::LogicalOr,
                LiteralType::Boolean,
            ),
            BoundBinaryOperator::build_binary(
                SyntaxKind::EqualsEquals,
                BoundBinaryOperatorKind::Equals,
                LiteralType::Boolean,
            ),
            BoundBinaryOperator::build_binary(
                SyntaxKind::BangEquals,
                BoundBinaryOperatorKind::NotEquals,
                LiteralType::Boolean,
            ),
            BoundBinaryOperator::build_unary(
                SyntaxKind::Less,
                BoundBinaryOperatorKind::Less,
                LiteralType::Integer,
                LiteralType::Boolean,
            ),
            BoundBinaryOperator::build_unary(
                SyntaxKind::LessOrEquals,
                BoundBinaryOperatorKind::LessOrEquals,
                LiteralType::Integer,
                LiteralType::Boolean,
            ),
            BoundBinaryOperator::build_unary(
                SyntaxKind::Greater,
                BoundBinaryOperatorKind::Greater,
                LiteralType::Integer,
                LiteralType::Boolean,
            ),
            BoundBinaryOperator::build_unary(
                SyntaxKind::GreaterOrEquals,
                BoundBinaryOperatorKind::GreaterOrEquals,
                LiteralType::Integer,
                LiteralType::Boolean,
            ),
        ];

        for operator in &operators {
            if operator.syntax_kind == syntax_kind
                && operator.right_type == right_type
                && operator.left_type == left_type
            {
                return Some(operator.clone());
            }
        }
        None
    }
}
