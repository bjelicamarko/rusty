mod binding;
mod evaluation;
mod lexical_analyzer;
mod reports;
mod syntax_analyzer;
mod syntax_tree;
mod util;
use binding::binder::Binder;
use evaluation::evaluator::Evaluator;
use reports::diagnostics::Diagnostics;
use syntax_tree::ast::SyntaxTree;

use crate::lexical_analyzer::lexer::Lexer;
use crate::syntax_analyzer::parser::Parser;
use std::cell::RefCell;
use std::fs::File;
use std::io::{self, Read};
use std::rc::Rc;

use crate::calculator::CalculatorParser;
use rustemo::Parser as OtherParser;

#[rustfmt::skip]
mod calculator;
#[allow(unused)]
#[rustfmt::skip]
mod calculator_actions;

fn main() -> io::Result<()> {
    let mut file = File::open("examples/example_1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));

    let mut lexer = Lexer::in_memory_reader(&contents, Rc::clone(&diagnostics));

    let mut parser: Parser = Parser::new(Rc::clone(&diagnostics));
    parser.create(&mut lexer);
    let root = parser.parse();

    let binder = Binder::new(Rc::clone(&diagnostics));
    let bound_expression = binder.bind_expression(root.clone());

    diagnostics.borrow_mut().print();

    let tree: SyntaxTree = SyntaxTree::new(root.clone());
    tree.print_tree(true);

    let evaluator = Evaluator::new(bound_expression);
    evaluator.evaluate();

    let expression = String::from("2+2");

    let result = CalculatorParser::new().parse(&expression);
    println!("{:#?}", result);
    let evaluator2 = Evaluator::new(binder.bind_expression(result.unwrap().expression.clone()));
    evaluator2.evaluate();

    Ok(())
}
