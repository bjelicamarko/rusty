use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum ParserType {
    Recursive,
    Lr,
}
