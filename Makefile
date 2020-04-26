all: web
	cargo run --release

desktop:
	cargo run

web:
	wasm-pack build --target web --out-name web --out-dir pkg