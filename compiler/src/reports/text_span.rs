use std::fmt::{Debug, Formatter, Result};

use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct TextSpan {
    start: usize,
    length: usize,
}

impl TextSpan {
    pub fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }

    fn end(&self) -> usize {
        self.start + self.length
    }
}

impl Debug for TextSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{ start: {}, end: {} }}", self.start, self.end())
    }
}
