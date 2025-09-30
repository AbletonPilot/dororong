# Dororong-Cli

Dororong will be dancing for you! A fun terminal animation app.

## Quick Installation

### Download Binary (Recommended)
Download the latest release for your platform from [GitHub Releases](https://github.com/AbletonPilot/dororong/releases)

### Linux (One-liner)
```bash
curl -fsSL https://raw.githubusercontent.com/AbletonPilot/dororong/main/install/install.sh | bash
```

## Installation Methods

### macOS

#### Manual Installation
Download from [Releases page](https://github.com/AbletonPilot/dororong/releases):
- `dororong-v*-x86_64-apple-darwin.tar.gz` (Intel Mac)
- `dororong-v*-aarch64-apple-darwin.tar.gz` (Apple Silicon/ARM)

#### Homebrew (Coming Soon)
```bash
# Will be available after Homebrew Core approval
brew install dororong
```

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

#### Chocolatey (Coming Soon)
```powershell
# Will be available after Chocolatey Community approval
choco install dororong
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