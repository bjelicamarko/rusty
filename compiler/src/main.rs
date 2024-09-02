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
use rocket::launch;
use rocket::serde::Deserialize;
use rocket::serde::{json::Json, Serialize};
use util::literals::LiteralType;

use crate::compiler::CompilerParser;
use rustemo::Parser;

#[rustfmt::skip]
mod compiler;
#[allow(unused)]
#[rustfmt::skip]
mod compiler_actions;

use crate::lexical_analyzer::lexer::Lexer;
use crate::syntax_analyzer::parser::Parser as CustomParser;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Program {
    pub code: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Pair {
    pub id: String,
    pub value: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Report {
    pub symbol_table: Vec<Pair>,
    pub diagnostics: Vec<Diagnostic>,
}

impl Report {
    pub fn new(diagnostics: Vec<Diagnostic>) -> Self {
        Self {
            symbol_table: Vec::new(),
            diagnostics,
        }
    }

    pub fn report_symbol_table(&mut self) {
        for (key, value) in SYMBOL_TABLE.lock().unwrap().iter() {
            let value = value.clone();
            self.symbol_table.push(Pair {
                id: key.id(),
                value: if value.is_some() {
                    let value_clone = value.unwrap();
                    if *value_clone.get_type() == LiteralType::Integer {
                        value_clone.as_integer().unwrap().to_string()
                    } else {
                        value_clone.as_boolean().unwrap().to_string()
                    }
                } else {
                    "None".to_string()
                },
            });
        }
    }
}

#[post("/generate", data = "<data>")]
fn generate(data: Json<Program>) -> Json<Report> {
    SYMBOL_TABLE.lock().unwrap().clear();

    let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));
    let mut lexer = Lexer::in_memory_reader(&data.code, Rc::clone(&diagnostics));

    if diagnostics.borrow().filter_type(TextType::Error).len() == 0 {
        let mut parser: CustomParser = CustomParser::new(Rc::clone(&diagnostics));
        parser.create(&mut lexer);
        let _root = parser.parse();
        let result = CompilerParser::new().parse(&data.code);

        //println!("{:#?}", result);
        //println!("{:#?}", root);

        if diagnostics.borrow().filter_type(TextType::Error).len() == 0 {
            let mut binder = Binder::new(Rc::clone(&diagnostics));
            let root = binder.bind_statement(result.unwrap());

            if diagnostics.borrow().filter_type(TextType::Error).len() == 0 {
                let evaluator = Evaluator::new(root);
                evaluator.evaluate();
            }
        }
    }

    diagnostics.borrow_mut().print();

    SYMBOL_TABLE
        .lock()
        .unwrap()
        .retain(|key, _| key.is_global());
    println!("{:?}", *SYMBOL_TABLE.lock().unwrap());

    let mut report = Report::new(diagnostics.borrow().get_diagnostics());
    report.report_symbol_table();

    Json(report)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![generate])
}
