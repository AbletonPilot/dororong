#!/bin/bash

# Dororong uninstall script

set -e

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_colored() {
    echo -e "${1}${2}${NC}"
}

# Find Dororong executable
find_dororong() {
    local paths=(
        "/usr/local/bin/dororong"
        "$HOME/.local/bin/dororong"
        "$HOME/bin/dororong"
    )
    
    for path in "${paths[@]}"; do
        if [ -f "$path" ]; then
            echo "$path"
            return 0
        fi
    done
    
    # Search in PATH
    if command -v dororong >/dev/null 2>&1; then
        which dororong
        return 0
    fi
    
    return 1
}

main() {
    print_colored "$BLUE" "🗑️  Starting Dororong removal..."
    
    DORORONG_PATH=$(find_dororong)
    
    if [ -z "$DORORONG_PATH" ]; then
        print_colored "$YELLOW" "Dororong is not installed."
        exit 0
    fi
    
    print_colored "$YELLOW" "Found at: $DORORONG_PATH"
    
    # Confirmation
    read -p "Are you sure you want to remove Dororong? (y/N): " -r
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_colored "$YELLOW" "Removal cancelled."
        exit 0
    fi
    
    # Remove
    if rm "$DORORONG_PATH"; then
        print_colored "$GREEN" "✅ Dororong has been successfully removed."
    else
        print_colored "$RED" "Error occurred during removal. You may need sudo permissions."
        print_colored "$YELLOW" "Remove manually: sudo rm $DORORONG_PATH"
        exit 1
    fi
}

main "$@"