use super::compiler::{Context, TokenKind};
use crate::syntax_analyzer::assignment::Assignment as CustomAssignment;
use crate::syntax_analyzer::binary_expression::BinaryExpressionSyntax;
use crate::syntax_analyzer::constant_declaration::ConstantDeclaration as CustomConstantDeclaration;
use crate::syntax_analyzer::else_statement::ElseStatement;
use crate::syntax_analyzer::for_statement::ForStatement as CustomForStatement;
use crate::syntax_analyzer::if_statement::IfStatement as CustomIfStatement;
use crate::syntax_analyzer::literal_expression::LiteralExpressionSyntax;
use crate::syntax_analyzer::name_expression::NameExpressionSyntax;
use crate::syntax_analyzer::parenthesized_expression::ParenthesizedExpressionSyntax;
use crate::syntax_analyzer::statement_list::StatementList as CustomStatementList;
use crate::syntax_analyzer::unary_expression::UnaryExpressionSyntax;
use crate::syntax_analyzer::variable_declaration::VariableDeclaration as CustomVariableDeclaration;
use crate::syntax_analyzer::while_statement::WhileStatement as CustomWhileStatement;
use crate::util;
use crate::util::literals::LiteralValue;
use crate::util::syntax_kind::SyntaxKind;
use crate::util::utils::transform_str;
use rustemo::Token as RustemoToken;
use std::str::FromStr;
use util::expression::Expression as CustomExpression;
use util::statement::Statement as CustomStatement;
use util::syntax_token::SyntaxToken as CustomToken;
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type Number = i32;
pub fn number(_ctx: &Ctx, token: Token) -> Number {
    token.value.parse().unwrap()
}
pub type Identifier = String;
pub fn identifier(_ctx: &Ctx, token: Token) -> Identifier {
    token.value.into()
}
fn return_operator(operator_type: &str) -> CustomToken {
    CustomToken::new(
        transform_str(operator_type),
        LiteralValue::String(transform_str(operator_type)),
        0,
        SyntaxKind::from_str(operator_type).unwrap(),
        transform_str(operator_type).len(),
    )
}
fn create_custom_token(name: &str, is_id: bool) -> CustomToken {
    CustomToken::new(
        name.to_string(),
        LiteralValue::String(name.to_string()),
        0,
        if is_id {
            SyntaxKind::IdentifierToken
        } else {
            SyntaxKind::from_str(name).unwrap()
        },
        name.len(),
    )
}
pub type Program = StatementList;
pub fn program_statement_list(_ctx: &Ctx, statement_list: StatementList) -> Program {
    statement_list
}
pub type StatementList = Box<dyn CustomStatement>;
pub fn statement_list_statement1(_ctx: &Ctx, statement1: Statement1) -> StatementList {
    let open_brace = create_custom_token("{", false);
    let close_brace = create_custom_token("}", false);
    Box::new(CustomStatementList::new(
        open_brace,
        statement1,
        close_brace,
    )) as Box<dyn CustomStatement>
}
pub type Statement1 = Vec<Box<dyn CustomStatement>>;
pub fn statement1_c1(
    _ctx: &Ctx,
    mut statement1: Statement1,
    statement: Box<dyn CustomStatement>,
) -> Statement1 {
    statement1.push(statement);
    statement1
}
pub fn statement1_statement(_ctx: &Ctx, statement: Box<dyn CustomStatement>) -> Statement1 {
    vec![statement]
}
pub type Statement = Box<dyn CustomStatement>;
pub fn statement_assignment(_ctx: &Ctx, assignment: Assignment) -> Statement {
    assignment
}
pub fn statement_if_statement(_ctx: &Ctx, if_statement: IfStatement) -> Statement {
    if_statement
}
pub fn statement_variable_declaration(
    _ctx: &Ctx,
    variable_declaration: VariableDeclaration,
) -> Statement {
    variable_declaration
}
pub fn statement_constant_declaration(
    _ctx: &Ctx,
    constant_declaration: ConstantDeclaration,
) -> Statement {
    constant_declaration
}
pub fn statement_while_statement(_ctx: &Ctx, while_statement: WhileStatement) -> Statement {
    while_statement
}
pub fn statement_for_statement(_ctx: &Ctx, for_statement: ForStatement) -> Statement {
    for_statement
}
pub fn statement_statement_list(_ctx: &Ctx, statement_list: StatementList) -> Statement {
    statement_list
}
pub type Assignment = Box<dyn CustomStatement>;
pub fn assignment_assignment(
    _ctx: &Ctx,
    identifier: Identifier,
    expression: Box<dyn CustomExpression>,
) -> Box<dyn CustomStatement> {
    let id = create_custom_token(&identifier, true);
    let equals = create_custom_token("=", false);
    let semi_colon = create_custom_token(";", false);

    Box::new(CustomAssignment::new(id, equals, expression, semi_colon)) as Box<dyn CustomStatement>
}
pub type IfStatement = Box<dyn CustomStatement>;
pub fn if_statement_if_statement(
    _ctx: &Ctx,
    expression: Box<dyn CustomExpression>,
    statement_list: Box<dyn CustomStatement>,
    else_clause: Option<Box<dyn CustomStatement>>,
) -> Box<dyn CustomStatement> {
    let if_token = create_custom_token("if", false);
    let open_parenthesis = create_custom_token("(", false);
    let close_parenthesis = create_custom_token(")", false);

    Box::new(CustomIfStatement::new(
        if_token,
        open_parenthesis,
        expression,
        close_parenthesis,
        statement_list,
        else_clause,
    )) as Box<dyn CustomStatement>
}
pub type ElseClauseOpt = Option<Box<dyn CustomStatement>>;
pub fn else_clause_opt_else_clause(_ctx: &Ctx, else_clause: ElseClause) -> ElseClauseOpt {
    Some(else_clause)
}
pub fn else_clause_opt_empty(_ctx: &Ctx) -> ElseClauseOpt {
    None
}
pub type ElseClause = Box<dyn CustomStatement>;
pub fn else_clause_else_statement(
    _ctx: &Ctx,
    statement_list: Box<dyn CustomStatement>,
) -> ElseClause {
    let else_token = create_custom_token("else", false);

    Box::new(ElseStatement::new(else_token, statement_list)) as Box<dyn CustomStatement>
}
pub type VariableDeclaration = Box<dyn CustomStatement>;
pub fn variable_declaration_variable_declaration(
    _ctx: &Ctx,
    identifier: Identifier,
    expression: Box<dyn CustomExpression>,
) -> Box<dyn CustomStatement> {
    let let_token = create_custom_token("let", false);
    let id = create_custom_token(&identifier, true);
    let equals = create_custom_token("=", false);
    let semi_colon = create_custom_token(";", false);
    Box::new(CustomVariableDeclaration::new(
        let_token, id, equals, expression, semi_colon,
    )) as Box<dyn CustomStatement>
}
pub type ConstantDeclaration = Box<dyn CustomStatement>;
pub fn constant_declaration_constant_declaration(
    _ctx: &Ctx,
    identifier: Identifier,
    expression: Box<dyn CustomExpression>,
) -> Box<dyn CustomStatement> {
    let const_token = create_custom_token("const", false);
    let id = create_custom_token(&identifier, true);
    let equals = create_custom_token("=", false);
    let semi_colon = create_custom_token(";", false);
    Box::new(CustomConstantDeclaration::new(
        const_token,
        id,
        equals,
        expression,
        semi_colon,
    )) as Box<dyn CustomStatement>
}
pub type WhileStatement = Box<dyn CustomStatement>;
pub fn while_statement_while(
    _ctx: &Ctx,
    expression: Box<dyn CustomExpression>,
    statement_list: Box<dyn CustomStatement>,
) -> Box<dyn CustomStatement> {
    let while_token = create_custom_token("while", false);
    let open_parenthesis = create_custom_token("(", false);
    let close_parenthesis = create_custom_token(")", false);

    Box::new(CustomWhileStatement::new(
        while_token,
        open_parenthesis,
        expression,
        close_parenthesis,
        statement_list,
    )) as Box<dyn CustomStatement>
}
pub type ForStatement = Box<dyn CustomStatement>;
pub fn for_statement_for(
    _ctx: &Ctx,
    identifier: Identifier,
    expression_5: Box<dyn CustomExpression>,
    expression_7: Box<dyn CustomExpression>,
    statement_list: Box<dyn CustomStatement>,
) -> Box<dyn CustomStatement> {
    let for_token = create_custom_token("for", false);
    let open_parenthesis = create_custom_token("(", false);
    let id = create_custom_token(&identifier, true);
    let equals = create_custom_token("=", false);
    let to = create_custom_token("to", false);
    let close_parenthesis = create_custom_token(")", false);

    Box::new(CustomForStatement::new(
        for_token,
        open_parenthesis,
        id,
        equals,
        expression_5,
        to,
        expression_7,
        close_parenthesis,
        statement_list,
    )) as Box<dyn CustomStatement>
}
pub type Expression = Box<dyn CustomExpression>;
pub fn expression_add(
    _ctx: &Ctx,
    left: Box<dyn CustomExpression>,
    right: Box<dyn CustomExpression>,
) -> Box<dyn CustomExpression> {
    let operator: CustomToken = return_operator("Plus");

    Box::new(BinaryExpressionSyntax::new(left, operator, right)) as Box<dyn CustomExpression>
}
pub fn expression_sub(
    _ctx: &Ctx,
    left: Box<dyn CustomExpression>,
    right: Box<dyn CustomExpression>,
) -> Box<dyn CustomExpression> {
    let operator: CustomToken = return_operator("Minus");

    Box::new(BinaryExpressionSyntax::new(left, operator, right)) as Box<dyn CustomExpression>
}
pub fn expression_mul(
    _ctx: &Ctx,
    left: Box<dyn CustomExpression>,
    right: Box<dyn CustomExpression>,
) -> Box<dyn CustomExpression> {
    let operator: CustomToken = return_operator("Mul");

    Box::new(BinaryExpressionSyntax::new(left, operator, right)) as Box<dyn CustomExpression>
}
pub fn expression_div(
    _ctx: &Ctx,
    left: Box<dyn CustomExpression>,
    right: Box<dyn CustomExpression>,
) -> Box<dyn CustomExpression> {
    let operator: CustomToken = return_operator("Div");

    Box::new(BinaryExpressionSyntax::new(left, operator, right)) as Box<dyn CustomExpression>
}
pub fn expression_equals(
    _ctx: &Ctx,
    left: Box<dyn CustomExpression>,
    right: Box<dyn CustomExpression>,
) -> Box<dyn CustomExpression> {
    let operator: CustomToken = return_operator("EqualsEquals");

    Box::new(BinaryExpressionSyntax::new(left, operator, right)) as Box<dyn CustomExpression>
}
pub fn expression_not_equals(
    _ctx: &Ctx,
    left: Box<dyn CustomExpression>,
    right: Box<dyn CustomExpression>,
) -> Box<dyn CustomExpression> {
    let operator: CustomToken = return_operator("BangEquals");

    Box::new(BinaryExpressionSyntax::new(left, operator, right)) as Box<dyn CustomExpression>
}
pub fn expression_less(
    _ctx: &Ctx,
    left: Box<dyn CustomExpression>,
    right: Box<dyn CustomExpression>,
) -> Box<dyn CustomExpression> {
    let operator: CustomToken = return_operator("Less");

    Box::new(BinaryExpressionSyntax::new(left, operator, right)) as Box<dyn CustomExpression>
}
pub fn expression_less_or_equals(
    _ctx: &Ctx,
    left: Box<dyn CustomExpression>,
    right: Box<dyn CustomExpression>,
) -> Box<dyn CustomExpression> {
    let operator: CustomToken = return_operator("LessOrEquals");

    Box::new(BinaryExpressionSyntax::new(left, operator, right)) as Box<dyn CustomExpression>
}
pub fn expression_greater(
    _ctx: &Ctx,
    left: Box<dyn CustomExpression>,
    right: Box<dyn CustomExpression>,
) -> Box<dyn CustomExpression> {
    let operator: CustomToken = return_operator("Greater");

    Box::new(BinaryExpressionSyntax::new(left, operator, right)) as Box<dyn CustomExpression>
}
pub fn expression_greater_or_equals(
    _ctx: &Ctx,
    left: Box<dyn CustomExpression>,
    right: Box<dyn CustomExpression>,
) -> Box<dyn CustomExpression> {
    let operator = return_operator("GreaterOrEquals");

    Box::new(BinaryExpressionSyntax::new(left, operator, right)) as Box<dyn CustomExpression>
}
pub fn expression_and(
    _ctx: &Ctx,
    left: Box<dyn CustomExpression>,
    right: Box<dyn CustomExpression>,
) -> Box<dyn CustomExpression> {
    let operator = return_operator("AmpersandAmpersand");

    Box::new(BinaryExpressionSyntax::new(left, operator, right)) as Box<dyn CustomExpression>
}
pub fn expression_or(
    _ctx: &Ctx,
    left: Box<dyn CustomExpression>,
    right: Box<dyn CustomExpression>,
) -> Box<dyn CustomExpression> {
    let operator = return_operator("PipePipe");

    Box::new(BinaryExpressionSyntax::new(left, operator, right)) as Box<dyn CustomExpression>
}
pub fn expression_unary_minus(
    _ctx: &Ctx,
    expression: Box<dyn CustomExpression>,
) -> Box<dyn CustomExpression> {
    let operator = return_operator("Minus");

    Box::new(UnaryExpressionSyntax::new(operator, expression)) as Box<dyn CustomExpression>
}
pub fn expression_unary_negation(
    _ctx: &Ctx,
    expression: Box<dyn CustomExpression>,
) -> Box<dyn CustomExpression> {
    let operator = return_operator("Bang");

    Box::new(UnaryExpressionSyntax::new(operator, expression)) as Box<dyn CustomExpression>
}
pub fn expression_expression(
    _ctx: &Ctx,
    expression: Box<dyn CustomExpression>,
) -> Box<dyn CustomExpression> {
    let open_parenthesis_token = create_custom_token("(", false);
    let close_parenthesis_token = create_custom_token(")", false);

    Box::new(ParenthesizedExpressionSyntax::new(
        open_parenthesis_token,
        expression,
        close_parenthesis_token,
    )) as Box<dyn CustomExpression>
}
pub fn expression_number(_ctx: &Ctx, number: Number) -> Box<dyn CustomExpression> {
    Box::new(LiteralExpressionSyntax::new(CustomToken::new(
        number.to_string(),
        LiteralValue::Integer(number),
        0,
        SyntaxKind::Number,
        number.to_string().len(),
    ))) as Box<dyn CustomExpression>
}
pub fn expression_true(_ctx: &Ctx) -> Box<dyn CustomExpression> {
    Box::new(LiteralExpressionSyntax::new(CustomToken::new(
        "true".to_string(),
        LiteralValue::Boolean(true),
        0,
        SyntaxKind::True,
        4,
    ))) as Box<dyn CustomExpression>
}
pub fn expression_false(_ctx: &Ctx) -> Box<dyn CustomExpression> {
    Box::new(LiteralExpressionSyntax::new(CustomToken::new(
        "false".to_string(),
        LiteralValue::Boolean(false),
        0,
        SyntaxKind::False,
        5,
    ))) as Box<dyn CustomExpression>
}
pub fn expression_identifier(_ctx: &Ctx, identifier: Identifier) -> Box<dyn CustomExpression> {
    let custom_token = CustomToken::new(
        identifier.to_string(),
        LiteralValue::String((identifier.to_string())),
        0,
        SyntaxKind::IdentifierToken,
        identifier.to_string().len(),
    );
    Box::new(NameExpressionSyntax::new(custom_token)) as Box<dyn CustomExpression>
}
