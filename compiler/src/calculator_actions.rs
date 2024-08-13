use core::num;
use std::str::FromStr;

use super::calculator::{Context, TokenKind};
use crate::syntax_analyzer::binary_expression::BinaryExpressionSyntax;
use crate::syntax_analyzer::literal_expression::LiteralExpressionSyntax;
use crate::syntax_analyzer::parenthesized_expression::ParenthesizedExpressionSyntax;
use crate::syntax_analyzer::unary_expression::UnaryExpressionSyntax;
use crate::util::literals::LiteralValue;
use crate::util::syntax_kind::SyntaxKind;
use crate::util::utils::transform_str;
use crate::util::{self, syntax_token::SyntaxToken};
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
use util::expression::Expression as CustomExpression;
use util::syntax_token::SyntaxToken as CustomToken;
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type Number = i32;
pub fn number(_ctx: &Ctx, token: Token) -> Number {
    token.value.parse().unwrap()
}
fn return_empty_expression() -> Pair {
    Pair {
        expression: Box::new(CustomToken::new(
            LiteralValue::String("".to_string()),
            0,
            SyntaxKind::None,
            0,
        )) as Box<dyn CustomExpression>,
        operator_type: "None".to_string(),
    }
}
fn return_operator(operator_type: &str) -> CustomToken {
    CustomToken::new(
        LiteralValue::String(transform_str(operator_type)),
        0,
        SyntaxKind::from_str(operator_type).unwrap(),
        transform_str(operator_type).len(),
    )
}
#[derive(Debug)]
pub struct Pair {
    pub expression: Box<dyn CustomExpression>,
    operator_type: String,
}
pub type Expression = Pair;
pub fn expression_logical_expression(
    _ctx: &Ctx,
    logical_expression: LogicalExpression,
) -> Expression {
    logical_expression
}
pub type LogicalExpression = Pair;
pub fn logical_expression_c1(
    _ctx: &Ctx,
    relational_expression: RelationalExpression,
    logical_expression_rest: LogicalExpressionRest,
) -> LogicalExpression {
    if logical_expression_rest.operator_type == "None" {
        return Pair {
            expression: relational_expression.expression,
            operator_type: "None".to_string(),
        };
    }
    let operator = return_operator("PipePipe");
    Pair {
        expression: Box::new(BinaryExpressionSyntax::new(
            relational_expression.expression,
            operator,
            logical_expression_rest.expression,
        )) as Box<dyn CustomExpression>,
        operator_type: "PipePipe".to_string(),
    }
}
pub type LogicalExpressionRest = Pair;
pub fn logical_expression_rest_c1(
    _ctx: &Ctx,
    relational_expression: RelationalExpression,
    logical_expression_rest: LogicalExpressionRest,
) -> LogicalExpressionRest {
    if logical_expression_rest.operator_type == "None" {
        return Pair {
            expression: relational_expression.expression,
            operator_type: "PipePipe".to_string(),
        };
    }
    let operator = return_operator("PipePipe");
    Pair {
        expression: Box::new(BinaryExpressionSyntax::new(
            relational_expression.expression,
            operator,
            logical_expression_rest.expression,
        )) as Box<dyn CustomExpression>,
        operator_type: "PipePipe".to_string(),
    }
}
pub fn logical_expression_rest_empty(_ctx: &Ctx) -> LogicalExpressionRest {
    return_empty_expression()
}
pub type RelationalExpression = Pair;
pub fn relational_expression_c1(
    _ctx: &Ctx,
    arithmetic_expression: ArithmeticExpression,
    relational_expression_rest: RelationalExpressionRest,
) -> RelationalExpression {
    if relational_expression_rest.operator_type == "None" {
        return Pair {
            expression: arithmetic_expression.expression,
            operator_type: "None".to_string(),
        };
    }
    let operator = return_operator(&relational_expression_rest.operator_type);
    Pair {
        expression: Box::new(BinaryExpressionSyntax::new(
            arithmetic_expression.expression,
            operator,
            relational_expression_rest.expression,
        )) as Box<dyn CustomExpression>,
        operator_type: relational_expression_rest.operator_type.to_string(),
    }
}
pub type RelationalExpressionRest = Pair;
pub fn relational_expression_rest_c1(
    _ctx: &Ctx,
    arithmetic_expression: ArithmeticExpression,
    relational_expression_rest: RelationalExpressionRest,
) -> RelationalExpressionRest {
    if relational_expression_rest.operator_type == "None" {
        return Pair {
            expression: arithmetic_expression.expression,
            operator_type: "AmpersandAmpersand".to_string(),
        };
    }
    let operator = return_operator(&relational_expression_rest.operator_type);
    Pair {
        expression: Box::new(BinaryExpressionSyntax::new(
            arithmetic_expression.expression,
            operator,
            relational_expression_rest.expression,
        )) as Box<dyn CustomExpression>,
        operator_type: relational_expression_rest.operator_type.to_string(),
    }
}
pub fn relational_expression_rest_c2(
    _ctx: &Ctx,
    arithmetic_expression: ArithmeticExpression,
    relational_expression_rest: RelationalExpressionRest,
) -> RelationalExpressionRest {
    if relational_expression_rest.operator_type == "None" {
        return Pair {
            expression: arithmetic_expression.expression,
            operator_type: "EqualsEquals".to_string(),
        };
    }
    let operator = return_operator(&relational_expression_rest.operator_type);
    Pair {
        expression: Box::new(BinaryExpressionSyntax::new(
            arithmetic_expression.expression,
            operator,
            relational_expression_rest.expression,
        )) as Box<dyn CustomExpression>,
        operator_type: relational_expression_rest.operator_type.to_string(),
    }
}
pub fn relational_expression_rest_c3(
    _ctx: &Ctx,
    arithmetic_expression: ArithmeticExpression,
    relational_expression_rest: RelationalExpressionRest,
) -> RelationalExpressionRest {
    if relational_expression_rest.operator_type == "None" {
        return Pair {
            expression: arithmetic_expression.expression,
            operator_type: "BangEquals".to_string(),
        };
    }
    let operator = return_operator(&relational_expression_rest.operator_type);
    Pair {
        expression: Box::new(BinaryExpressionSyntax::new(
            arithmetic_expression.expression,
            operator,
            relational_expression_rest.expression,
        )) as Box<dyn CustomExpression>,
        operator_type: relational_expression_rest.operator_type.to_string(),
    }
}
pub fn relational_expression_rest_empty(_ctx: &Ctx) -> RelationalExpressionRest {
    return_empty_expression()
}
pub type ArithmeticExpression = Pair;
pub fn arithmetic_expression_c1(
    _ctx: &Ctx,
    term: Term,
    arithmetic_expression_rest: ArithmeticExpressionRest,
) -> ArithmeticExpression {
    if arithmetic_expression_rest.operator_type == "None" {
        return Pair {
            expression: term.expression,
            operator_type: "None".to_string(),
        };
    }
    let operator = return_operator(&arithmetic_expression_rest.operator_type);
    Pair {
        expression: Box::new(BinaryExpressionSyntax::new(
            term.expression,
            operator,
            arithmetic_expression_rest.expression,
        )) as Box<dyn CustomExpression>,
        operator_type: arithmetic_expression_rest.operator_type.to_string(),
    }
}
pub type ArithmeticExpressionRest = Pair;
pub fn arithmetic_expression_rest_c1(
    _ctx: &Ctx,
    term: Term,
    arithmetic_expression_rest: ArithmeticExpressionRest,
) -> ArithmeticExpressionRest {
    if arithmetic_expression_rest.operator_type == "None" {
        return Pair {
            expression: term.expression,
            operator_type: "Plus".to_string(),
        };
    }
    let operator = return_operator(&arithmetic_expression_rest.operator_type);
    Pair {
        expression: Box::new(BinaryExpressionSyntax::new(
            term.expression,
            operator,
            arithmetic_expression_rest.expression,
        )) as Box<dyn CustomExpression>,
        operator_type: arithmetic_expression_rest.operator_type.to_string(),
    }
}
pub fn arithmetic_expression_rest_c2(
    _ctx: &Ctx,
    term: Term,
    arithmetic_expression_rest: ArithmeticExpressionRest,
) -> ArithmeticExpressionRest {
    if arithmetic_expression_rest.operator_type == "None" {
        return Pair {
            expression: term.expression,
            operator_type: "Minus".to_string(),
        };
    }
    let operator = return_operator(&arithmetic_expression_rest.operator_type);
    Pair {
        expression: Box::new(BinaryExpressionSyntax::new(
            term.expression,
            operator,
            arithmetic_expression_rest.expression,
        )) as Box<dyn CustomExpression>,
        operator_type: arithmetic_expression_rest.operator_type.to_string(),
    }
}
pub fn arithmetic_expression_rest_empty(_ctx: &Ctx) -> ArithmeticExpressionRest {
    return_empty_expression()
}

pub type Term = Pair;
pub fn term_c1(_ctx: &Ctx, factor: Factor, term_rest: TermRest) -> Term {
    if term_rest.operator_type == "None" {
        return Pair {
            expression: factor.expression,
            operator_type: "None".to_string(),
        };
    }
    let operator = return_operator(&term_rest.operator_type);
    Pair {
        expression: Box::new(BinaryExpressionSyntax::new(
            factor.expression,
            operator,
            term_rest.expression,
        )) as Box<dyn CustomExpression>,
        operator_type: term_rest.operator_type.to_string(),
    }
}
pub type TermRest = Pair;
pub fn term_rest_c1(_ctx: &Ctx, factor: Factor, term_rest: TermRest) -> TermRest {
    if term_rest.operator_type == "None" {
        return Pair {
            expression: factor.expression,
            operator_type: "Mul".to_string(),
        };
    }
    let operator = return_operator(&term_rest.operator_type);
    Pair {
        expression: Box::new(BinaryExpressionSyntax::new(
            factor.expression,
            operator,
            term_rest.expression,
        )) as Box<dyn CustomExpression>,
        operator_type: term_rest.operator_type.to_string(),
    }
}
pub fn term_rest_c2(_ctx: &Ctx, factor: Factor, term_rest: TermRest) -> TermRest {
    if term_rest.operator_type == "None" {
        return Pair {
            expression: factor.expression,
            operator_type: "Div".to_string(),
        };
    }
    let operator = return_operator(&term_rest.operator_type);
    Pair {
        expression: Box::new(BinaryExpressionSyntax::new(
            factor.expression,
            operator,
            term_rest.expression,
        )) as Box<dyn CustomExpression>,
        operator_type: term_rest.operator_type.to_string(),
    }
}
pub fn term_rest_empty(_ctx: &Ctx) -> TermRest {
    return_empty_expression()
}
pub type Factor = Pair;
pub fn factor_factor1(_ctx: &Ctx, factor: Factor) -> Factor {
    let operator = return_operator("Bang");

    Pair {
        expression: Box::new(UnaryExpressionSyntax::new(operator, factor.expression))
            as Box<dyn CustomExpression>,
        operator_type: "None".to_string(),
    }
}
pub fn factor_expression(_ctx: &Ctx, expression: Expression) -> Factor {
    let open_parenthesis_token = CustomToken::new(
        LiteralValue::String("(".to_string()),
        0,
        SyntaxKind::OpenParenthesis,
        1,
    );
    let close_parenthesis_token = CustomToken::new(
        LiteralValue::String(")".to_string()),
        0,
        SyntaxKind::CloseParenthesis,
        1,
    );

    Pair {
        expression: Box::new(ParenthesizedExpressionSyntax::new(
            open_parenthesis_token,
            expression.expression,
            close_parenthesis_token,
        )) as Box<dyn CustomExpression>,
        operator_type: "None".to_string(),
    }
}
pub fn factor_factor2(_ctx: &Ctx, factor: Factor) -> Factor {
    let operator = return_operator("Minus");
    Pair {
        expression: Box::new(UnaryExpressionSyntax::new(operator, factor.expression))
            as Box<dyn CustomExpression>,
        operator_type: "None".to_string(),
    }
}
pub fn factor_number(_ctx: &Ctx, number: Number) -> Factor {
    Pair {
        expression: Box::new(LiteralExpressionSyntax::new(
            CustomToken::new(
                LiteralValue::Integer(number),
                0,
                SyntaxKind::Number,
                number.to_string().len(),
            ),
            LiteralValue::Integer(number),
        )) as Box<dyn CustomExpression>,
        operator_type: "None".to_string(),
    }
}
pub fn factor_true(_ctx: &Ctx) -> Factor {
    Pair {
        expression: Box::new(LiteralExpressionSyntax::new(
            CustomToken::new(LiteralValue::Boolean(true), 0, SyntaxKind::True, 4),
            LiteralValue::Boolean(true),
        )) as Box<dyn CustomExpression>,
        operator_type: "None".to_string(),
    }
}
pub fn factor_false(_ctx: &Ctx) -> Factor {
    Pair {
        expression: Box::new(LiteralExpressionSyntax::new(
            CustomToken::new(LiteralValue::Boolean(false), 0, SyntaxKind::False, 5),
            LiteralValue::Boolean(false),
        )) as Box<dyn CustomExpression>,
        operator_type: "None".to_string(),
    }
}
