#
#
#
CMD = ./target/debug/lispox

all: build

build:
	@cargo build

debug: 
	@cargo run

fmt:
	@cargo fmt

run:
	@$(CMD)

tests:
	@cargo test

clean:
	@rm -rf Cargo.lock target
