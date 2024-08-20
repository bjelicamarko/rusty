mod binding;
mod evaluation;
pub mod global_state;
mod lexical_analyzer;
mod reports;
mod syntax_analyzer;
mod syntax_tree;
mod util;
use binding::binder::Binder;
use evaluation::evaluator::Evaluator;
use global_state::SYMBOL_TABLE;
use reports::diagnostics::Diagnostics;
use reports::text_type::TextType;
use syntax_tree::ast::SyntaxTree;
use util::literals::LiteralValue;
use util::variable_symbol::VariableSymbol;

use crate::lexical_analyzer::lexer::Lexer;
use crate::syntax_analyzer::parser::Parser;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::rc::Rc;
use std::sync::Mutex;

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
    //diagnostics.borrow_mut().print();
    let root = parser.parse();

    // let mut binder = Binder::new(Rc::clone(&diagnostics));
    // let root = binder.bind_statement(root.clone());

    diagnostics.borrow_mut().print();

    // let evaluator = Evaluator::new(root);
    // evaluator.evaluate();
    // println!("{:?}", *SYMBOL_TABLE.lock().unwrap());
    // // let tree: SyntaxTree = SyntaxTree::new(root.clone());
    // tree.print_tree(diagnostics.borrow_mut().filter_type(TextType::Error).len() == 0);

    // let evaluator = Evaluator::new(bound_expression);
    // evaluator.evaluate();

    // let expression = String::from("-1+2+2+2");

    // let result = CalculatorParser::new().parse(&expression);
    // println!("{:#?}", result);
    // let evaluator2 = Evaluator::new(binder.bind_expression(result.unwrap().expression.clone()));
    // evaluator2.evaluate();

    Ok(())
}
