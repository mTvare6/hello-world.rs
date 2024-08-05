include config.mk

all:
	@echo "It takes just an afternoon to build this LLVM optimised🚀, memory safe🚀, robust🚀, minimal🚀 and configurable🚀 project, please wait for the awesomeness 🚀"
	@sleep 5
	@cargo build --release

clean: ./target
	@rm -rf -- target
 
install: ./target/release/hello-world
	@install -Dm755 -- ./target/release/hello-world $(DESTDIR)$(PREFIX)/bin/hello-world

uninstall: $(DESTDIR)$(PREFIX)/bin/hello-world
	@rm -- $(DESTDIR)$(PREFIX)/bin/hello-world

test: tests

tests:
	@cargo test
