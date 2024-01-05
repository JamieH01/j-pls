use std::env;

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
            let res = run(&cmd);
            match res {
                Ok(s) if s.success() => {},
                Ok(s) => println!("process exited with code: {:?}", s.code()),
                Err(e) => println!("oops! {e}"),
            }
        },
    }
}


