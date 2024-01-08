use std::{fmt::Display, io, process::{Command, ExitStatus}, fs, os::unix::process::ExitStatusExt, cmp::Ordering};

use crate::{parse::{ParserError, rulemultiline, clear_between}, config::CONFIG};

#[derive(Debug)]
pub struct Rule {
    pub front: String,
    pub back: Vec<String>,
    pub args: Vec<String>,
    pub global: bool,
}
impl Rule {
    pub fn run(&self, args: Vec<String>) -> Result<ExitStatus, io::Error> {
    for cmd in &self.back {
        let mut proc = Command::new("bash");
        proc.arg("-c").arg(cmd);

        for (var, val) in self.args.iter().zip(args.iter()) {
            proc.env(var, val);
        }

        let res = proc.status()?;
        if !res.success() { return Ok(res) }
    }

    Ok(ExitStatus::from_raw(0))
    }
}



pub fn run(cmd: &str, args: Vec<String>) -> Result<ExitStatus, RunError> {

    for rule in get_rules()? {
        if rule.front == cmd {
            match rule.args.len().cmp(&args.len()) {
                Ordering::Less => return Err(RunError::TooManyArgs),
                Ordering::Greater => return Err(RunError::NotEnoughArgs),
                _ => {},
            }

            return Ok(rule.run(args)?)
        }
    }

    Err(RunError::UnknownRule(cmd.to_string()))
}

pub fn get_rules() -> Result<Vec<Rule>, RunError> {
    let mut out = vec![];

    if let Ok(str) = fs::read_to_string(CONFIG.look()) { 
        out.append(&mut parse_file(str, false)?);
    }

    if let Ok(str) = fs::read_to_string(CONFIG.global()) { 
        out.append(&mut parse_file(str, true)?);
    }

    Ok(out)
}

fn parse_file(mut str: String, global: bool) -> Result<Vec<Rule>, RunError> {
    str = clear_between(str, '#', '\n');
    let mut iter = str.lines().peekable();
    let mut out = vec![];
    while iter.peek().is_some() {
        match rulemultiline(&mut iter, global) {
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
    TooManyArgs,
    NotEnoughArgs,
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
            Self::TooManyArgs => write!(f, "too many args provided"),
            Self::NotEnoughArgs => write!(f, "not enough args provided"),
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
