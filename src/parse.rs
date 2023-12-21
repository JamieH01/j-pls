use std::{str::Chars, fmt::Display};

use crate::cmd::Rule;

type ParserFunc<T> = Box<dyn Fn(&mut Chars) -> Result<T, ParserError>>; 

pub struct Parser<T>(ParserFunc<T>);
impl<T> Parser<T> {
    pub fn new<F: Fn(&mut Chars) -> Result<T, ParserError> + 'static>(f: F) -> Self {
        Self(Box::new(f))
    }

    pub fn run(&self, str: &mut Chars) -> Result<T, ParserError> {
        (self.0)(str)
    }

    pub fn into_iter(self, str: String) -> ParserIter<T> {
        ParserIter(self, str)
    }

}


pub struct ParserIter<T>(Parser<T>, Vec<char>);

impl<T> Iterator for ParserIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.run(&mut self.2).ok()
    }
}

pub fn parse_pls(contents: String) -> ParserIter<Rule> { 
    let front_parser = Parser::new(|s| {
        let mut front = String::new();
        while let Some(c) = s.next() {
            if c == ':' {break}
            if c == '\n' {return Err(ParserError::EmptyRule(front))}

            front.push(c);
        }

        Ok(front)
    });

    todo!()
}

#[derive(Debug)]
pub enum ParserError {
    EmptyRule(String),
}
impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyRule(s) => write!(f, "empty rule: {s:?}"),
        }
    }
}


