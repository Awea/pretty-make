#@name Project name
#@description Project description.
#@color-title #70c3cc
#@color-subtitle #c970cc
#@color-link #0314fc

.PHONY: deps
deps: node_modules $(THEME_DIR)/vendor

node_modules: package.json yarn.lock
	@yarn install
	@touch $@

.PHONY: this-is-a-supa-command
## Supa dupa command description
this-is-a-supa-command:

.PHONY: test
## Test site
## and stuff
test: deps
	@test

## Another beaking the wall
another:
	@another

.PHONY: build
## Build site
build: deps
	@build

.DEFAULT_GOAL := serve
## Serve site at http://localhost:3000 with live reloading
.PHONY: serve
serve: deps
	@$(GULP) --continue

## List available commands
.PHONY: help
help:
	@bin/pretty-make Makefile
