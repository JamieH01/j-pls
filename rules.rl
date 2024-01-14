do: echo "i did it!"

sayhi: echo "hai!"
build: cargo build

# install local binary
install-local: 
    cargo build 
    cargo install --path . --force

info: bat Cargo.toml
