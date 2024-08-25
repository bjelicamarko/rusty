use crate::{
    binding::{
        bound_assignment::BoundAssignment, bound_binary_expression::BoundBinaryExpression,
        bound_binary_operator_kind::BoundBinaryOperatorKind,
        bound_constant_declaration::BoundConstantDeclaration, bound_expression::BoundExpression,
        bound_for_statement::BoundForStatement, bound_if_statement::BoundIfStatement,
        bound_kind::BoundKind, bound_literal_expression::BoundLiteralExpression,
        bound_statement::BoundStatement, bound_statement_list::BoundStatementList,
        bound_unary_expression::BoundUnaryExpression,
        bound_unary_operator_kind::BoundUnaryOperatorKind,
        bound_variable_declaration::BoundVariableDeclaration,
        bound_while_statement::BoundWhileStatement,
    },
    global_state::SYMBOL_TABLE,
    util::{
        literals::{LiteralType, LiteralValue},
        syntax_kind::SyntaxKind,
        variable_symbol::VariableSymbol,
    },
};

pub struct Evaluator {
    statements: Box<dyn BoundStatement>,
}

impl Evaluator {
    pub fn new(statements: Box<dyn BoundStatement>) -> Self {
        Self { statements }
    }

    pub fn evaluate(&self) {
        self.evaluate_statements(self.statements.clone());
    }

    fn evaluate_statements(&self, statement: Box<dyn BoundStatement>) {
        match *statement.get_type_of_bound() {
            BoundKind::BoundStatementList => {
                return self.evaluate_statement_list(
                    statement
                        .as_any()
                        .downcast_ref::<BoundStatementList>()
                        .unwrap(),
                )
            }
            BoundKind::BoundAssignment => {
                return self.evaluate_assignment(
                    statement
                        .as_any()
                        .downcast_ref::<BoundAssignment>()
                        .unwrap(),
                )
            }
            BoundKind::BoundVariableDeclaration => {
                return self.evaluate_variable_declaration(
                    statement
                        .as_any()
                        .downcast_ref::<BoundVariableDeclaration>()
                        .unwrap(),
                )
            }
            BoundKind::BoundConstantDeclaration => {
                return self.evaluate_constant_declaration(
                    statement
                        .as_any()
                        .downcast_ref::<BoundConstantDeclaration>()
                        .unwrap(),
                )
            }
            BoundKind::BoundIfStatement => {
                return self.evaluate_if_statement(
                    statement
                        .as_any()
                        .downcast_ref::<BoundIfStatement>()
                        .unwrap(),
                )
            }
            BoundKind::BoundWhileStatement => {
                return self.evaluate_while_statement(
                    statement
                        .as_any()
                        .downcast_ref::<BoundWhileStatement>()
                        .unwrap(),
                )
            }
            BoundKind::BoundForStatement => {
                return self.evaluate_for_statement(
                    statement
                        .as_any()
                        .downcast_ref::<BoundForStatement>()
                        .unwrap(),
                )
            }
            _ => panic!("Evaluating ERROR: Unexpected bound kind for statement."),
        }
    }

    fn evaluate_statement_list(&self, statement_list: &BoundStatementList) {
        for statement in statement_list.get_statements() {
            self.evaluate_statements(statement);
        }
    }

    fn evaluate_assignment(&self, assignment: &BoundAssignment) {
        let value = self.evaluate_expression(assignment.get_bound_expression());
        self.insert_into_symbol_table(assignment.get_variable(), value);
    }

    fn evaluate_variable_declaration(&self, variable_declaration: &BoundVariableDeclaration) {
        let value = self.evaluate_expression(variable_declaration.get_bound_expression());
        self.insert_into_symbol_table(variable_declaration.get_variable(), value);
    }

    fn evaluate_constant_declaration(&self, constant_declaration: &BoundConstantDeclaration) {
        let value = self.evaluate_expression(constant_declaration.get_bound_expression());
        self.insert_into_symbol_table(constant_declaration.get_variable(), value);
    }

    fn evaluate_if_statement(&self, if_statement: &BoundIfStatement) {
        let condition = self
            .evaluate_expression(if_statement.get_condition())
            .as_boolean()
            .unwrap();
        if condition {
            self.evaluate_statements(if_statement.get_then_statement());
        } else if if_statement.get_else_statement().is_some() {
            self.evaluate_statements(if_statement.get_else_statement().unwrap());
        }
    }

    fn evaluate_while_statement(&self, while_statement: &BoundWhileStatement) {
        let mut condition = self
            .evaluate_expression(while_statement.get_condition())
            .as_boolean()
            .unwrap();

        while condition {
            self.evaluate_statements(while_statement.get_body());
            condition = self
                .evaluate_expression(while_statement.get_condition())
                .as_boolean()
                .unwrap();
        }
    }

    fn evaluate_for_statement(&self, for_statement: &BoundForStatement) {
        let lower_bound = self
            .evaluate_expression(for_statement.get_lower_bound())
            .as_integer()
            .unwrap();
        let upper_bound = self
            .evaluate_expression(for_statement.get_upper_bound())
            .as_integer()
            .unwrap();

        let variable = for_statement.get_variable();
        for i in lower_bound..upper_bound {
            self.insert_into_symbol_table(variable.clone(), LiteralValue::Integer(i));
            self.evaluate_statements(for_statement.get_body());
        }
    }

    fn insert_into_symbol_table(&self, variable: VariableSymbol, value: LiteralValue) {
        SYMBOL_TABLE.lock().unwrap().insert(variable, Some(value));
    }

    fn evaluate_expression(&self, expression: Box<dyn BoundExpression>) -> LiteralValue {
        match *expression.get_type_of_bound() {
            BoundKind::BoundLiteralExpression => {
                return self.evaluate_literal_expression(
                    expression
                        .as_any()
                        .downcast_ref::<BoundLiteralExpression>()
                        .unwrap(),
                )
            }
            BoundKind::BoundUnaryExpression => {
                return self.evaluate_unary_expression(
                    expression
                        .as_any()
                        .downcast_ref::<BoundUnaryExpression>()
                        .unwrap(),
                )
            }
            BoundKind::BoundBinaryExpression => {
                return self.evaluate_binary_expression(
                    expression
                        .as_any()
                        .downcast_ref::<BoundBinaryExpression>()
                        .unwrap(),
                )
            }
            _ => panic!("Evaluating ERROR: Unexpected bound kind for expression."),
        }
    }

    fn evaluate_literal_expression(
        &self,
        literal_expression: &BoundLiteralExpression,
    ) -> LiteralValue {
        if literal_expression.get_kind() == SyntaxKind::Variable {
            let value = {
                SYMBOL_TABLE
                    .lock()
                    .unwrap()
                    .iter()
                    .find(|(symbol, _)| {
                        symbol.id() == literal_expression.get_value().as_string().unwrap()
                    })
                    .map(|(_, res)| res.clone())
                    .flatten()
            };
            return value.unwrap();
        }
        return literal_expression.get_value();
    }

    fn evaluate_unary_expression(&self, unary_expression: &BoundUnaryExpression) -> LiteralValue {
        let operand = self.evaluate_expression(unary_expression.get_operand());

        match unary_expression.get_operator().get_kind() {
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

    fn evaluate_binary_expression(
        &self,
        binary_expression: &BoundBinaryExpression,
    ) -> LiteralValue {
        let left = self.evaluate_expression(binary_expression.get_left());
        let right = self.evaluate_expression(binary_expression.get_right());

        match binary_expression.get_operator().get_kind() {
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
            BoundBinaryOperatorKind::Less => {
                return LiteralValue::Boolean(
                    left.as_integer().unwrap() < right.as_integer().unwrap(),
                );
            }
            BoundBinaryOperatorKind::LessOrEquals => {
                return LiteralValue::Boolean(
                    left.as_integer().unwrap() <= right.as_integer().unwrap(),
                );
            }
            BoundBinaryOperatorKind::Greater => {
                return LiteralValue::Boolean(
                    left.as_integer().unwrap() > right.as_integer().unwrap(),
                );
            }
            BoundBinaryOperatorKind::GreaterOrEquals => {
                return LiteralValue::Boolean(
                    left.as_integer().unwrap() >= right.as_integer().unwrap(),
                );
            }
        }
    }
}
