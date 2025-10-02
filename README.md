# Dororong-Cli

Dororong will be dancing for you! A fun terminal animation app.

## Preview

https://github.com/user-attachments/assets/368feb4a-2b06-495e-9f33-d9eace37d488

## Quick Installation

### Download Binary (Recommended)
Download the latest release for your platform from [GitHub Releases](https://github.com/AbletonPilot/dororong/releases)

### Linux (One-liner)
```bash
curl -fsSL https://raw.githubusercontent.com/AbletonPilot/dororong/main/install/install.sh | bash
```

## Installation Methods

### macOS

#### Homebrew (Recommended)
```bash
brew tap abletonpilot/dororong
brew install dororong
```

#### Manual Installation
Download from [Releases page](https://github.com/AbletonPilot/dororong/releases):
- `dororong-v*-x86_64-apple-darwin.tar.gz` (Intel Mac)
- `dororong-v*-aarch64-apple-darwin.tar.gz` (Apple Silicon/ARM)

### Linux

#### One-liner Install Script (Recommended)
```bash
# Install (auto-configures PATH and uses sudo when needed)
curl -fsSL https://raw.githubusercontent.com/AbletonPilot/dororong/main/install/install.sh | bash

# Or using wget
wget -qO- https://raw.githubusercontent.com/AbletonPilot/dororong/main/install/install.sh | bash
```

#### Homebrew on Linux
```bash
brew tap abletonpilot/dororong
brew install dororong
```
### Windows

#### Manual Installation (Recommended)
1. Download the latest Windows release from [Releases page](https://github.com/AbletonPilot/dororong/releases)
   - Look for `dororong-v*-x86_64-pc-windows-msvc.zip`
2. Extract the archive
3. Add `dororong.exe` to PATH:

```powershell
$path = [Environment]::GetEnvironmentVariable("PATH", "User")
[Environment]::SetEnvironmentVariable("PATH", "$path;C:\path\to\dororong", "User")
```

## Usage

### Basic Commands
```bash
# Show help
dororong --help

# Check version
dororong --version

# Static display with text
dororong say "Hello World"
```

### Animations
```bash
dororong bokbok      # Bokbok animation
dororong pangpang    # Pangpang animation
dororong run         # Running animation
dororong dance       # Dance animation
dororong frontback   # Front-back animation
dororong updown      # Up-down animation
```

### Options
- `--fast` or `-f`: Fast animation
- Exit: `q`, `Esc`, or `Ctrl+C`

## Uninstallation

### Linux/macOS (Script Install)
```bash
# One-liner uninstall
curl -fsSL https://raw.githubusercontent.com/AbletonPilot/dororong/main/install/uninstall.sh | bash

# Or using wget
wget -qO- https://raw.githubusercontent.com/AbletonPilot/dororong/main/install/uninstall.sh | bash
```

### Homebrew
```bash
brew uninstall dororong
brew untap abletonpilot/dororong  # Optional: remove tap
```

### Manual Uninstall
```bash
# Linux/macOS - remove binary
sudo rm /usr/local/bin/dororong
# or
rm ~/.local/bin/dororong

# Remove PATH entries from shell config if needed
# Edit ~/.zshrc, ~/.bashrc, etc. and remove dororong-related lines
```

## Building from Source

```bash
# Clone repository
git clone https://github.com/AbletonPilot/dororong.git
cd dororong

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --release

# Run
./target/release/dororong
```

## System Requirements

- **Linux**: glibc 2.31+ (Ubuntu 20.04+, Debian 11+, CentOS 8+)
- **macOS**: 10.15+ (Catalina)
- **Windows**: Windows 10+

## Troubleshooting

### Permission Error (Linux/macOS)
```bash
chmod +x /path/to/dororong
```

### PATH Issues
```bash
# Add to PATH for current session
export PATH="/path/to/dororong:$PATH"

# Add to PATH permanently (bash)
echo 'export PATH="/path/to/dororong:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Windows Defender Warning
Some antivirus software may report false positives. This is common with Rust-compiled binaries. Mark as safe or add to exceptions.

## For Homebrew Tap Maintainers

This project includes a Homebrew formula in the `Formula/` directory. To set up your own tap:

```bash
# 1. Create a tap repository named 'homebrew-dororong'
# 2. Copy Formula/dororong.rb to your tap repository
# 3. Users can install with:
brew tap abletonpilot/dororong
brew install dororong
```

The formula builds from source using Cargo, ensuring compatibility across all platforms.

## Support & Contributing

- **Bug Reports**: [GitHub Issues](https://github.com/AbletonPilot/dororong/issues)
- **Feature Requests**: [GitHub Discussions](https://github.com/AbletonPilot/dororong/discussions)
- **Contributing**: Pull requests are always welcome!