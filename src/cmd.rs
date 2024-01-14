use std::{fmt::Display, io, process::{Command, ExitStatus}, fs, os::unix::process::ExitStatusExt, cmp::Ordering};

use crate::{parse::{ParserError, rulemultiline, clear_between}, config::CONFIG};

//refactor rule logic into this type
#[derive(Debug)]
pub struct Ruleset {
    pub local_rules: Vec<Rule>,
    pub global_rules: Vec<Rule>,
    pub do_cmd: Option<Rule>,
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rule {
    pub front: String,
    pub back: Vec<String>,
    pub args: Vec<String>,
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



//TODO: refactor use [Ruleset]
pub fn run(cmd: &str, args: Vec<String>) -> Result<ExitStatus, RunError> {
    let set = get_rules()?;

    for rule in set.local_rules {
        if rule.front == cmd {
            match rule.args.len().cmp(&args.len()) {
                Ordering::Less => return Err(RunError::TooManyArgs),
                Ordering::Greater => return Err(RunError::NotEnoughArgs),
                _ => {},
            }

            return Ok(rule.run(args)?)
        }
    }
    for rule in set.global_rules {
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

pub fn get_rules() -> Result<Ruleset, RunError> {
    let mut rules = Ruleset { local_rules: vec![], global_rules: vec![], do_cmd: None };



    if let Ok(str) = fs::read_to_string(CONFIG.look()) { 
        rules.local_rules.append(&mut parse_file(str, false)?);
    }

    if let Ok(str) = fs::read_to_string(CONFIG.global()) { 
        rules.global_rules.append(&mut parse_file(str, true)?);
    }

    if let Some((do_cmd, i)) = find(&rules.local_rules, |r| {r.front == "do"}) {
        rules.do_cmd = Some(do_cmd.clone()); 
        rules.local_rules.remove(i);
    }

    Ok(rules)
}

fn find<T>(vec: &Vec<T>, pred: impl Fn(&T)->bool) -> Option<(&T, usize)> {
    for (i, e) in vec.iter().enumerate() {
        if pred(e) == true { return Some((e, i)) };
    }
    None
}

//idky this returns a run error but im too sick to care rn
fn parse_file(mut str: String, global: bool) -> Result<Vec<Rule>, RunError> {
    str = clear_between(str, '#', '\n');
    let mut iter = str.lines().peekable();
    let mut out = vec![];
    while iter.peek().is_some() {
        match rulemultiline(&mut iter, global) {
            Ok(rule) => {
                if find(&out, |r: &Rule| {r.front == rule.front}).is_some() {
                    return Err(RunError::DuplicateRule(rule.front))
                }
                out.push(rule)
            },
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
    DuplicateRule(String),
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
            Self::DuplicateRule(cmd) => write!(f, "multiple instances of rule {cmd:?}"),
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
