use std::{str::{Chars, Lines}, fmt::Display, iter::Peekable, error::Error, io::Write};

use owned_chars::OwnedChars;

use crate::cmd::{Rule, RunError};

pub fn rule1line(str: &str) -> Result<Rule, ParserError> {
    use ParserError as E;
    let (front, back) = str.split_once(':').ok_or(E::EmptyRule(str.to_owned()))?;
    Ok(Rule { front: front.to_owned(), back: back.to_owned() })
}

pub fn rulemultiline(lines: &mut Peekable<Lines>) -> Result<Rule, ParserError> {
    use ParserError as E;
    let first = lines.next().ok_or(E::NoElem)?;
    let mut rule = rule1line(first)?;
    rule.back = rule.back.trim().to_owned();

    while let Some(Err(_)) = lines.peek().map(|s| rule1line(s)) {
        let next = unsafe { lines.next().unwrap_unchecked() };
        if !next.is_empty() {
            if !rule.back.is_empty() { 
                rule.back.push_str(" && ");
            }
            rule.back.push_str(next.trim());
        }
    } 

    Ok(rule)
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
