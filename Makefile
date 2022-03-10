copy:
	wasm-pack build
	rm -r example/node_modules/fourier-animation
	mv pkg example/node_modules/fourier-animation