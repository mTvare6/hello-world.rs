include config.mk

all:
	@echo "It takes just a afternoon to build this LLVM optimised🦀, memory safe🦀, robust🦀, minimal🦀 and configurable🦀 project, please wait for the awesomeness 🦀"
	@sleep 5
	@cargo build --release


install:
	@cp target/release/hello-world $(DESTDIR)$(PREFIX)/bin/hello-world
	@chmod 755 $(DESTDIR)$(PREFIX)/bin/hello-world

uninstall:
	@rm -f $(DESTDIR)$(PREFIX)/bin/hello-world

test: tests

tests:
	@cargo test
