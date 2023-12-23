use std::{str::Chars, fmt::Display, iter::Peekable, error::Error, io::Write};

use owned_chars::OwnedChars;

use crate::cmd::{Rule, RunError};

pub fn rule1line(str: &str) -> Result<Rule, ParserError> {
    use ParserError as E;
    let (front, back) = str.split_once(':').ok_or(E::EmptyRule(str.to_owned()))?;
    Ok(Rule { front: front.to_owned(), back: back.to_owned() })
}

#[derive(Debug, Clone)]
pub enum ParserError {
    EmptyRule(String),
    NoElem,
}
impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyRule(s) => write!(f, "empty rule: {s:?}"),
            Self::NoElem => write!(f, "expected element"),
        }
    }
}
