use std::{fmt::Display, io};

#[derive(Debug)]
pub enum Error {
    UnknownRule(String), 
    EmptyRule(String),
    Io(io::Error),
    NoRule,
    MissingRuleFile(String),
}
impl std::error::Error for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnknownRule(cmd) => write!(f, "unknown rule: {cmd:?}"),
            Error::EmptyRule(rule) => write!(f, "empty rule: {rule:} has no command"),
            Error::Io(err) => write!(f, "program error:\n{err}"),
            Error::NoRule => write!(f, "missing rule!"),
            Error::MissingRuleFile(file) => write!(f, "couldnt find rule file (looking for {file:?})"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
