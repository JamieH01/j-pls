use std::{str::{Chars, Lines}, fmt::Display, iter::Peekable, error::Error, io::Write};

use crate::cmd::{Rule, RunError};

pub fn rule1line(str: &str, global: bool) -> Result<Rule, ParserError> {
    use ParserError as E;
    let (front, back) = str
        .split_once(':')
        .ok_or(E::EmptyRule(str.to_owned()))?;

    let (front, args) = parse_args(front)?;
    

    Ok(Rule { 
        front, 
        back: vec![back.to_owned()], 
        global, 
        args 
    })
}

fn parse_args(inp: &str) -> Result<(String, Vec<String>), ParserError> {
    let mut args = vec![];
    let name: &str;
    if let Some((l, r)) = inp.split_once('[') {
        if r.chars().last() != Some(']') { 
            return Err(ParserError::UnclosedArgument(l.to_owned())) 
        }
        args = r.split(',').map(str::trim).map(String::from).collect();

        args.last_mut().map(|a| a.pop()); 

        name = l.trim();
    } else {
        name = inp.trim();
    } 

    Ok((name.to_owned(), args))
}

pub fn rulemultiline(lines: &mut Peekable<Lines>, global: bool) -> Result<Rule, ParserError> {
    use ParserError as E;
    let first = lines.next().ok_or(E::NoElem)?;
    let mut rule = rule1line(first, global)?;

    while let Some(Err(_)) = lines.peek().map(|s| rule1line(s, global)) {
        let next = unsafe { lines.next().unwrap_unchecked() };
        if !next.is_empty() {
            rule.back.push(next.trim().to_string());
        }
    } 

    Ok(rule)
}


pub fn clear_between(str: String, front: char, back: char) -> String {
    let mut out = String::with_capacity(str.len());
    let mut push = true;

    for c in str.chars() {
        if c == front { push = false } 
        if c == back { push = true } 
        if push { out.push(c) }

    }

    out
}



#[derive(Debug, Clone)]
pub enum ParserError {
    EmptyRule(String),
    UnclosedArgument(String),
    NoElem,
}
impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyRule(s) => write!(f, "empty rule: {s:?}"),
            Self::NoElem => write!(f, "expected element"),
            Self::UnclosedArgument(s) => write!(f, "unclosed argument list for rule {s:?}"),
        }
    }
}
