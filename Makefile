#
#
#
RUNPATH = ./target/debug/fnox

all: build

build:
	@cargo build

debug: 
	@cargo run

fmt:
	@cargo fmt

run:
	@$(RUNPATH)

tests:
	-cargo test

clean:
	@rm -rf Cargo.lock target
