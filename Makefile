check lint:
	cargo clippy 

fix:
	cargo clippy --fix --allow-dirty

fmt:
	cargo +nightly fmt


