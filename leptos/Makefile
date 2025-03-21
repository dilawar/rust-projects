all:
	trunk serve 

fmt:
	# cargo +nightly fmt
	leptosfmt src

check:
	cargo clippy --no-deps -- -Dwarnings

fix: fmt
	cargo clippy --fix --allow-dirty 

css:
	stylance . --output-file app.css
