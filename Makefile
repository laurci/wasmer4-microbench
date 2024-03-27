.PHONY: wasm

wasm:
	cd wasm && cargo build --target wasm32-unknown-unknown --release