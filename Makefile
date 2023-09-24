# Variables
BINARY_NAME=titan
BINARY_PATH=target/release/$(BINARY_NAME)

# Compile and run the program in development mode
dev:
	cargo run

# Build the program as a packaged binary and set up as a startup service
build: compile setup-service

compile:
	cargo build --release 

# Set up the program to run as a startup service
setup-service:
	chmod +x setup.sh
	./setup.sh
	sudo cp titan.service /etc/systemd/system/
	sudo systemctl daemon-reload
	sudo systemctl enable titan.service
	sudo systemctl start titan.service

uninstall:
	@pkill -f $(BINARY_NAME) || true
	sudo systemctl disable titan.service
	sudo rm /etc/systemd/system/titan.service

all: dev

.PHONY: dev compile build setup-service
