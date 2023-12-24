sayhi: echo "hai!"
build: cargo build

# install local binary
install-local: 
    cargo build 
    cargo install --path . --force

info: bat Cargo.toml

publish:
    cargo publish --dry-run
    git push
    cargo publish
