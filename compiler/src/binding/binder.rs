use crate::global_state::{insert_into_symbol_table, SYMBOL_TABLE};
use crate::syntax_analyzer::constant_declaration::ConstantDeclaration;
use crate::syntax_analyzer::else_statement::ElseStatement;
use crate::syntax_analyzer::for_statement::ForStatement;
use crate::syntax_analyzer::if_statement::IfStatement;
use crate::syntax_analyzer::name_expression::NameExpressionSyntax;
use crate::syntax_analyzer::variable_declaration::VariableDeclaration;
use crate::syntax_analyzer::while_statement::WhileStatement;
use crate::util::literals::LiteralType;
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

use super::bound_constant_declaration::BoundConstantDeclaration;
use super::bound_for_statement::BoundForStatement;
use super::bound_if_statement::BoundIfStatement;
use super::bound_scope::BoundScope;
use super::bound_variable_declaration::BoundVariableDeclaration;
use super::bound_while_statement::BoundWhileStatement;
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
            SyntaxKind::StatementList => self
                .bind_statement_list(statement.as_any().downcast_ref::<StatementList>().unwrap()),
            SyntaxKind::Assignment => {
                self.bind_assignment(statement.as_any().downcast_ref::<Assignment>().unwrap())
            }
            SyntaxKind::VariableDeclaration => self.bind_variable_declaration(
                statement
                    .as_any()
                    .downcast_ref::<VariableDeclaration>()
                    .unwrap(),
            ),
            SyntaxKind::ConstantDeclaration => self.bind_constant_declaration(
                statement
                    .as_any()
                    .downcast_ref::<ConstantDeclaration>()
                    .unwrap(),
            ),
            SyntaxKind::IfStatement => {
                self.bind_if_statement(statement.as_any().downcast_ref::<IfStatement>().unwrap())
            }
            SyntaxKind::ElseStatement => self
                .bind_else_statement(statement.as_any().downcast_ref::<ElseStatement>().unwrap()),
            SyntaxKind::WhileStatement => self
                .bind_while_statement(statement.as_any().downcast_ref::<WhileStatement>().unwrap()),
            SyntaxKind::ForStatement => {
                self.bind_for_statement(statement.as_any().downcast_ref::<ForStatement>().unwrap())
            }
            _ => panic!("Binding ERROR: Unexpected syntax kind"),
        }
    }

    fn bind_statement_list(&mut self, statement_list: &StatementList) -> Box<dyn BoundStatement> {
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

    fn bind_for_statement(&mut self, for_statement: &ForStatement) -> Box<dyn BoundStatement> {
        let lower_bound = self
            .bind_expression_and_check_type(for_statement.get_lower_bound(), LiteralType::Integer);
        let upper_bound = self
            .bind_expression_and_check_type(for_statement.get_upper_bound(), LiteralType::Integer);

        let name = for_statement.get_identifier().name();
        let variable = VariableSymbol::new(name, LiteralType::Integer, false, false);

        insert_into_symbol_table(&variable, None);
        self.scope.variables.push(variable.clone());

        let body = self.bind_statement(for_statement.get_body());

        Box::new(BoundForStatement::new(
            variable,
            lower_bound,
            upper_bound,
            body,
        )) as Box<dyn BoundStatement>
    }

    fn bind_while_statement(
        &mut self,
        while_statement: &WhileStatement,
    ) -> Box<dyn BoundStatement> {
        let condition = self
            .bind_expression_and_check_type(while_statement.get_condition(), LiteralType::Boolean);
        let body = self.bind_statement(while_statement.get_body());

        Box::new(BoundWhileStatement::new(condition, body)) as Box<dyn BoundStatement>
    }

    fn bind_if_statement(&mut self, if_statement: &IfStatement) -> Box<dyn BoundStatement> {
        let condition =
            self.bind_expression_and_check_type(if_statement.get_condition(), LiteralType::Boolean);
        let then_statement = self.bind_statement(if_statement.get_then_statement());
        let else_statement = if if_statement.get_else_clause().is_some() {
            Some(self.bind_statement(if_statement.get_else_clause().unwrap()))
        } else {
            None
        };

        Box::new(BoundIfStatement::new(
            condition,
            then_statement,
            else_statement,
        )) as Box<dyn BoundStatement>
    }

    fn bind_else_statement(&mut self, else_statement: &ElseStatement) -> Box<dyn BoundStatement> {
        self.bind_statement(else_statement.get_else_statement())
    }

    fn bind_constant_declaration(
        &mut self,
        constant_declaration: &ConstantDeclaration,
    ) -> Box<dyn BoundStatement> {
        let token = constant_declaration.get_variable();

        let key = SYMBOL_TABLE
            .lock()
            .unwrap()
            .iter()
            .find(|(symbol, _)| symbol.id() == token.name())
            .map(|(symbol, _)| symbol.clone());

        if let Some(key) = key {
            if self.check_scope_of_variable(&key) {
                self.diagnostics
                    .borrow_mut()
                    .report_variable_already_declared(
                        token.name(),
                        TextSpan::new(token.position(), token.length()),
                        TextPlace::Semantic,
                        TextType::Error,
                    );
            }
        }

        let expr = self.bind_expression(constant_declaration.get_expression());
        let variable = VariableSymbol::new(
            token.name(),
            *expr.get_type(),
            true,
            self.scope
                .get_parent()
                .as_mut()
                .unwrap()
                .borrow()
                .to_owned()
                .get_parent()
                .is_none(),
        );

        insert_into_symbol_table(&variable, None);

        self.scope.variables.push(variable.clone());

        Box::new(BoundConstantDeclaration::new(variable, expr))
    }

    fn bind_variable_declaration(
        &mut self,
        variable_declaration: &VariableDeclaration,
    ) -> Box<dyn BoundStatement> {
        let token = variable_declaration.get_variable();

        let key = SYMBOL_TABLE
            .lock()
            .unwrap()
            .iter()
            .find(|(symbol, _)| symbol.id() == token.name())
            .map(|(symbol, _)| symbol.clone());

        if let Some(key) = key {
            if self.check_scope_of_variable(&key) {
                self.diagnostics
                    .borrow_mut()
                    .report_variable_already_declared(
                        token.name(),
                        TextSpan::new(token.position(), token.length()),
                        TextPlace::Semantic,
                        TextType::Error,
                    );
            }
        }

        let expr = self.bind_expression(variable_declaration.get_expression());
        let variable = VariableSymbol::new(
            token.name(),
            *expr.get_type(),
            false,
            self.scope
                .get_parent()
                .as_mut()
                .unwrap()
                .borrow()
                .to_owned()
                .get_parent()
                .is_none(),
        );

        insert_into_symbol_table(&variable, None);

        self.scope.variables.push(variable.clone());

        Box::new(BoundVariableDeclaration::new(variable, expr))
    }

    fn bind_assignment(&mut self, assignment: &Assignment) -> Box<dyn BoundStatement> {
        let token = assignment.get_variable();

        let key = SYMBOL_TABLE
            .lock()
            .unwrap()
            .iter()
            .find(|(symbol, _)| symbol.id() == token.name())
            .map(|(symbol, _)| symbol.clone());

        if key.is_none() || (key.is_some() && !self.check_scope_of_variable(&key.unwrap())) {
            self.diagnostics.borrow_mut().report_variable_not_declared(
                token.name(),
                TextSpan::new(token.position(), token.length()),
                TextPlace::Semantic,
                TextType::Error,
            );
        }

        let key2 = SYMBOL_TABLE
            .lock()
            .unwrap()
            .iter()
            .find(|(symbol, _)| symbol.id() == token.name())
            .map(|(symbol, _)| symbol.clone());

        if key2.is_some()
            && key2.clone().unwrap().is_read_only()
            && self.check_scope_of_variable(&key2.unwrap())
        {
            self.diagnostics.borrow_mut().report_constant_redefined(
                token.name(),
                TextSpan::new(token.position(), token.length()),
                TextPlace::Semantic,
                TextType::Error,
            );
        }

        let expr = self.bind_expression(assignment.get_expression());
        let variable = VariableSymbol::new(
            token.name(),
            *expr.get_type(),
            false,
            self.scope
                .get_parent()
                .as_mut()
                .unwrap()
                .borrow()
                .to_owned()
                .get_parent()
                .is_none(),
        );

        insert_into_symbol_table(&variable, None);

        self.scope.variables.push(variable.clone());

        Box::new(BoundAssignment::new(variable, expr))
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

    fn bind_expression_and_check_type(
        &self,
        expression: Box<dyn Expression>,
        target_type: LiteralType,
    ) -> Box<dyn BoundExpression> {
        let result = self.bind_expression(expression);

        if *result.get_type() != target_type {
            self.diagnostics.borrow_mut().report_invalid_literal_type(
                *result.get_type(),
                target_type,
                TextSpan::new(0, 2),
                TextPlace::Semantic,
                TextType::Error,
            );
        }

        result
    }

    fn bind_expression(&self, expression: Box<dyn Expression>) -> Box<dyn BoundExpression> {
        match *expression.get_kind() {
            SyntaxKind::NameExpression => self.bind_name_expression(
                expression
                    .as_any()
                    .downcast_ref::<NameExpressionSyntax>()
                    .unwrap(),
            ),
            SyntaxKind::LiteralExpression => self.bind_literal_expression(
                expression
                    .as_any()
                    .downcast_ref::<LiteralExpressionSyntax>()
                    .unwrap(),
            ),
            SyntaxKind::UnaryExpression => self.bind_unary_expression(
                expression
                    .as_any()
                    .downcast_ref::<UnaryExpressionSyntax>()
                    .unwrap(),
            ),
            SyntaxKind::BinaryExpression => self.bind_binary_expression(
                expression
                    .as_any()
                    .downcast_ref::<BinaryExpressionSyntax>()
                    .unwrap(),
            ),
            SyntaxKind::ParenthesizedExpression => self.bind_parenthesized_expression(
                expression
                    .as_any()
                    .downcast_ref::<ParenthesizedExpressionSyntax>()
                    .unwrap(),
            ),
            _ => panic!("Binding ERROR: Unexpected syntax kind"),
        }
    }

    fn bind_parenthesized_expression(
        &self,
        parenthesized_expression: &ParenthesizedExpressionSyntax,
    ) -> Box<dyn BoundExpression> {
        self.bind_expression(parenthesized_expression.get_expression())
    }

    fn bind_name_expression(
        &self,
        name_expression: &NameExpressionSyntax,
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
        literal_expression: &LiteralExpressionSyntax,
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
        unary_expression: &UnaryExpressionSyntax,
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
        binary_expression: &BinaryExpressionSyntax,
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
