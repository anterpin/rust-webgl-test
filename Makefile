all: compile optimize

compile:
	cargo build --target wasm32-unknown-unknown

optimize:
	wasm-bindgen target/wasm32-unknown-unknown/debug/engine.wasm --out-dir .