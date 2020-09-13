#
#
#
CMD = ./target/debug/lispox

all: build run

build:
	@cargo build

debug: 
	@cargo run

run:
	@$(CMD)

tests:
	@cargo test

install:

clean:
	@rm -rf Cargo.lock target
