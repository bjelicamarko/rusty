use crate::global_state::SYMBOL_TABLE;
use crate::syntax_analyzer::name_expression::NameExpressionSyntax;
use crate::{
    reports::{
        diagnostics::Diagnostics, text_place::TextPlace, text_span::TextSpan, text_type::TextType,
    },
    syntax_analyzer::{
        assignment::Assignment, binary_expression::BinaryExpressionSyntax,
        literal_expression::LiteralExpressionSyntax,
        parenthesized_expression::ParenthesizedExpressionSyntax, statement_list::StatementList,
        unary_expression::UnaryExpressionSyntax,
    },
    util::{
        expression::Expression, statement::Statement, syntax_kind::SyntaxKind,
        variable_symbol::VariableSymbol,
    },
};

use std::{cell::RefCell, rc::Rc};

use super::bound_scope::BoundScope;
use super::{
    bound_assignment::BoundAssignment, bound_binary_expression::BoundBinaryExpression,
    bound_binary_operator::BoundBinaryOperator, bound_expression::BoundExpression,
    bound_literal_expression::BoundLiteralExpression, bound_statement::BoundStatement,
    bound_statement_list::BoundStatementList, bound_unary_expression::BoundUnaryExpression,
    bound_unary_operator::BoundUnaryOperator,
};

pub struct Binder {
    diagnostics: Rc<RefCell<Diagnostics>>,
    scope: BoundScope,
}

impl Binder {
    pub fn new(diagnostics: Rc<RefCell<Diagnostics>>) -> Self {
        Self {
            diagnostics,
            scope: BoundScope::new(None),
        }
    }

    pub fn bind_statement(&mut self, statement: Box<dyn Statement>) -> Box<dyn BoundStatement> {
        match *statement.get_kind() {
            SyntaxKind::StatementList => self.bind_statement_list(
                statement
                    .as_any()
                    .downcast_ref::<StatementList>()
                    .unwrap()
                    .clone(),
            ),
            SyntaxKind::Assignment => self.bind_assignment(
                statement
                    .as_any()
                    .downcast_ref::<Assignment>()
                    .unwrap()
                    .clone(),
            ),
            _ => panic!("Binding ERROR: Unexpected syntax kind"),
        }
    }

    fn bind_statement_list(&mut self, statement_list: StatementList) -> Box<dyn BoundStatement> {
        self.scope = BoundScope::new(Some(Rc::new(RefCell::new(self.scope.clone()))));

        let mut statements: Vec<Box<dyn BoundStatement>> = Vec::new();

        for statement in statement_list.get_statements() {
            let bound_statement = self.bind_statement(statement);
            statements.push(bound_statement);
        }

        self.scope = self
            .scope
            .get_parent()
            .as_mut()
            .unwrap()
            .borrow()
            .to_owned();

        Box::new(BoundStatementList::new(statements)) as Box<dyn BoundStatement>
    }

    fn bind_assignment(&mut self, assignment: Assignment) -> Box<dyn BoundStatement> {
        let name = assignment.get_variable().name();
        let expr = self.bind_expression(assignment.get_expression());
        let variable_symbol = VariableSymbol::new(name, *expr.get_type());

        SYMBOL_TABLE
            .lock()
            .unwrap()
            .insert(variable_symbol.clone(), None);

        self.scope.variables.push(variable_symbol.clone());

        Box::new(BoundAssignment::new(variable_symbol, expr))
    }

    fn check_scope_of_variable(&self, v: &VariableSymbol) -> bool {
        let mut local_scope = self.scope.clone();

        loop {
            for variable in &local_scope.variables {
                if variable.id() == v.id() {
                    return true;
                }
            }
            if local_scope.get_parent().is_none() {
                break;
            }
            local_scope = local_scope.get_parent().unwrap().borrow().to_owned()
        }
        return false;
    }

    fn bind_expression(&self, expression: Box<dyn Expression>) -> Box<dyn BoundExpression> {
        match *expression.get_kind() {
            SyntaxKind::NameExpression => self.bind_name_expression(
                expression
                    .as_any()
                    .downcast_ref::<NameExpressionSyntax>()
                    .unwrap()
                    .clone(),
            ),
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
            SyntaxKind::ParenthesizedExpression => self.bind_parenthesized_expression(
                expression
                    .as_any()
                    .downcast_ref::<ParenthesizedExpressionSyntax>()
                    .unwrap()
                    .clone(),
            ),
            _ => panic!("Binding ERROR: Unexpected syntax kind"),
        }
    }

    fn bind_parenthesized_expression(
        &self,
        parenthesized_expression: ParenthesizedExpressionSyntax,
    ) -> Box<dyn BoundExpression> {
        self.bind_expression(parenthesized_expression.get_expression())
    }

    fn bind_name_expression(
        &self,
        name_expression: NameExpressionSyntax,
    ) -> Box<dyn BoundExpression> {
        let token = name_expression.get_token();
        let value = name_expression.get_value();
        let key = SYMBOL_TABLE
            .lock()
            .unwrap()
            .iter()
            .find(|(symbol, _)| symbol.id() == token.name())
            .map(|(symbol, _)| symbol.clone());

        if let Some(key) = key {
            if self.check_scope_of_variable(&key) {
                return Box::new(BoundLiteralExpression::new(
                    value.clone(),
                    key.get_type(),
                    SyntaxKind::Variable,
                )) as Box<dyn BoundExpression>;
            }
        }

        self.diagnostics.borrow_mut().report_undefined_name(
            token.name(),
            TextSpan::new(token.position(), token.length()),
            TextPlace::Semantic,
            TextType::Error,
        );

        Box::new(BoundLiteralExpression::new(
            value.clone(),
            *value.get_type(),
            SyntaxKind::Error,
        )) as Box<dyn BoundExpression>
    }

    fn bind_literal_expression(
        &self,
        literal_expression: LiteralExpressionSyntax,
    ) -> Box<dyn BoundExpression> {
        let value = literal_expression.get_value();

        Box::new(BoundLiteralExpression::new(
            value.clone(),
            *value.get_type(),
            SyntaxKind::None,
        )) as Box<dyn BoundExpression>
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
