use std::env;

mod parse;

mod cmd;
use cmd::*;
use colored::Colorize;

mod config;

fn main() {
    let mut iter = env::args();
    let rule = iter.nth(1);

    let args: Vec<String> = iter.collect();
    
    match rule {
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
            let res = run(&cmd, args);
            match res {
                Ok(s) if s.success() => {},
                Ok(s) => println!("process exited with code: {:?}", s.code()),
                Err(e) => println!("oops! {e}"),
            }
        },
    }
}


