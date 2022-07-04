test:
	cargo fmt --check
	cargo clippy
	cargo test

watch:
	cargo watch -x run

convert:
	sudo convert out.ppm out.png

generate:
	cargo run && sudo convert out.ppm out.png