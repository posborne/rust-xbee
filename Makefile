RUSTC ?= rustc
RUSTFLAGS ?=

RUST_SRC=$(shell find . -type f -name '*.rs')

.PHONY: all
all: xbeetest

xbeetest: $(RUST_SRC)
	$(RUSTC) $(RUSTFLAGS) $< -o $@
	touch $@

.PHONY: clean
clean:
	rm -f *.o *.a *.so *.dylib *.dll *.dummy *-test xbeetest
