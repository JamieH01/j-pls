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

# install binary locally
install-local: 
    cargo build 
    cargo install --path . --force
```

Run `pls` with no arguments to view available rules.
To use a rule, call it with `pls`!
```bash
pls build
```

## Configuration
`pls` looks in `$XDG_CONFIG/pls` for configuration files.
```
# file: $XDG_CONFIG/pls/config.pls
# these are available settings with their defaults

# where pls will look for local rules, relative to cwd
look: rules.pls

# where pls will look for global rules, relative to $XDG_CONFIG/pls/ 
global: global.pls
```

## Todo 
Current features being worked on, in order of priority.
- [x] multiline commands
- [x] global rules
- [ ] arguments 
- [ ] subcommands
- [ ] confirm prompts
- more advanced scripting..(?)

