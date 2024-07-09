# Variables
TARGET = target/release/cinit
INSTALL_DIR = /usr/local/bin

# Default target
all: build

# Build the release binary
build:
	cargo build --release

# Install the binary
install: build
	@echo "Installing cinit to $(INSTALL_DIR)"
	@sudo cp $(TARGET) $(INSTALL_DIR)

# Uninstall the binary
uninstall:
	@echo "Removing cinit from $(INSTALL_DIR)"
	@sudo rm -f $(INSTALL_DIR)/cinit

# Clean the project
clean:
	cargo clean

# Run tests
test: build
	cargo test

# Phony targets to avoid conflicts with files
.PHONY: all build install uninstall clean test
