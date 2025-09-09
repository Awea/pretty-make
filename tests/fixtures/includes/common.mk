## Common utilities from includes/common.mk
.PHONY: common-setup
common-setup:
	@echo "Setting up common utilities"

## Install dependencies
.PHONY: install-deps
install-deps:
	@echo "Installing common dependencies"

## Run common tests
.PHONY: test-common
test-common:
	@echo "Running common test suite"

## Common cleanup
.PHONY: common-clean
common-clean:
	@echo "Cleaning up common resources"
