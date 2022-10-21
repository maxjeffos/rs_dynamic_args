# Build system related variables
GOCMD = go
GOOS = $(shell go env GOOS)
GOARCH = $(shell go env GOARCH)

# Project related variables
WORKING_DIR = $(CURDIR)
BUILD_DIR = $(WORKING_DIR)/_bin

# build related variables
_SEPARATOR = _
_EXE_POSTFIX = 

ifeq ($(GOOS),windows)
	_EXE_POSTFIX = .exe
endif

# some globally assembled variables
APPLICATION_NAME = sclix-woof
PLATFORM_STRING = $(GOOS)$(_SEPARATOR)$(GOARCH)
EXECUTABLE_NAME = $(APPLICATION_NAME)$(_SEPARATOR)$(PLATFORM_STRING)$(_EXE_POSTFIX)

# some make file variables
LOG_PREFIX = --

.PHONY: clean
clean:
	@$(GOCMD) clean
	@rm -f -r $(BUILD_DIR)

.PHONY: format
format:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy

.PHONY: build
build:
	@echo "Building..."
	@cargo build

dev_deploy/extensions/x_woof:
	@mkdir -p dev_deploy/extensions/x_woof

dev_deploy/extensions/x_dg:
	@mkdir -p dev_deploy/extensions/x_dg

dev_deploy/extensions/x_woof/x_woof_darwin_arm64: dev_deploy/extensions/x_woof
	@cp target/debug/x_woof dev_deploy/extensions/x_woof/x_woof_darwin_arm64
	@cp x_woof/extension.json dev_deploy/extensions/x_woof/extension.json

dev_deploy/extensions/x_dg/x_dg_darwin_arm64: dev_deploy/extensions/x_dg
	@cp target/debug/x_dg dev_deploy/extensions/x_dg/x_dg_darwin_arm64
	@cp x_dg/extension.json dev_deploy/extensions/x_dg/extension.json

.PHONY: deploy_ext_woof
deploy_ext_woof: dev_deploy/extensions/x_woof/x_woof_darwin_arm64

.PHONY: deploy_ext_dg
deploy_ext_dg: dev_deploy/extensions/x_dg/x_dg_darwin_arm64

.PHONY: deploy_clean
deploy_clean:
	@rm -rf dev_deploy

.PHONY: test
test: 
	@echo "Testing..."

.PHONY: help
help:
	@echo "Main targets:"
	@echo "$(LOG_PREFIX) format"
	@echo "$(LOG_PREFIX) deploy_clean"
	@echo "$(LOG_PREFIX) deploy_ext_woof"
	@echo "$(LOG_PREFIX) deploy_ext_dg"
	@echo "$(LOG_PREFIX) lint"
	@echo "$(LOG_PREFIX) build"
	@echo "$(LOG_PREFIX) test"
	@echo "$(LOG_PREFIX) clean"
	@echo "\nAvailable parameter:"
	@echo "$(LOG_PREFIX) GOOS                       Specify Operating System to compile for (see golang GOOS, default=$(GOOS))"
	@echo "$(LOG_PREFIX) GOARCH                     Specify Architecture to compile for (see golang GOARCH, default=$(GOARCH))"
