.PHONY: deps
deps: node_modules $(THEME_DIR)/vendor

node_modules: package.json yarn.lock
	@yarn install
	@touch $@

.DEFAULT_GOAL := serve
## Serve site at http://localhost:3000 with live reloading
.PHONY: serve
serve: deps
	@$(GULP) --continue

## List available commands
.PHONY: help
help:
	@bin/pretty-make
