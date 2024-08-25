use crate::{
    binding::{
        bound_assignment::BoundAssignment, bound_binary_expression::BoundBinaryExpression,
        bound_binary_operator_kind::BoundBinaryOperatorKind,
        bound_constant_declaration::BoundConstantDeclaration, bound_expression::BoundExpression,
        bound_for_statement::BoundForStatement, bound_if_statement::BoundIfStatement,
        bound_literal_expression::BoundLiteralExpression, bound_statement::BoundStatement,
        bound_statement_list::BoundStatementList, bound_unary_expression::BoundUnaryExpression,
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

    fn evaluate_statements(&self, node: Box<dyn BoundStatement>) {
        if let Some(statements) = node.as_any().downcast_ref::<BoundStatementList>() {
            for statement in statements.get_statements() {
                self.evaluate_statements(statement);
            }
        }
        if let Some(statement) = node.as_any().downcast_ref::<BoundAssignment>() {
            let value = self.evaluate_expression(statement.get_bound_expression());

            self.insert_into_symbol_table(statement.get_variable(), value);
        }
        if let Some(statement) = node.as_any().downcast_ref::<BoundVariableDeclaration>() {
            let value = self.evaluate_expression(statement.get_bound_expression());

            self.insert_into_symbol_table(statement.get_variable(), value);
        }
        if let Some(statement) = node.as_any().downcast_ref::<BoundConstantDeclaration>() {
            let value = self.evaluate_expression(statement.get_bound_expression());

            self.insert_into_symbol_table(statement.get_variable(), value);
        }
        if let Some(statement) = node.as_any().downcast_ref::<BoundIfStatement>() {
            let condition = self
                .evaluate_expression(statement.get_condition())
                .as_boolean()
                .unwrap();
            if condition {
                self.evaluate_statements(statement.get_then_statement());
            } else if statement.get_else_statement().is_some() {
                self.evaluate_statements(statement.get_else_statement().unwrap());
            }
        }
        if let Some(statement) = node.as_any().downcast_ref::<BoundWhileStatement>() {
            let mut condition = self
                .evaluate_expression(statement.get_condition())
                .as_boolean()
                .unwrap();

            while condition {
                self.evaluate_statements(statement.get_body());
                condition = self
                    .evaluate_expression(statement.get_condition())
                    .as_boolean()
                    .unwrap();
            }
        }
        if let Some(statement) = node.as_any().downcast_ref::<BoundForStatement>() {
            let lower_bound = self
                .evaluate_expression(statement.get_lower_bound())
                .as_integer()
                .unwrap();
            let upper_bound = self
                .evaluate_expression(statement.get_upper_bound())
                .as_integer()
                .unwrap();

            let variable = statement.get_variable();
            for i in lower_bound..upper_bound {
                self.insert_into_symbol_table(variable.clone(), LiteralValue::Integer(i));
                self.evaluate_statements(statement.get_body());
            }
        }
    }

    fn insert_into_symbol_table(&self, variable: VariableSymbol, value: LiteralValue) {
        SYMBOL_TABLE.lock().unwrap().insert(variable, Some(value));
    }

    fn evaluate_expression(&self, node: Box<dyn BoundExpression>) -> LiteralValue {
        if let Some(expr) = node.as_any().downcast_ref::<BoundLiteralExpression>() {
            if expr.get_kind() == SyntaxKind::Variable {
                let value = {
                    SYMBOL_TABLE
                        .lock()
                        .unwrap()
                        .iter()
                        .find(|(symbol, _)| symbol.id() == expr.get_value().as_string().unwrap())
                        .map(|(_, res)| res.clone())
                        .flatten()
                };
                return value.unwrap();
            }
            return expr.get_value();
        }
        if let Some(expr) = node.as_any().downcast_ref::<BoundUnaryExpression>() {
            let operand = self.evaluate_expression(expr.get_operand());

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
            let left = self.evaluate_expression(expr.get_left());
            let right = self.evaluate_expression(expr.get_right());

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

        LiteralValue::Integer(-1)
    }
}
