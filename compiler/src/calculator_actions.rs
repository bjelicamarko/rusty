use crate::util::syntax_token::SyntaxToken;
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;

use super::calculator::{Context, TokenKind};
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type Number = String;
pub fn number(_ctx: &Ctx, token: Token) -> Number {
    token.value.into()
}
pub type Expression = Box<LogicalExpression>;
pub fn expression_logical_expression(
    _ctx: &Ctx,
    logical_expression: LogicalExpression,
) -> Expression {
    Box::new(logical_expression)
}
#[derive(Debug, Clone)]
pub enum LogicalExpression {
    RelationalExpression(RelationalExpression),
    LogicalExpressionRest(LogicalExpressionRest),
}
pub fn logical_expression_relational_expression(
    _ctx: &Ctx,
    relational_expression: RelationalExpression,
) -> LogicalExpression {
    LogicalExpression::RelationalExpression(relational_expression)
}
pub fn logical_expression_logical_expression_rest(
    _ctx: &Ctx,
    logical_expression_rest: LogicalExpressionRest,
) -> LogicalExpression {
    LogicalExpression::LogicalExpressionRest(logical_expression_rest)
}
#[derive(Debug, Clone)]
pub struct LogicalExpressionRestNoO {
    pub relational_expression: RelationalExpression,
    pub logical_expression_rest: Box<LogicalExpressionRest>,
}
pub type LogicalExpressionRest = Option<LogicalExpressionRestNoO>;
pub fn logical_expression_rest_c1(
    _ctx: &Ctx,
    relational_expression: RelationalExpression,
    logical_expression_rest: LogicalExpressionRest,
) -> LogicalExpressionRest {
    Some(LogicalExpressionRestNoO {
        relational_expression,
        logical_expression_rest: Box::new(logical_expression_rest),
    })
}
pub fn logical_expression_rest_empty(_ctx: &Ctx) -> LogicalExpressionRest {
    None
}
#[derive(Debug, Clone)]
pub struct RelationalExpression {
    pub arithmetic_expression: ArithmeticExpression,
    pub relational_expression_rest: RelationalExpressionRest,
}
pub fn relational_expression_c1(
    _ctx: &Ctx,
    arithmetic_expression: ArithmeticExpression,
    relational_expression_rest: RelationalExpressionRest,
) -> RelationalExpression {
    RelationalExpression {
        arithmetic_expression,
        relational_expression_rest,
    }
}
#[derive(Debug, Clone)]
pub struct RelationalExpressionRestC1 {
    pub arithmetic_expression: ArithmeticExpression,
    pub relational_expression_rest: Box<RelationalExpressionRest>,
}
#[derive(Debug, Clone)]
pub struct RelationalExpressionRestC2 {
    pub arithmetic_expression: ArithmeticExpression,
    pub relational_expression_rest: Box<RelationalExpressionRest>,
}
#[derive(Debug, Clone)]
pub struct RelationalExpressionRestC3 {
    pub arithmetic_expression: ArithmeticExpression,
    pub relational_expression_rest: Box<RelationalExpressionRest>,
}
pub type RelationalExpressionRest = Option<RelationalExpressionRestNoO>;
#[derive(Debug, Clone)]
pub enum RelationalExpressionRestNoO {
    C1(RelationalExpressionRestC1),
    C2(RelationalExpressionRestC2),
    C3(RelationalExpressionRestC3),
}
pub fn relational_expression_rest_c1(
    _ctx: &Ctx,
    arithmetic_expression: ArithmeticExpression,
    relational_expression_rest: RelationalExpressionRest,
) -> RelationalExpressionRest {
    Some(RelationalExpressionRestNoO::C1(
        RelationalExpressionRestC1 {
            arithmetic_expression,
            relational_expression_rest: Box::new(relational_expression_rest),
        },
    ))
}
pub fn relational_expression_rest_c2(
    _ctx: &Ctx,
    arithmetic_expression: ArithmeticExpression,
    relational_expression_rest: RelationalExpressionRest,
) -> RelationalExpressionRest {
    Some(RelationalExpressionRestNoO::C2(
        RelationalExpressionRestC2 {
            arithmetic_expression,
            relational_expression_rest: Box::new(relational_expression_rest),
        },
    ))
}
pub fn relational_expression_rest_c3(
    _ctx: &Ctx,
    arithmetic_expression: ArithmeticExpression,
    relational_expression_rest: RelationalExpressionRest,
) -> RelationalExpressionRest {
    Some(RelationalExpressionRestNoO::C3(
        RelationalExpressionRestC3 {
            arithmetic_expression,
            relational_expression_rest: Box::new(relational_expression_rest),
        },
    ))
}
pub fn relational_expression_rest_empty(_ctx: &Ctx) -> RelationalExpressionRest {
    None
}
#[derive(Debug, Clone)]
pub struct ArithmeticExpression {
    pub term: Term,
    pub arithmetic_expression_rest: ArithmeticExpressionRest,
}
pub fn arithmetic_expression_c1(
    _ctx: &Ctx,
    term: Term,
    arithmetic_expression_rest: ArithmeticExpressionRest,
) -> ArithmeticExpression {
    ArithmeticExpression {
        term,
        arithmetic_expression_rest,
    }
}
#[derive(Debug, Clone)]
pub struct ArithmeticExpressionRestC1 {
    pub term: Term,
    pub arithmetic_expression_rest: Box<ArithmeticExpressionRest>,
}
#[derive(Debug, Clone)]
pub struct ArithmeticExpressionRestC2 {
    pub term: Term,
    pub arithmetic_expression_rest: Box<ArithmeticExpressionRest>,
}
pub type ArithmeticExpressionRest = Option<ArithmeticExpressionRestNoO>;
#[derive(Debug, Clone)]
pub enum ArithmeticExpressionRestNoO {
    C1(ArithmeticExpressionRestC1),
    C2(ArithmeticExpressionRestC2),
}
pub fn arithmetic_expression_rest_c1(
    _ctx: &Ctx,
    term: Term,
    arithmetic_expression_rest: ArithmeticExpressionRest,
) -> ArithmeticExpressionRest {
    Some(ArithmeticExpressionRestNoO::C1(
        ArithmeticExpressionRestC1 {
            term,
            arithmetic_expression_rest: Box::new(arithmetic_expression_rest),
        },
    ))
}
pub fn arithmetic_expression_rest_c2(
    _ctx: &Ctx,
    term: Term,
    arithmetic_expression_rest: ArithmeticExpressionRest,
) -> ArithmeticExpressionRest {
    Some(ArithmeticExpressionRestNoO::C2(
        ArithmeticExpressionRestC2 {
            term,
            arithmetic_expression_rest: Box::new(arithmetic_expression_rest),
        },
    ))
}
pub fn arithmetic_expression_rest_empty(_ctx: &Ctx) -> ArithmeticExpressionRest {
    None
}
#[derive(Debug, Clone)]
pub struct Term {
    pub factor: Factor,
    pub term_rest: TermRest,
}
pub fn term_c1(_ctx: &Ctx, factor: Factor, term_rest: TermRest) -> Term {
    Term { factor, term_rest }
}
#[derive(Debug, Clone)]
pub struct TermRestC1 {
    pub factor: Factor,
    pub term_rest: Box<TermRest>,
}
#[derive(Debug, Clone)]
pub struct TermRestC2 {
    pub factor: Factor,
    pub term_rest: Box<TermRest>,
}
pub type TermRest = Option<TermRestNoO>;
#[derive(Debug, Clone)]
pub enum TermRestNoO {
    C1(TermRestC1),
    C2(TermRestC2),
}
pub fn term_rest_c1(_ctx: &Ctx, factor: Factor, term_rest: TermRest) -> TermRest {
    Some(TermRestNoO::C1(TermRestC1 {
        factor,
        term_rest: Box::new(term_rest),
    }))
}
pub fn term_rest_c2(_ctx: &Ctx, factor: Factor, term_rest: TermRest) -> TermRest {
    Some(TermRestNoO::C2(TermRestC2 {
        factor,
        term_rest: Box::new(term_rest),
    }))
}
pub fn term_rest_empty(_ctx: &Ctx) -> TermRest {
    None
}
#[derive(Debug, Clone)]
pub enum Factor {
    Factor1(Box<Factor>),
    Expression(Expression),
    Factor2(Box<Factor>),
    Number(Number),
    True,
    False,
}
pub fn factor_factor1(_ctx: &Ctx, factor: Factor) -> Factor {
    Factor::Factor1(Box::new(factor))
}
pub fn factor_expression(_ctx: &Ctx, expression: Expression) -> Factor {
    Factor::Expression(expression)
}
pub fn factor_factor2(_ctx: &Ctx, factor: Factor) -> Factor {
    Factor::Factor2(Box::new(factor))
}
pub fn factor_number(_ctx: &Ctx, number: Number) -> Factor {
    Factor::Number(number)
}
pub fn factor_true(_ctx: &Ctx) -> Factor {
    Factor::True
}
pub fn factor_false(_ctx: &Ctx) -> Factor {
    Factor::False
}
