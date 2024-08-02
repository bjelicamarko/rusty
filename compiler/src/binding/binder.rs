use std::{cell::RefCell, rc::Rc};

use crate::{
    reports::{
        diagnostics::Diagnostics, text_place::TextPlace, text_span::TextSpan, text_type::TextType,
    },
    syntax_analyzer::{
        binary_expression::BinaryExpressionSyntax, literal_expression::LiteralExpressionSyntax,
        unary_expression::UnaryExpressionSyntax,
    },
    util::{expression::Expression, syntax_kind::SyntaxKind},
};

use super::{
    bound_binary_expression::BoundBinaryExpression, bound_binary_operator::BoundBinaryOperator,
    bound_expression::BoundExpression, bound_literal_expression::BoundLiteralExpression,
    bound_unary_expression::BoundUnaryExpression, bound_unary_operator::BoundUnaryOperator,
};

pub struct Binder {
    diagnostics: Rc<RefCell<Diagnostics>>,
}

impl Binder {
    pub fn new(diagnostics: Rc<RefCell<Diagnostics>>) -> Self {
        Self { diagnostics }
    }

    pub fn bind_expression(&self, expression: Box<dyn Expression>) -> Box<dyn BoundExpression> {
        match *expression.get_kind() {
            SyntaxKind::LiteralExpression => self.bind_literal_expression(
                expression
                    .as_any()
                    .downcast_ref::<LiteralExpressionSyntax>()
                    .unwrap()
                    .clone(),
            ),
            SyntaxKind::UnaryExpression => self.bind_unary_expression(
                expression
                    .as_any()
                    .downcast_ref::<UnaryExpressionSyntax>()
                    .unwrap()
                    .clone(),
            ),
            SyntaxKind::BinaryExpression => self.bind_binary_expression(
                expression
                    .as_any()
                    .downcast_ref::<BinaryExpressionSyntax>()
                    .unwrap()
                    .clone(),
            ),
            _ => panic!("Binding ERROR: Unexpected syntax kind"),
        }
    }

    fn bind_literal_expression(
        &self,
        literal_expression: LiteralExpressionSyntax,
    ) -> Box<dyn BoundExpression> {
        Box::new(BoundLiteralExpression::new(literal_expression.get_value()))
            as Box<dyn BoundExpression>
    }

    fn bind_unary_expression(
        &self,
        unary_expression: UnaryExpressionSyntax,
    ) -> Box<dyn BoundExpression> {
        let bound_operand: Box<dyn BoundExpression> =
            self.bind_expression(unary_expression.operand());
        let bound_operator = BoundUnaryOperator::bind(
            *unary_expression.operator().kind(),
            *bound_operand.get_type(),
        );

        if let Some(operator) = bound_operator {
            Box::new(BoundUnaryExpression::new(operator, bound_operand)) as Box<dyn BoundExpression>
        } else {
            self.diagnostics
                .borrow_mut()
                .report_undefined_unary_operator(
                    *unary_expression.operator().kind(),
                    *bound_operand.get_type(),
                    TextSpan::new(
                        unary_expression.operator().position(),
                        unary_expression.operator().length(),
                    ),
                    TextPlace::Semantic,
                    TextType::Error,
                );
            bound_operand
        }
    }

    fn bind_binary_expression(
        &self,
        binary_expression: BinaryExpressionSyntax,
    ) -> Box<dyn BoundExpression> {
        let bound_left: Box<dyn BoundExpression> =
            self.bind_expression(binary_expression.get_left());
        let bound_right: Box<dyn BoundExpression> =
            self.bind_expression(binary_expression.get_right());
        let bound_operator: Option<BoundBinaryOperator> = BoundBinaryOperator::bind(
            *binary_expression.get_operator().kind(),
            *bound_left.get_type(),
            *bound_right.get_type(),
        );

        if let Some(operator) = bound_operator {
            Box::new(BoundBinaryExpression::new(
                bound_left,
                operator,
                bound_right,
            )) as Box<dyn BoundExpression>
        } else {
            self.diagnostics
                .borrow_mut()
                .report_undefined_binary_operator(
                    *binary_expression.get_operator().kind(),
                    *bound_left.get_type(),
                    *bound_right.get_type(),
                    TextSpan::new(
                        binary_expression.get_operator().position(),
                        binary_expression.get_operator().length(),
                    ),
                    TextPlace::Semantic,
                    TextType::Error,
                );
            bound_left
        }
    }
}
