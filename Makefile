RUSTC ?= rustc

LIBSRC=$(wildcard src/**/*.rs)
TESTS=$(wildcard test/*.rs test/**/*.rs)

# ------------------
# Internal variables
dummy1 := $(shell mkdir bin 2> /dev/null)

all: ract test

ract: lib bin/ract

lib: src/ract.rs $(LIBSRC)
	$(RUSTC) --out-dir bin -O src/ract.rs

bin/ract: src/main.rs
	$(RUSTC) -L bin -o bin/ract $<

test: bin/test
	./bin/test

bin/test: lib $(TESTS)
	$(RUSTC) --test -L bin -o $@ test/tests.rs

.PHONY : clean
clean:
	rm -rf bin