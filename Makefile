all: build examples doc

build: nsf termbox.rs 
	rustc --lib -L nsf termbox.rs

nsf: nsf/libtermbox.a

nsf/libtermbox.a:
	mkdir -p nsf
	(cd nsf && curl -L https://github.com/nsf/termbox/tarball/master | tar -xz)
	(cd nsf/nsf-termbox* && make)
	rm -f nsf/libtermbox.a
	mv nsf/nsf-termbox*/libtermbox.a nsf/libtermbox.a
	rm -rf nsf/nsf-termbox*

examples: examples/hello examples/demo
	
examples/hello: build examples/hello.rs
	(cd examples && rustc -L .. hello.rs)

examples/demo: build examples/demo.rs
	(cd examples && rustc -L .. demo.rs)

doc:
	rm -f doc/*.html
	rustdoc --output-dir doc --output-format html termbox.rs

clean:
	rm -rf nsf
	rm -f libtermbox*.so
	rm -f demo
	rm -f doc/*.html

.PHONY: clean doc nsf examples
