.PHONY: build, release, run

SHELL := /bin/bash

build:
	cargo build

release:
	cargo build --release --locked

run:
	cargo run
