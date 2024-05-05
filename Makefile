build:
	cd wasm && wasm-pack build --target web

make up:
	pnpm dev

clean:
	rm -rf pkg
	rm -rf target
