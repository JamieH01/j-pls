use std::error::Error;
use std::{env, marker::PhantomData, process::Termination};
use std::fmt::Debug;

mod parse;

mod cmd;
use cmd::*;
use colored::Colorize;

mod config;
use config::*;

fn main() {
    let mut iter = env::args();
    let rule = iter.nth(1);

    let args: Vec<String> = iter.collect();
    let ruleset = match get_rules() {
        Ok(it) => it,
        Err(e) => {println!("oops! {e}"); return;},
    };
    
    match rule {
        None => {
            if let Some(cmd) = ruleset.do_cmd {
                //run do command
                let res = cmd.run(args);
                match res {
                    Ok(s) if s.success() => {},
                    Ok(s) => println!("process exited with code: {:?}", s.code()),
                    Err(e) => println!("oops! {e}"),
                }

            } else {
                if CONFIG.rich {
                    show_rules_rich(ruleset);
                } else {
                    show_rules(ruleset);
                }
            }
        },
        Some(cmd) if cmd == "-l" => {
            if CONFIG.rich {
                show_rules_rich(ruleset);
            } else {
                show_rules(ruleset);
            }
        }
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

fn show_rules(ruleset: Ruleset) {
    println!("{}", "available commands:".blue());

    if let Some(cmd) = ruleset.do_cmd {
        println!("{}", cmd.front.yellow()); 
    }

    for rule in ruleset.local_rules {
        println!("{}", rule.front.green()); 
    }

    for rule in ruleset.global_rules {
        println!("{}", rule.front.red()); 
    }
}

macro_rules! pretty_print {
    ($cmd: tt, $clr: ident) => {
        print!("{}", $cmd.front.$clr()); 

        if $cmd.args.len() > 0 {
            print!(" {}", format!("({})", $cmd.args.len()).magenta()); 
        } 

        print!(": {}", $cmd.back[0]);

        if $cmd.back.len() > 1 {
            print!(" {}", format!("(+{})", $cmd.back.len()-1).dimmed().italic());
        } 

        print!("\n");
    };
}

fn show_rules_rich(ruleset: Ruleset) {
    println!("{}", "available commands:".blue());

    if let Some(cmd) = ruleset.do_cmd {
        pretty_print!(cmd, yellow);
    }

    for rule in ruleset.local_rules {
        pretty_print!(rule, green);
    }

    for rule in ruleset.global_rules {
        pretty_print!(rule, red);
    }
}
