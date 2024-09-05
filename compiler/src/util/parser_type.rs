use rocket::serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(crate = "rocket::serde")]
pub enum ParserType {
    Recursive,
    Glr,
}
