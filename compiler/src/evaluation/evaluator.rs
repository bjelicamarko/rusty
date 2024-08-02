use crate::{
    binding::{
        bound_binary_expression::BoundBinaryExpression,
        bound_binary_operator_kind::BoundBinaryOperatorKind, bound_expression::BoundExpression,
        bound_literal_expression::BoundLiteralExpression,
        bound_unary_expression::BoundUnaryExpression,
        bound_unary_operator_kind::BoundUnaryOperatorKind,
    },
    util::literals::{LiteralType, LiteralValue},
};

pub struct Evaluator {
    root: Box<dyn BoundExpression>,
}

impl Evaluator {
    pub fn new(root: Box<dyn BoundExpression>) -> Self {
        Self { root }
    }

    pub fn evaluate(&self) {
        let res = Self::evaluate_expression(self.root.clone());
        if *res.get_type() == LiteralType::Integer {
            println!("Result: {}", res.as_integer().unwrap());
        }

        if *res.get_type() == LiteralType::Boolean {
            println!("Result: {}", res.as_integer().unwrap());
        }
    }

    fn evaluate_expression(node: Box<dyn BoundExpression>) -> LiteralValue {
        if let Some(expr) = node.as_any().downcast_ref::<BoundLiteralExpression>() {
            return expr.get_value();
        }
        if let Some(expr) = node.as_any().downcast_ref::<BoundUnaryExpression>() {
            let operand = Self::evaluate_expression(expr.get_operand());

            match expr.get_operator().get_kind() {
                BoundUnaryOperatorKind::Identity => {
                    return LiteralValue::Integer(operand.as_integer().unwrap());
                }
                BoundUnaryOperatorKind::Negation => {
                    return LiteralValue::Integer(-operand.as_integer().unwrap());
                }
                BoundUnaryOperatorKind::LogicalNegation => {
                    return LiteralValue::Boolean(operand.as_boolean().unwrap());
                }
            }
        }
        if let Some(expr) = node.as_any().downcast_ref::<BoundBinaryExpression>() {
            let left = Self::evaluate_expression(expr.get_left());
            let right = Self::evaluate_expression(expr.get_right());

            match expr.get_operator().get_kind() {
                BoundBinaryOperatorKind::Addition => {
                    return LiteralValue::Integer(
                        left.as_integer().unwrap() + right.as_integer().unwrap(),
                    );
                }
                BoundBinaryOperatorKind::Subtraction => {
                    return LiteralValue::Integer(
                        left.as_integer().unwrap() - right.as_integer().unwrap(),
                    );
                }
                BoundBinaryOperatorKind::Multiplication => {
                    return LiteralValue::Integer(
                        left.as_integer().unwrap() * right.as_integer().unwrap(),
                    );
                }
                BoundBinaryOperatorKind::Division => {
                    return LiteralValue::Integer(
                        left.as_integer().unwrap() / right.as_integer().unwrap(),
                    );
                }
                BoundBinaryOperatorKind::LogicalAnd => {
                    return LiteralValue::Boolean(
                        left.as_boolean().unwrap() && right.as_boolean().unwrap(),
                    );
                }
                BoundBinaryOperatorKind::LogicalOr => {
                    return LiteralValue::Boolean(
                        left.as_boolean().unwrap() || right.as_boolean().unwrap(),
                    );
                }
                BoundBinaryOperatorKind::Equals => {
                    if *left.get_type() == LiteralType::Boolean {
                        return LiteralValue::Boolean(
                            left.as_boolean().unwrap() == right.as_boolean().unwrap(),
                        );
                    } else {
                        return LiteralValue::Boolean(
                            left.as_integer().unwrap() == right.as_integer().unwrap(),
                        );
                    }
                }
                BoundBinaryOperatorKind::NotEquals => {
                    if *left.get_type() == LiteralType::Boolean {
                        return LiteralValue::Boolean(
                            left.as_boolean().unwrap() != right.as_boolean().unwrap(),
                        );
                    } else {
                        return LiteralValue::Boolean(
                            left.as_integer().unwrap() != right.as_integer().unwrap(),
                        );
                    }
                }
            }
        }

        LiteralValue::Integer(-1)
    }
}
