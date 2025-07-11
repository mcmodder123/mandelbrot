CC = cargo
CFLAGS =  build --release

mandelbrot: src/main.rs
	$(CC) $(CFLAGS)
	cp ./target/release/mandelbrot ./mandelbrot

.PHONY: install
install:
	cp mandelbrot /usr/local/bin/mandelbrot
