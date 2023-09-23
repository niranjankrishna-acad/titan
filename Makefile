# Variables
BINARY_NAME=titan
BINARY_PATH=target/release/$(BINARY_NAME)

# Compile and run the program in development mode
dev:
	cargo run

# Build the program as a packaged binary
build:
	cargo build --release

# Set up the program as a cron job to run at startup indefinitely
setup-cron:
	@echo "@reboot sleep 300 && $(PWD)/$(BINARY_PATH)" | crontab -

# Default target (if you just run `make` without arguments)
all: dev

.PHONY: dev build setup-cron
