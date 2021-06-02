#@name Project name
#@description Project description.
#@color-title #70c3cc
#@color-subtitle #c970cc
#@color-link #0314fc

# Here we ensure that every command this Makefile run will run in a bash shell,
# instead of the default 'sh'. This is actually just a variable assignment.
SHELL := /usr/bin/env bash

WEBPACK        := $(PWD)/node_modules/.bin/webpack
WEBPACK_SERVER := $(PWD)/node_modules/.bin/webpack-dev-server

.PHONY: deps
deps: node_modules api/vendor

node_modules: package.json yarn.lock
	@yarn install
	@touch $@

api/vendor: api/composer.json api/composer.lock
	@cd api ; composer install
	touch $@

## Do something fancy
$(shell foo)/fancy/path/recipe:
	touch $@

.PHONY: 2020
## Reflect on last year
2020:
	echo "Last year"

# ✨ Here is a section
# --------------
#
# Section dedicated to dependencies related target:
# - .mk folder
# - bin folder
# - bin/mitogen

.PHONY: docker-compose
## Up containers
docker-compose:
	docker-compose up -d

# This is a comment and it won't appear in the `make help`.

.DEFAULT_GOAL := serve
## Serve:
## - Site at http://localhost:3000 with hot reloading
## - API at http://localhost:3010
## - phpMyAdmin at http://localhost:3011
.PHONY: serve
serve: api deps
	@$(WEBPACK_SERVER) --inline --progress --config webpack/dev.js

.PHONY: api
api:
	@docker-compose up -d

## Build site for production use
.PHONY: build
build: deps
	@echo "Building front-end"
	@rm -rf site/*
	@NODE_ENV=production $(WEBPACK) --config webpack/prod.js
	@echo "Front-end built!"

# 💡 Here is another section
# --------------
#
# Section dedicated to dependencies related target:
# - .mk folder
# - bin folder
# - bin/mitogen

## Deploy site to https://a-super-website.sub.domain.com
deploy_staging: build
	@echo "Deploy to a-super-website.sub.domain.com"
	@rsync -avz --delete-before source target

## Deploy site to https://a-super-website.fr
deploy_prod: build
	@echo "Deploy to a-super-website.fr"
	@rsync -avz --delete-before source target

bin/pretty-make:
	@bash <(curl -Ls https://raw.githubusercontent.com/awea/pretty-make/master/scripts/install.sh)

.PHONY: help
## List available commands
help: bin/pretty-make
	@bin/pretty-make Makefile
