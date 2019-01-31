#!/usr/bin/make

install: ./target/release/scaffold-rs ./target/release/scaffold-rs.bash
	cp ./target/release/scaffold-rs /usr/bin/scaffold-rs
	cp ./target/release/scaffold-rs.bash /usr/share/bash-completion/completions/scaffold-rs

uninstall:
	rm /usr/bin/scaffold-rs
	rm /usr/share/bash-completion/completions/scaffold-rs

build:
	cargo build

release:
	cargo build --release
	@strip ./target/release/scaffold-rs

test:
	cargo test
