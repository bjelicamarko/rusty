use crate::{
    generate,
    reports::{text_place::TextPlace, text_type::TextType},
    util::parser_type::ParserType,
    Pair, Program, Report,
};
use rocket::{http::Status, local::blocking::Client, serde::json};

#[test]
#[ignore]
pub fn api_test_suit() {
    test_generate_variables_assignments();
    test_generate_for_loop();
    test_generate_while_loop();
    test_generate_reassignment_variable_already_declare();
    test_generate_if_statement();
    test_generate_else_statement();
    test_generate_const_reassignment();
    test_generate_assignment_without_declaration();
    test_generate_using_non_existing_variable();
}

#[test]
#[ignore]
fn test_generate_variables_assignments() {
    let rocket = rocket::build().mount("/", rocket::routes![generate]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let data = Program {
        code: "{ let a = 3; const b = 4; let res = 3 + a * b;  }".to_string(),
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
                value: "3".to_string(),
            },
            &report
        ),
        true
    );
    assert_eq!(
        check_result(
            &Pair {
                id: "b".to_string(),
                value: "4".to_string(),
            },
            &report
        ),
        true
    );
    assert_eq!(
        check_result(
            &Pair {
                id: "res".to_string(),
                value: "15".to_string(),
            },
            &report
        ),
        true
    );
}

#[test]
#[ignore]
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
#[ignore]
fn test_generate_while_loop() {
    let rocket = rocket::build().mount("/", rocket::routes![generate]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let data = Program {
        code: "{ let res = 0; while (res < 5) { res = res + 1; } }".to_string(),
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
                value: "5".to_string(),
            },
            &report
        ),
        true
    );
}

#[test]
#[ignore]
fn test_generate_if_statement() {
    let rocket = rocket::build().mount("/", rocket::routes![generate]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let data = Program {
        code: "{ let a = 3; if (a == 3) { a = 4; } }".to_string(),
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
                value: "4".to_string(),
            },
            &report
        ),
        true
    );
}

#[test]
#[ignore]
fn test_generate_else_statement() {
    let rocket = rocket::build().mount("/", rocket::routes![generate]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let data = Program {
        code: "{ let a = 3; if (a != 3) { a = 4; } else { a = 5; } }".to_string(),
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
                value: "5".to_string(),
            },
            &report
        ),
        true
    );
}

#[test]
#[ignore]
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

#[test]
#[ignore]
fn test_generate_const_reassignment() {
    let rocket = rocket::build().mount("/", rocket::routes![generate]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let data = Program {
        code: "{ const a = 3; a = 4; }".to_string(),
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
            "Const a cannot be redefined.",
            &TextPlace::Semantic,
            &TextType::Error,
            &report
        ),
        true
    );
}

#[test]
#[ignore]
fn test_generate_assignment_without_declaration() {
    let rocket = rocket::build().mount("/", rocket::routes![generate]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let data = Program {
        code: "{ a = 4; }".to_string(),
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
            "Name a is not declared yet.",
            &TextPlace::Semantic,
            &TextType::Error,
            &report
        ),
        true
    );
}

#[test]
#[ignore]
fn test_generate_using_non_existing_variable() {
    let rocket = rocket::build().mount("/", rocket::routes![generate]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let data = Program {
        code: "{ let res = 3 + a; }".to_string(),
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
                value: "None".to_string(),
            },
            &report
        ),
        true
    );

    assert_eq!(
        check_message(
            "Name a is undefined.",
            &TextPlace::Semantic,
            &TextType::Error,
            &report
        ),
        true
    );
}

#[test]
#[ignore]
fn test_generate_bad_scoping() {
    let rocket = rocket::build().mount("/", rocket::routes![generate]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let data = Program {
        code: "{ let a = true; { let b = 3; { let a = 3; } } }".to_string(),
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

#[test]
#[ignore]
fn test_generate_good_scoping() {
    let rocket = rocket::build().mount("/", rocket::routes![generate]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let data = Program {
        code: "{ let a = 0; { let b = 3; a = b; } { let b = 4; a = b; } }".to_string(),
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
                value: "4".to_string(),
            },
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
