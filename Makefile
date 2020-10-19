#@name Pretty Make
#@description An attempt to make Make pretty.

# Here we ensure that every command this Makefile run will run in a bash shell,
# instead of the default 'sh'. This is actually just a variable assignment.
SHELL := /usr/bin/env bash

.DEFAULT_GOAL := run
.PHONY: run
## Run Pretty Make on tests/fixtures/Makefile
run:
	@cargo run -- tests/fixtures/Makefile

bin/pretty-make:
	@curl -Ls https://raw.githubusercontent.com/awea/pretty-make/master/scripts/install.sh | bash -s

.PHONY: help
## List available commands
help: bin/pretty-make
	@bin/pretty-make Makefile
