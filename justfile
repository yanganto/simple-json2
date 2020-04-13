alias b := build
build:
	cargo remote -- build --release

alias t := test
test:
	cargo remote -- test --release

alias r := run
run:
	cargo remote -- run --release

alias c := clean
clean:
	cargo remote -- clean
