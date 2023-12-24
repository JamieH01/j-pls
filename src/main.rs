use std::process::{Command, Child, ExitStatus};
use std::{fs, env};

mod parse;

mod cmd;
use cmd::*;
use colored::Colorize;

mod config;

fn main() {
    match env::args().nth(1) {
        None => {
            let cmds = match get_rules() {
                Ok(c) => c,
                Err(e) => {println!("oops! {e}"); return;},
            };

            println!("{}", "available commands:".blue());
            for rule in cmds {
                if rule.global {
                    println!("{}", rule.front.red()); 
                } else {
                    println!("{}", rule.front.green()); 
                }
            }
        },
        Some(cmd) => {
            let res = run(&cmd).map(|mut p| p.wait());
            handle(res);
        },
    }
}

fn handle(res: Result<Result<ExitStatus, std::io::Error>, RunError>) {
    match res {
        Ok(Ok(s)) if s.success() => {},
        Ok(Ok(s)) => match s.code() {
            Some(i) => println!("process exited with code: {i}"),
            None => println!("process terminated"),
        },
        Ok(Err(e)) => println!("oops! {e}"),
        Err(e) => println!("oops! {e}"),
    }
}

