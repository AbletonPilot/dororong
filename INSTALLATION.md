# 📦 Dororong 배포 가이드

Dororong을 다양한 플랫폼에서 설치할 수 있는 방법을 제공합니다.

## 🍺 Homebrew (macOS)

가장 쉬운 macOS 설치 방법:

```bash
# tap 추가 (한 번만 실행)
brew tap AbletonPilot/tap

# 설치
brew install dororong

# 업데이트
brew upgrade dororong
```

## 🐧 Linux

### 원클릭 설치 스크립트

```bash
# 설치
curl -fsSL https://raw.githubusercontent.com/AbletonPilot/dororong/main/install/install.sh | bash

# 또는 wget 사용
wget -qO- https://raw.githubusercontent.com/AbletonPilot/dororong/main/install/install.sh | bash

# 제거
curl -fsSL https://raw.githubusercontent.com/AbletonPilot/dororong/main/install/uninstall.sh | bash
```

### Snap (Ubuntu/Debian)

```bash
# 설치
sudo snap install dororong

# 제거
sudo snap remove dororong
```

### 수동 설치

1. [Releases 페이지](https://github.com/AbletonPilot/dororong/releases)에서 최신 버전 다운로드
2. 적절한 Linux 아키텍처 선택:
   - `dororong-linux-x86_64.tar.gz` (Intel/AMD 64-bit)
   - `dororong-linux-aarch64.tar.gz` (ARM 64-bit)

```bash
# 다운로드 및 설치 예시 (x86_64)
wget https://github.com/AbletonPilot/dororong/releases/latest/download/dororong-linux-x86_64.tar.gz
tar -xzf dororong-linux-x86_64.tar.gz
sudo mv dororong-*/dororong /usr/local/bin/
```

## 🪟 Windows

### Chocolatey

```powershell
# 설치
choco install dororong

# 업데이트
choco upgrade dororong

# 제거
choco uninstall dororong
```

### 수동 설치

1. [Releases 페이지](https://github.com/AbletonPilot/dororong/releases)에서 `dororong-windows-x86_64.zip` 다운로드
2. 압축 해제
3. `dororong.exe`를 PATH에 추가

```powershell
# PowerShell에서 PATH에 추가하는 예시
$path = [Environment]::GetEnvironmentVariable("PATH", "User")
[Environment]::SetEnvironmentVariable("PATH", "$path;C:\path\to\dororong", "User")
```

## 🔨 개발자용 빌드

소스코드에서 직접 빌드:

```bash
# 저장소 클론
git clone https://github.com/AbletonPilot/dororong.git
cd dororong

# Rust 설치 (필요한 경우)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 빌드
cargo build --release

# 실행
./target/release/dororong
```

## 📋 시스템 요구사항

- **Linux**: glibc 2.31+ (Ubuntu 20.04+, Debian 11+, CentOS 8+)
- **macOS**: 10.15+ (Catalina)
- **Windows**: Windows 10+

## 🔍 설치 확인

설치가 완료된 후 다음 명령으로 확인:

```bash
dororong --help
dororong --version
```

## 🚀 사용법

```bash
# 기본 춤 보기
dororong

# 특정 애니메이션 선택
dororong --animation dance

# 정적 이미지 보기
dororong --static

# 도움말 보기
dororong --help
```

## 🐛 문제 해결

### 권한 오류 (Linux/macOS)
```bash
# 실행 권한 추가
chmod +x /path/to/dororong
```

### PATH 문제
```bash
# 현재 세션에만 PATH 추가
export PATH="/path/to/dororong:$PATH"

# 영구적으로 PATH 추가 (bash)
echo 'export PATH="/path/to/dororong:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Windows Defender 경고
일부 바이러스 백신에서 허위 양성을 보고할 수 있습니다. 이는 Rust로 컴파일된 바이너리의 일반적인 문제입니다. 안전하다고 표시하거나 예외에 추가하세요.

## 📞 지원

- 🐛 버그 리포트: [GitHub Issues](https://github.com/AbletonPilot/dororong/issues)
- 💡 기능 요청: [GitHub Discussions](https://github.com/AbletonPilot/dororong/discussions)
- 📖 문서: [GitHub Wiki](https://github.com/AbletonPilot/dororong/wiki)

## 🔄 자동 업데이트

### Homebrew
```bash
brew upgrade dororong
```

### Chocolatey
```powershell
choco upgrade dororong
```

### Snap
```bash
sudo snap refresh dororong
```

### 수동 설치의 경우
설치 스크립트를 다시 실행하면 최신 버전으로 업데이트됩니다.