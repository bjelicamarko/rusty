#[macro_use]
extern crate rocket;
mod binding;
mod evaluation;
pub mod global_state;
mod lexical_analyzer;
mod reports;
mod syntax_analyzer;
mod syntax_tree;
mod tests;
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
use util::parser_type::ParserType;
use util::statement::Statement;

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
use std::time::Instant;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Program {
    pub code: String,
    pub parser: ParserType,
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
    pub seconds: f64,
}

impl Report {
    pub fn new(diagnostics: Vec<Diagnostic>) -> Self {
        Self {
            symbol_table: Vec::new(),
            diagnostics,
            seconds: 0.0,
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
    let start = Instant::now();

    let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));

    compile_program(Rc::clone(&diagnostics), &data.code, &data.parser);

    diagnostics.borrow_mut().print();

    let mut report = Report::new(diagnostics.borrow().get_diagnostics());
    report.report_symbol_table();

    let end = Instant::now();
    let duration = end.duration_since(start);
    report.seconds = duration.as_secs_f64();

    Json(report)
}

fn compile_program(diagnostics: Rc<RefCell<Diagnostics>>, code: &str, parser_type: &ParserType) {
    SYMBOL_TABLE.lock().unwrap().clear();

    let mut lexer = Lexer::in_memory_reader(code, Rc::clone(&diagnostics));

    if diagnostics.borrow().filter_type(TextType::Error).len() == 0 {
        let res: Option<Box<dyn Statement>>;

        if *parser_type == ParserType::Recursive {
            let mut parser: CustomParser = CustomParser::new(Rc::clone(&diagnostics));
            parser.create(&mut lexer);
            res = Some(parser.parse());
        } else {
            res = Some(CompilerParser::new().parse(code).unwrap());
        }

        if diagnostics.borrow().filter_type(TextType::Error).len() == 0 {
            let mut binder = Binder::new(Rc::clone(&diagnostics));
            let root = binder.bind_statement(res.unwrap());

            if diagnostics.borrow().filter_type(TextType::Error).len() == 0 {
                let evaluator = Evaluator::new(root);
                evaluator.evaluate();
            }
        }
    }

    SYMBOL_TABLE
        .lock()
        .unwrap()
        .retain(|key, _| key.is_global());
    println!("{:?}", *SYMBOL_TABLE.lock().unwrap());
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![generate])
}
