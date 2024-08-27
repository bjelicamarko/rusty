#[macro_use]
extern crate rocket;
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
use reports::diagnostic::Diagnostic;
use reports::diagnostics::Diagnostics;
use reports::text_type::TextType;
use rocket::serde::{json::Json, Serialize};
use rocket::{get, launch, serde};
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

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Report {
    pub symbol_table: HashMap<String, String>,
    pub diagnostics: Vec<Diagnostic>,
    pub tree: String,
}

impl Report {
    pub fn new(diagnostics: Vec<Diagnostic>, tree: String) -> Self {
        Self {
            symbol_table: HashMap::new(),
            diagnostics,
            tree,
        }
    }

    pub fn report_symbol_table(&mut self) {
        for (key, value) in SYMBOL_TABLE.lock().unwrap().iter() {
            self.symbol_table.insert(
                key.id(),
                value.clone().unwrap().as_integer().unwrap().to_string(),
            );
        }
    }
}

#[get("/greetings")]
fn greetings() -> Json<Report> {
    let contents = String::from(
        "{
            let res = 0;
            for (j = 0 to 10) {
                res = res + j;
            }
        }",
    );
    let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));
    let mut lexer = Lexer::in_memory_reader(&contents, Rc::clone(&diagnostics));
    let mut parser: Parser = Parser::new(Rc::clone(&diagnostics));
    parser.create(&mut lexer);

    let root = parser.parse();
    let tree = format!("{:?}", root);

    let mut binder = Binder::new(Rc::clone(&diagnostics));
    let root = binder.bind_statement(root.clone());

    diagnostics.borrow_mut().print();

    let evaluator = Evaluator::new(root);
    evaluator.evaluate();

    SYMBOL_TABLE
        .lock()
        .unwrap()
        .retain(|key, _| key.is_global());
    println!("{:?}", *SYMBOL_TABLE.lock().unwrap());

    let mut report = Report::new(diagnostics.borrow().get_diagnostics(), tree);
    report.report_symbol_table();

    Json(report)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![greetings])
}
