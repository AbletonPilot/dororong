#!/bin/bash

# Dororong installation script

set -e

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function definitions
print_colored() {
    echo -e "${1}${2}${NC}"
}

error_exit() {
    print_colored "$RED" "Error: $1"
    exit 1
}

# System detection
detect_arch() {
    case $(uname -m) in
        x86_64) echo "x86_64" ;;
        aarch64|arm64) echo "aarch64" ;;
        *) error_exit "Unsupported architecture: $(uname -m)" ;;
    esac
}

detect_os() {
    case $(uname -s) in
        Linux) echo "linux" ;;
        Darwin) echo "macos" ;;
        *) error_exit "Unsupported OS: $(uname -s)" ;;
    esac
}

# Get latest version
get_latest_version() {
    curl -s https://api.github.com/repos/AbletonPilot/dororong/releases/latest | \
        grep '"tag_name":' | \
        sed -E 's/.*"([^"]+)".*/\1/' || error_exit "Failed to get latest version"
}

# Main function
main() {
    print_colored "$BLUE" "🎭 Starting Dororong installation..."
    
    # System information
    OS=$(detect_os)
    ARCH=$(detect_arch)
    VERSION=$(get_latest_version)
    
    print_colored "$YELLOW" "Detected system: $OS-$ARCH"
    print_colored "$YELLOW" "Installing version: $VERSION"
    
    # Download URL configuration
    if [ "$OS" = "macos" ]; then
        ARCHIVE_NAME="dororong-$VERSION-x86_64-apple-darwin.tar.gz"
    elif [ "$OS" = "linux" ]; then
        ARCHIVE_NAME="dororong-$VERSION-x86_64-unknown-linux-gnu.tar.gz"
    else
        error_exit "Unsupported operating system"
    fi
    
    DOWNLOAD_URL="https://github.com/AbletonPilot/dororong/releases/download/$VERSION/$ARCHIVE_NAME"
    
    # Create temporary directory
    TEMP_DIR=$(mktemp -d)
    trap "rm -rf $TEMP_DIR" EXIT
    
    print_colored "$BLUE" "Downloading: $DOWNLOAD_URL"
    
    # Download
    if command -v curl >/dev/null 2>&1; then
        curl -L -o "$TEMP_DIR/$ARCHIVE_NAME" "$DOWNLOAD_URL" || error_exit "Download failed"
    elif command -v wget >/dev/null 2>&1; then
        wget -O "$TEMP_DIR/$ARCHIVE_NAME" "$DOWNLOAD_URL" || error_exit "Download failed"
    else
        error_exit "curl or wget is required"
    fi
    
    # Extract archive
    print_colored "$BLUE" "Extracting archive..."
    cd "$TEMP_DIR"
    if [ "${ARCHIVE_NAME##*.}" = "gz" ]; then
        tar -xzf "$ARCHIVE_NAME" || error_exit "Extraction failed"
    else
        unzip "$ARCHIVE_NAME" || error_exit "Extraction failed"
    fi
    
    # Find binary file
    BINARY_FILE=$(find . -name "dororong" -type f | head -1)
    if [ -z "$BINARY_FILE" ]; then
        error_exit "Could not find dororong binary in extracted files"
    fi
    
    # Set executable permissions
    chmod +x "$BINARY_FILE"
    
    # Determine installation directory
    if [ -w "/usr/local/bin" ]; then
        INSTALL_DIR="/usr/local/bin"
    elif [ -w "$HOME/.local/bin" ]; then
        INSTALL_DIR="$HOME/.local/bin"
        mkdir -p "$INSTALL_DIR"
    else
        INSTALL_DIR="$HOME/bin"
        mkdir -p "$INSTALL_DIR"
    fi
    
    # Install
    print_colored "$BLUE" "Installing to: $INSTALL_DIR/dororong"
    cp "$BINARY_FILE" "$INSTALL_DIR/dororong"
    
    # Check PATH
    if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
        print_colored "$YELLOW" "Warning: $INSTALL_DIR is not in PATH."
        print_colored "$YELLOW" "Run the following commands to add it to PATH:"
        print_colored "$YELLOW" "echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.bashrc"
        print_colored "$YELLOW" "source ~/.bashrc"
    fi
    
    print_colored "$GREEN" "✅ Dororong installation completed!"
    print_colored "$GREEN" "Usage: dororong --help"
    
    # Test execution
    if "$INSTALL_DIR/dororong" --help >/dev/null 2>&1; then
        print_colored "$GREEN" "🎉 Installation successful!"
    else
        print_colored "$YELLOW" "Installation completed but execution test failed."
    fi
}

main "$@"