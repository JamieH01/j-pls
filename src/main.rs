/*
orders.pls

#normal
build: cargo build
build_then_run: (
    cargo build
    ./target/debug/pls
)

run: cargo run
| release: cargo run --release

*/
use std::process::{Command, Child, ExitStatus};
use std::{fs, env};

mod parse;
use parse::*;

mod cmd;
use cmd::*;

fn main() {
    let res = run().map(|mut p| p.wait());
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
