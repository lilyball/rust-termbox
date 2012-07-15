
build: termbox.rs nsf
	rustc --lib -L nsf termbox.rs

nsf: nsf/libtermbox.a

nsf/libtermbox.a:
	mkdir -p nsf
	(cd nsf && curl -L https://github.com/nsf/termbox/tarball/master | tar -xz)
	(cd nsf/nsf-termbox* && make)
	rm -f nsf/libtermbox.a
	mv nsf/nsf-termbox*/libtermbox.a nsf/libtermbox.a
	rm -rf nsf/nsf-termbox*

demo: build
	rustc -L . demo.rs

doc:
	rm -f doc/*.html
	rustdoc --output-dir doc --output-format html termbox.rs

clean:
	rm -rf nsf
	rm -f libtermbox*.so
	rm -f demo
	rm -f doc/*.html

.PHONY: clean doc nsf demo
