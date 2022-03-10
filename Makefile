copy:
	wasm-pack build
	rm -rf example/node_modules/fourier-animation
	mv pkg example/node_modules/fourier-animation