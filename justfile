alias b := build
alias t := test

build:
    @cargo build

test:
    @cargo test -- --nocapture

lint:
    @cargo clippy

fmt:
    @cargo +nightly fmt
