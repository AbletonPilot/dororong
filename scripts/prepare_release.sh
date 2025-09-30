#!/bin/bash

# Release preparation automation script
# Usage: ./prepare_release.sh <version>

set -e

if [ $# -ne 1 ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.1.1"
    exit 1
fi

VERSION=$1
TAG="v$VERSION"

echo "Preparing release $VERSION..."

# 1. Download release artifacts from GitHub
echo "Downloading release artifacts..."
TEMP_DIR=$(mktemp -d)

# Source archive (for Homebrew)
SOURCE_URL="https://github.com/AbletonPilot/dororong/archive/$TAG.tar.gz"
SOURCE_FILE="$TEMP_DIR/$TAG.tar.gz"

# Windows binary (for Chocolatey)
WINDOWS_URL="https://github.com/AbletonPilot/dororong/releases/download/$TAG/dororong-$TAG-x86_64-pc-windows-msvc.tar.gz"
WINDOWS_FILE="$TEMP_DIR/dororong-$TAG-x86_64-pc-windows-msvc.tar.gz"

echo "Downloading source archive: $SOURCE_URL"
curl -L -o "$SOURCE_FILE" "$SOURCE_URL"

echo "Downloading Windows binary: $WINDOWS_URL"
curl -L -o "$WINDOWS_FILE" "$WINDOWS_URL"

# 2. Calculate checksums
echo "Calculating checksums..."
SOURCE_SHA256=$(sha256sum "$SOURCE_FILE" | cut -d' ' -f1)
WINDOWS_SHA256=$(sha256sum "$WINDOWS_FILE" | cut -d' ' -f1)

echo "Source archive SHA256: $SOURCE_SHA256"
echo "Windows binary SHA256: $WINDOWS_SHA256"

# 3. Update Homebrew Formula
echo "Updating Homebrew Formula..."
sed -i.bak "s|archive/v[0-9.]*\.tar\.gz|archive/$TAG.tar.gz|g" homebrew-core-formula.rb
sed -i.bak "s|PLACEHOLDER_SHA256|$SOURCE_SHA256|g" homebrew-core-formula.rb

# 4. Update Chocolatey files
echo "Updating Chocolatey package..."
sed -i.bak "s|<version>[0-9.]*</version>|<version>$VERSION</version>|g" chocolatey/dororong.nuspec
sed -i.bak "s|\$version = '[0-9.]*'|\$version = '$VERSION'|g" chocolatey/tools/chocolateyinstall.ps1
sed -i.bak "s|CHECKSUM64_PLACEHOLDER|$WINDOWS_SHA256|g" chocolatey/tools/chocolateyinstall.ps1

# 5. Cleanup
rm -rf "$TEMP_DIR"
rm -f homebrew-core-formula.rb.bak chocolatey/dororong.nuspec.bak chocolatey/tools/chocolateyinstall.ps1.bak

echo "Release $VERSION preparation complete!"
echo ""
echo "Next steps:"
echo "1. Review changes and commit"
echo "2. Create PR to Homebrew Core"
echo "3. Submit package to Chocolatey Community Repository"