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

mod error;
use error::*;

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

fn run() -> Result<Child, Error> {
    let str = fs::read_to_string("rules.pls").map_err(|_| Error::MissingRuleFile("rules.pls".into()))?;
    let cmd = env::args().nth(1).ok_or(Error::NoRule)?;
    
    for line in str.lines().filter(|s| !s.is_empty()) {
        let (front, back) = line
            .split_once(':').ok_or(Error::EmptyRule(line.to_owned()))?
            .map(str::trim);
         
        if cmd == front {
            return Ok(Command::new("bash")
                .arg("-c")
                .arg(back)
                .spawn()?);
        }
    }

    Err(Error::UnknownRule(cmd))
}


trait TupleExtensions<T> {
    fn map<F: Fn(T) -> T>(self, f:F) -> Self;
}

impl<T> TupleExtensions<T> for (T, T) {
    fn map<F: Fn(T) -> T>(self, f:F) -> Self {
        (f(self.0), f(self.1))
    }
}


