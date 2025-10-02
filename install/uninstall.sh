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
    print_colored "$BLUE" "Starting Dororong removal..."
    
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
    
    # Remove (with sudo if needed)
    if rm "$DORORONG_PATH" 2>/dev/null; then
        print_colored "$GREEN" "Dororong has been successfully removed."
    elif command -v sudo >/dev/null 2>&1 && sudo rm "$DORORONG_PATH" 2>/dev/null; then
        print_colored "$GREEN" "Dororong has been successfully removed."
    else
        print_colored "$RED" "Error occurred during removal."
        print_colored "$YELLOW" "Try manually: sudo rm $DORORONG_PATH"
        exit 1
    fi
    
    # Remove PATH entries from shell configs
    print_colored "$BLUE" "Checking shell configuration files..."
    local shell_configs=(
        "$HOME/.zshrc"
        "$HOME/.bashrc"
        "$HOME/.bash_profile"
        "$HOME/.profile"
    )
    
    local cleaned=false
    for config in "${shell_configs[@]}"; do
        if [ -f "$config" ] && grep -q "Added by Dororong installer" "$config" 2>/dev/null; then
            # Remove Dororong PATH entries
            sed -i.bak '/# Added by Dororong installer/d' "$config"
            sed -i.bak '/\.local\/bin.*dororong/d' "$config"
            sed -i.bak '/export PATH.*\.local\/bin/d' "$config"
            rm -f "$config.bak"
            print_colored "$GREEN" "Cleaned PATH from $config"
            cleaned=true
        fi
    done
    
    if [ "$cleaned" = false ]; then
        print_colored "$YELLOW" "No PATH entries found in shell configs"
    fi
    
    print_colored "$GREEN" ""
    print_colored "$GREEN" "Uninstallation complete!"
}

main "$@"