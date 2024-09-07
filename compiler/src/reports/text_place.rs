use rocket::serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(crate = "rocket::serde")]
pub enum TextPlace {
    Syntax,
    Lexical,
    Semantic,
}
