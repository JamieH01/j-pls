# plssssssssss

`pls` is a scripting utility to create local messy aliases quickly.
remember that pls is not finished and has many features planned!

## Installation
`pls` is available on crates.io as `j-pls`.
```bash
cargo install j-pls
```

## Usage
`pls` looks for a `rules.pls` file to pull rules from. Rule syntax looks like this:
```
build: cargo build --release 
run: cargo run

install-local: 
    cargo build 
    cargo install --path . --force
```

To use a rule, call it with `pls`!
```bash
pls build
```

## Todo 
Current features being worked on, in order of priority.
- [x] multiline commands
- [ ] global rules
- [ ] arguments 
- [ ] subcommands
- [ ] confirm prompts
- more advanced scripting..(?)

