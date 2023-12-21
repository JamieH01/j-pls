use std::{fmt::Display, io, process::{Child, Command}, fs, env};

use crate::parse::{parse_pls, ParserError};

pub struct Rule {
    front: String,
    back: String,
}
impl Rule {
    pub fn run(&self) -> Result<Child, io::Error> {
    Command::new("bash")
            .arg("-c")
            .arg(self.back)
            .spawn()
    }
}

pub fn run() -> Result<Child, RunError> {
    let str = fs::read_to_string("rules.pls").map_err(|_| RunError::MissingRuleFile("rules.pls".into()))?;
    let cmd = env::args().nth(1).ok_or(RunError::NoRule)?;
    
    for cmd in parse_pls(str) {
         
    }

    Err(RunError::UnknownRule(cmd))
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
