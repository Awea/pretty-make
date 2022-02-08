#@name Pretty Make
#@description An attempt to make Make pretty.

# Here we ensure that every command this Makefile run will run in a bash shell,
# instead of the default 'sh'. This is actually just a variable assignment.
SHELL := /usr/bin/env bash

.DEFAULT_GOAL := pretty-help
.PHONY: pretty-help
## Run Pretty Make pretty-help on tests/fixtures/Makefile
pretty-help:
	@cargo run -- pretty-help tests/fixtures/Makefile

.PHONY: build
## Build pretty-make
build:
	@cargo build

.PHONY: empty-run
## List available commands for bin/pretty-make
run-help:
	@cargo run -- --help

bin/pretty-make:
	@curl -Ls https://raw.githubusercontent.com/awea/pretty-make/master/scripts/install.sh | bash -s

.PHONY: help
## List available commands
help: bin/pretty-make
	@bin/pretty-make pretty-help Makefile
