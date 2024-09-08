#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        compile_program,
        global_state::SYMBOL_TABLE,
        reports::{diagnostics::Diagnostics, text_place::TextPlace, text_type::TextType},
        util::{literals::LiteralValue, parser_type::ParserType},
        Program,
    };

    #[test]
    fn test_suit() {
        test_variables_assignments(&Program {
            code: "{ let a = 3; const b = 4; let res = 3 + a * b;  }".to_string(),
            parser: ParserType::Recursive,
        });
        test_variables_assignments(&Program {
            code: "{ let a = 3; const b = 4; let res = 3 + a * b;  }".to_string(),
            parser: ParserType::Lr,
        });

        test_for_loop(&Program {
            code: "{ let res = 0; for (j = 0 to 10) { res = res + j; } }".to_string(),
            parser: ParserType::Recursive,
        });
        test_for_loop(&Program {
            code: "{ let res = 0; for (j = 0 to 10) { res = res + j; } }".to_string(),
            parser: ParserType::Lr,
        });

        test_while_loop(&Program {
            code: "{ let res = 0; while (res < 5) { res = res + 1; } }".to_string(),
            parser: ParserType::Recursive,
        });
        test_while_loop(&Program {
            code: "{ let res = 0; while (res < 5) { res = res + 1; } }".to_string(),
            parser: ParserType::Lr,
        });

        test_if_statement(&Program {
            code: "{ let a = 3; if (a == 3) { a = 4; } }".to_string(),
            parser: ParserType::Recursive,
        });
        test_if_statement(&Program {
            code: "{ let a = 3; if (a == 3) { a = 4; } }".to_string(),
            parser: ParserType::Lr,
        });

        test_else_statement(&Program {
            code: "{ let a = 3; if (a != 3) { a = 4; } else { a = 5; } }".to_string(),
            parser: ParserType::Recursive,
        });
        test_else_statement(&Program {
            code: "{ let a = 3; if (a != 3) { a = 4; } else { a = 5; } }".to_string(),
            parser: ParserType::Lr,
        });

        test_reassignment_variable_already_declared(&Program {
            code: "{ let a = 0; let a = 1; }".to_string(),
            parser: ParserType::Recursive,
        });
        test_reassignment_variable_already_declared(&Program {
            code: "{ let a = 0; let a = 1; }".to_string(),
            parser: ParserType::Lr,
        });

        test_const_reassignment(&Program {
            code: "{ const a = 3; a = 4; }".to_string(),
            parser: ParserType::Recursive,
        });
        test_const_reassignment(&Program {
            code: "{ const a = 3; a = 4; }".to_string(),
            parser: ParserType::Lr,
        });

        test_assignment_without_declaration(&Program {
            code: "{ a = 4; }".to_string(),
            parser: ParserType::Recursive,
        });
        test_assignment_without_declaration(&Program {
            code: "{ a = 4; }".to_string(),
            parser: ParserType::Lr,
        });

        test_using_non_existing_variable(&Program {
            code: "{ let res = 3 + a; }".to_string(),
            parser: ParserType::Recursive,
        });
        test_using_non_existing_variable(&Program {
            code: "{ let res = 3 + a; }".to_string(),
            parser: ParserType::Lr,
        });

        test_mixing_operators(&Program {
            code: "{ let a = true; let b = 3; let res = a + b; }".to_string(),
            parser: ParserType::Recursive,
        });
        test_mixing_operators(&Program {
            code: "{ let a = true; let b = 3; let res = a + b; }".to_string(),
            parser: ParserType::Lr,
        });

        test_bad_scoping(&Program {
            code: "{ let a = true; { let b = 3; { let a = 3; } } }".to_string(),
            parser: ParserType::Recursive,
        });
        test_bad_scoping(&Program {
            code: "{ let a = true; { let b = 3; { let a = 3; } } }".to_string(),
            parser: ParserType::Lr,
        });

        test_good_scoping(&Program {
            code: "{ let a = 0; { let b = 3; a = b; } { let b = 4; a = b; } }".to_string(),
            parser: ParserType::Recursive,
        });
        test_good_scoping(&Program {
            code: "{ let a = 0; { let b = 3; a = b; } { let b = 4; a = b; } }".to_string(),
            parser: ParserType::Lr,
        });
    }

    fn test_variables_assignments(data: &Program) {
        let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));

        compile_program(Rc::clone(&diagnostics), &data.code, &data.parser);

        assert_eq!(get_value_from_key("a"), Some(LiteralValue::Integer(3)));
        assert_eq!(get_value_from_key("b"), Some(LiteralValue::Integer(4)));
        assert_eq!(get_value_from_key("res"), Some(LiteralValue::Integer(15)));
        assert_eq!(
            check_for_errors_in_diagnostics(Rc::clone(&diagnostics)),
            false
        );

        diagnostics.borrow_mut().print();
    }

    fn test_if_statement(data: &Program) {
        let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));

        compile_program(Rc::clone(&diagnostics), &data.code, &data.parser);

        assert_eq!(get_value_from_key("a"), Some(LiteralValue::Integer(4)));
        assert_eq!(
            check_for_errors_in_diagnostics(Rc::clone(&diagnostics)),
            false
        );

        diagnostics.borrow_mut().print();
    }

    fn test_else_statement(data: &Program) {
        let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));

        compile_program(Rc::clone(&diagnostics), &data.code, &data.parser);

        assert_eq!(get_value_from_key("a"), Some(LiteralValue::Integer(5)));
        assert_eq!(
            check_for_errors_in_diagnostics(Rc::clone(&diagnostics)),
            false
        );

        diagnostics.borrow_mut().print();
    }

    fn test_for_loop(data: &Program) {
        let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));

        compile_program(Rc::clone(&diagnostics), &data.code, &data.parser);

        assert_eq!(get_value_from_key("res"), Some(LiteralValue::Integer(45)));
        assert_eq!(
            check_for_errors_in_diagnostics(Rc::clone(&diagnostics)),
            false
        );

        diagnostics.borrow_mut().print();
    }

    fn test_while_loop(data: &Program) {
        let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));

        compile_program(Rc::clone(&diagnostics), &data.code, &data.parser);

        assert_eq!(get_value_from_key("res"), Some(LiteralValue::Integer(5)));
        assert_eq!(
            check_for_errors_in_diagnostics(Rc::clone(&diagnostics)),
            false
        );

        diagnostics.borrow_mut().print();
    }

    fn test_reassignment_variable_already_declared(data: &Program) {
        let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));

        compile_program(Rc::clone(&diagnostics), &data.code, &data.parser);

        assert_eq!(get_value_from_key("a"), None);
        assert_eq!(
            check_for_errors_in_diagnostics(Rc::clone(&diagnostics)),
            true
        );
        assert_eq!(
            check_specific_message_in_diagnostics(
                Rc::clone(&diagnostics),
                "Name a is already declared.",
                &TextPlace::Semantic,
                &TextType::Error
            ),
            true
        );
        diagnostics.borrow_mut().print();
    }

    fn test_const_reassignment(data: &Program) {
        let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));

        compile_program(Rc::clone(&diagnostics), &data.code, &data.parser);

        assert_eq!(get_value_from_key("a"), None);
        assert_eq!(
            check_for_errors_in_diagnostics(Rc::clone(&diagnostics)),
            true
        );
        assert_eq!(
            check_specific_message_in_diagnostics(
                Rc::clone(&diagnostics),
                "Const a cannot be redefined.",
                &TextPlace::Semantic,
                &TextType::Error
            ),
            true
        );
        diagnostics.borrow_mut().print();
    }

    fn test_assignment_without_declaration(data: &Program) {
        let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));

        compile_program(Rc::clone(&diagnostics), &data.code, &data.parser);

        assert_eq!(get_value_from_key("a"), None);
        assert_eq!(
            check_for_errors_in_diagnostics(Rc::clone(&diagnostics)),
            true
        );
        assert_eq!(
            check_specific_message_in_diagnostics(
                Rc::clone(&diagnostics),
                "Name a is not declared yet.",
                &TextPlace::Semantic,
                &TextType::Error
            ),
            true
        );
        diagnostics.borrow_mut().print();
    }

    fn test_using_non_existing_variable(data: &Program) {
        let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));

        compile_program(Rc::clone(&diagnostics), &data.code, &data.parser);

        assert_eq!(get_value_from_key("res"), None);
        assert_eq!(
            check_for_errors_in_diagnostics(Rc::clone(&diagnostics)),
            true
        );
        assert_eq!(
            check_specific_message_in_diagnostics(
                Rc::clone(&diagnostics),
                "Name a is undefined.",
                &TextPlace::Semantic,
                &TextType::Error
            ),
            true
        );
        diagnostics.borrow_mut().print();
    }

    fn test_mixing_operators(data: &Program) {
        let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));

        compile_program(Rc::clone(&diagnostics), &data.code, &data.parser);

        assert_eq!(get_value_from_key("res"), None);
        assert_eq!(get_value_from_key("a"), None);
        assert_eq!(get_value_from_key("b"), None);
        assert_eq!(
            check_for_errors_in_diagnostics(Rc::clone(&diagnostics)),
            true
        );
        assert_eq!(
            check_specific_message_in_diagnostics(
                Rc::clone(&diagnostics),
                "Binary operator Plus is not defined for types Boolean and Integer.",
                &TextPlace::Semantic,
                &TextType::Error
            ),
            true
        );
        diagnostics.borrow_mut().print();
    }

    fn test_bad_scoping(data: &Program) {
        let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));

        compile_program(Rc::clone(&diagnostics), &data.code, &data.parser);

        assert_eq!(get_value_from_key("a"), None);
        assert_eq!(
            check_for_errors_in_diagnostics(Rc::clone(&diagnostics)),
            true
        );
        assert_eq!(
            check_specific_message_in_diagnostics(
                Rc::clone(&diagnostics),
                "Name a is already declared.",
                &TextPlace::Semantic,
                &TextType::Error
            ),
            true
        );
        diagnostics.borrow_mut().print();
    }

    fn test_good_scoping(data: &Program) {
        let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));

        compile_program(Rc::clone(&diagnostics), &data.code, &data.parser);

        assert_eq!(get_value_from_key("a"), Some(LiteralValue::Integer(4)));
        assert_eq!(
            check_for_errors_in_diagnostics(Rc::clone(&diagnostics)),
            false
        );

        diagnostics.borrow_mut().print();
    }

    fn get_value_from_key(name: &str) -> Option<LiteralValue> {
        SYMBOL_TABLE
            .lock()
            .unwrap()
            .iter()
            .find(|(symbol, _)| symbol.id() == name)
            .map(|(_, value)| value)
            .unwrap()
            .clone()
    }

    fn check_for_errors_in_diagnostics(diagnostics: Rc<RefCell<Diagnostics>>) -> bool {
        for diagnostic in &diagnostics.borrow_mut().diagnostics {
            if *diagnostic.get_type() == TextType::Error {
                return true;
            }
        }
        false
    }

    fn check_specific_message_in_diagnostics(
        diagnostics: Rc<RefCell<Diagnostics>>,
        message: &str,
        place: &TextPlace,
        text_type: &TextType,
    ) -> bool {
        for diagnostic in &diagnostics.borrow_mut().diagnostics {
            if *diagnostic.get_type() == *text_type
                && *diagnostic.get_place() == *place
                && *diagnostic.get_message() == *message
            {
                return true;
            }
        }
        false
    }
}
