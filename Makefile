#
#
#

all: clean build run

build:
	cargo build
run:
	cargo run
clean:
	@rm -rf Cargo.lock target
