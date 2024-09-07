use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(crate = "rocket::serde")]
pub enum TextType {
    Info,
    Error,
}
