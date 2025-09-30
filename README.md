# 🐧 Dororong

Dororong will be dancing for you! A fun terminal animation app.

## Quick Installation

### Homebrew (macOS)
```bash
brew install dororong
```

### Linux (One-liner)
```bash
curl -fsSL https://raw.githubusercontent.com/AbletonPilot/dororong/main/install/install.sh | bash
```

### Windows (Chocolatey)
```powershell
choco install dororong
```

## Installation Methods

### macOS

#### Homebrew (Recommended)
```bash
brew install dororong

# Update
brew upgrade dororong
```

#### Manual Installation
Download from [Releases page](https://github.com/AbletonPilot/dororong/releases):
- `dororong-macos-x86_64` (Intel Mac)
- `dororong-macos-aarch64` (Apple Silicon)

### Linux

#### One-liner Install Script (Recommended)
```bash
# Install
curl -fsSL https://raw.githubusercontent.com/AbletonPilot/dororong/main/install/install.sh | bash

# Or using wget
wget -qO- https://raw.githubusercontent.com/AbletonPilot/dororong/main/install/install.sh | bash

# Uninstall
curl -fsSL https://raw.githubusercontent.com/AbletonPilot/dororong/main/install/uninstall.sh | bash
```

#### Snap Package
```bash
sudo snap install dororong
sudo snap remove dororong  # Uninstall
```

#### Manual Installation
```bash
# Download and extract the archive for your platform
wget https://github.com/AbletonPilot/dororong/releases/latest/download/dororong-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
tar -xzf dororong-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
sudo mv dororong-v0.1.0-x86_64-unknown-linux-gnu/dororong /usr/local/bin/
```

### Windows

#### Chocolatey (Recommended)
```powershell
choco install dororong
choco upgrade dororong  # Update
choco uninstall dororong  # Uninstall
```

#### Manual Installation
1. Download `dororong-v0.1.0-x86_64-pc-windows-msvc.zip` from [Releases page](https://github.com/AbletonPilot/dororong/releases)
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

## Support & Contributing

- **Bug Reports**: [GitHub Issues](https://github.com/AbletonPilot/dororong/issues)
- **Feature Requests**: [GitHub Discussions](https://github.com/AbletonPilot/dororong/discussions)
- **Contributing**: Pull requests are always welcome!