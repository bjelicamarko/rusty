use rocket::serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub enum TextPlace {
    Syntax,
    Lexical,
    Semantic,
}
