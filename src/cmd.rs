use std::{fmt::Display, io, process::{Child, Command}, fs, env};

use crate::{parse::{ParserError, rule1line, rulemultiline, clear_between}, config::CONFIG};

pub struct Rule {
    pub front: String,
    pub back: String,
}
impl Rule {
    pub fn run(&self) -> Result<Child, io::Error> {
    Command::new("bash")
            .arg("-c")
            .arg(&self.back)
            .spawn()
    }
}

pub fn run(cmd: &str) -> Result<Child, RunError> {

    for rule in get_rules()? {
        if rule.front == cmd {return Ok(rule.run()?)}
    }

    Err(RunError::UnknownRule(cmd.to_string()))
}

pub fn get_rules() -> Result<Vec<Rule>, RunError> {
    let mut out = vec![];

    if let Ok(str) = fs::read_to_string(CONFIG.look()) { 
        out.append(&mut parse_file(str)?);
    }

    if let Ok(str) = fs::read_to_string(CONFIG.global()) { 
        out.append(&mut parse_file(str)?);
    }

    Ok(out)
}

fn parse_file(mut str: String) -> Result<Vec<Rule>, RunError> {
    str = clear_between(str, '#', '\n');
    let mut iter = str.lines().peekable();
    let mut out = vec![];
    while iter.peek().is_some() {
        match rulemultiline(&mut iter) {
            Ok(rule) => out.push(rule),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(out)
}

#[derive(Debug)]
pub enum RunError {
    UnknownRule(String), 
    Io(io::Error),
    NoRule,
    MissingRuleFile(String),
    ParserError(ParserError),
}

impl std::error::Error for RunError {}
impl Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownRule(cmd) => write!(f, "unknown rule: {cmd:?}"),
            Self::Io(err) => write!(f, "program error:\n{err}"),
            Self::NoRule => write!(f, "missing rule!"),
            Self::MissingRuleFile(path) => write!(f, "missing rule file (looking for {path:?}"),
            Self::ParserError(err) => write!(f, "{err}"),
        }
    }
}

impl From<io::Error> for RunError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ParserError> for RunError {
    fn from(value: ParserError) -> Self {
        Self::ParserError(value)
    }
}
