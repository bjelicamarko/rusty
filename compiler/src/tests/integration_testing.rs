use crate::{generate, util::parser_type::ParserType, Pair, Program, Report};
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

    let res = Pair {
        id: "res".to_string(),
        value: "45".to_string(),
    };
    assert_eq!(check_result(&res, &report), true);
}

fn check_result(pair: &Pair, report: &Report) -> bool {
    for p in &report.symbol_table {
        if *p.id == *pair.id && *p.value == *pair.value {
            return true;
        }
    }
    false
}
