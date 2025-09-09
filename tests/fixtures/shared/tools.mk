## Development tools from shared/tools.mk
.PHONY: lint
lint:
	@echo "Running code linters and style checks"

## Format code automatically
.PHONY: format
format:
	@echo "Auto-formatting code with standard tools"

## Generate documentation
.PHONY: docs
docs:
	@echo "Generating project documentation"

## Run security audit
.PHONY: security-audit
security-audit:
	@echo "Running security vulnerability scan"

include vendor.mk

## Development server for testing
.PHONY: dev-server
dev-server:
	@echo "Starting development server with hot reload"

## Profile application performance
.PHONY: profile
profile:
	@echo "Running performance profiling tools"
