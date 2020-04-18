alias b := build
build:
	cargo build --release --no-default-features

alias t := test
test:
	cargo test --release --no-default-features

alias r := run
run:
	cargo run --release --no-default-features

alias c := clean
clean:
	cargo clean
