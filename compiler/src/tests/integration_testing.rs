use crate::{
    generate,
    reports::{
        text_place::TextPlace,
        text_type::{self, TextType},
    },
    util::parser_type::ParserType,
    Pair, Program, Report,
};
use rocket::{http::Status, local::blocking::Client, serde::json};

#[test]
fn test_generate_for_loop() {
    let rocket = rocket::build().mount("/", rocket::routes![generate]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let data = Program {
        code: "{ let res = 0; for (j = 0 to 10) { res = res + j; } }".to_string(),
        parser: ParserType::Recursive,
    };
    let response = client.post("/generate").json(&data).dispatch();

    assert_eq!(response.status(), Status::Ok);

    let report: Report =
        json::from_str(&response.into_string().unwrap()).expect("deserialize response body");
    println!("{:?}", report);

    assert_eq!(
        check_result(
            &Pair {
                id: "res".to_string(),
                value: "45".to_string(),
            },
            &report
        ),
        true
    );
}

#[test]
fn test_generate_reassignment_variable_already_declare() {
    let rocket = rocket::build().mount("/", rocket::routes![generate]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let data = Program {
        code: "{ let a = 0; let a = 1; }".to_string(),
        parser: ParserType::Recursive,
    };
    let response = client.post("/generate").json(&data).dispatch();

    assert_eq!(response.status(), Status::Ok);

    let report: Report =
        json::from_str(&response.into_string().unwrap()).expect("deserialize response body");
    println!("{:?}", report);

    assert_eq!(
        check_result(
            &Pair {
                id: "a".to_string(),
                value: "None".to_string(),
            },
            &report
        ),
        true
    );

    assert_eq!(
        check_message(
            "Name a is already declared.",
            &TextPlace::Semantic,
            &TextType::Error,
            &report
        ),
        true
    );
}

fn check_result(pair: &Pair, report: &Report) -> bool {
    for p in &report.symbol_table {
        if *p.id == *pair.id && *p.value == *pair.value {
            return true;
        }
    }
    false
}

fn check_message(message: &str, place: &TextPlace, text_type: &TextType, report: &Report) -> bool {
    for diagnostic in &report.diagnostics {
        if *diagnostic.get_type() == *text_type
            && *diagnostic.get_place() == *place
            && *diagnostic.get_message() == *message
        {
            return true;
        }
    }
    false
}
