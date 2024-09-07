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
    fn test_for_loop() {
        let data = Program {
            code: "{ let res = 0; for (j = 0 to 10) { res = res + j; } }".to_string(),
            parser: ParserType::Recursive,
        };
        let diagnostics = Rc::new(RefCell::new(Diagnostics::new()));

        compile_program(Rc::clone(&diagnostics), &data.code, &data.parser);

        assert_eq!(get_value_from_key("res"), Some(LiteralValue::Integer(45)));
        assert_eq!(
            check_for_errors_in_diagnostics(Rc::clone(&diagnostics)),
            false
        );

        diagnostics.borrow_mut().print();
    }

    #[test]
    fn test_for_reassignment_variable_already_declared() {
        let data = Program {
            code: "{ let a = 0; let a = 1; }".to_string(),
            parser: ParserType::Recursive,
        };
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
