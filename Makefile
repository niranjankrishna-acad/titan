# Variables
BINARY_NAME=titan
BINARY_PATH=target/release/$(BINARY_NAME)
CHECK_INTERVAL ?= 5

# Compile and run the program in development mode
dev:
	cargo run  -- -c $(CHECK_INTERVAL)

# Build the program as a packaged binary and set up as a cron job
build: compile setup-cron

compile:
	cargo build --release -- -c $(CHECK_INTERVAL)

# Set up the program to run 5 minutes from now and then at every subsequent startup
setup-cron:
	@echo "@reboot $(PWD)/$(BINARY_PATH)" | crontab -
	echo "$(PWD)/$(BINARY_PATH)" | at now + 5 minutes

uninstall:
	@pkill -f $(BINARY_NAME) || true
	@crontab -l | grep -v "$(PWD)/$(BINARY_PATH)" | crontab -

all: dev

.PHONY: dev compile build setup-cron
