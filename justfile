alias b := build
build:
	cargo remote -- build --release --no-default-features

alias t := test
test:
	cargo remote -- test --release --no-default-features

alias r := run
run:
	cargo remote -- run --release --no-default-features

alias c := clean
clean:
	cargo remote -- clean
