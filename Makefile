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

.PHONY: template
## 🚧 Run Pretty Make template
template:
	@cargo run -- template $(PWD)/rust-logo-512x512.png https://gist.githubusercontent.com/Awea/5db1726abca9d731428491c67ccedc85/raw/b4a195d7d079a3f48a84260fade2e03c78f23526/Makefile

.PHONY: template-help
## List available command for Pretty Make template
template-help:
	@cargo run -- help template

.PHONY: empty-run
## List available commands for bin/pretty-make
run-help:
	@cargo run -- --help

bin/pretty-make:
	@curl -Ls https://raw.githubusercontent.com/awea/pretty-make/master/scripts/install.sh | bash -s

.PHONY: help
## List available commands
help: bin/pretty-make
	@bin/pretty-make Makefile
