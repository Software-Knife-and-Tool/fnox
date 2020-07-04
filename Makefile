#
#
#

all: build run

build:
	@cargo build
run:
	@cargo run
tests:
	@cargo test

clean:
	@rm -rf Cargo.lock target
