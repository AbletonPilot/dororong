#!/bin/bash

# Dororong 설치 스크립트
# Installation script for Dororong

set -e

# 색상 정의
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 함수 정의
print_colored() {
    echo -e "${1}${2}${NC}"
}

error_exit() {
    print_colored "$RED" "오류: $1"
    exit 1
}

# 시스템 정보 감지
detect_arch() {
    case $(uname -m) in
        x86_64) echo "x86_64" ;;
        aarch64|arm64) echo "aarch64" ;;
        *) error_exit "지원하지 않는 아키텍처: $(uname -m)" ;;
    esac
}

detect_os() {
    case $(uname -s) in
        Linux) echo "linux" ;;
        Darwin) echo "macos" ;;
        *) error_exit "지원하지 않는 운영체제: $(uname -s)" ;;
    esac
}

# 최신 버전 가져오기
get_latest_version() {
    curl -s https://api.github.com/repos/AbletonPilot/dororong/releases/latest | \
        grep '"tag_name":' | \
        sed -E 's/.*"([^"]+)".*/\1/' || error_exit "최신 버전을 가져올 수 없습니다"
}

# 메인 함수
main() {
    print_colored "$BLUE" "🎭 Dororong 설치를 시작합니다..."
    
    # 시스템 정보
    OS=$(detect_os)
    ARCH=$(detect_arch)
    VERSION=$(get_latest_version)
    
    print_colored "$YELLOW" "감지된 시스템: $OS-$ARCH"
    print_colored "$YELLOW" "설치할 버전: $VERSION"
    
    # 다운로드 URL 구성
    if [ "$OS" = "macos" ]; then
        BINARY_NAME="dororong-macos-$ARCH"
    else
        BINARY_NAME="dororong-linux-$ARCH"
    fi
    
    DOWNLOAD_URL="https://github.com/AbletonPilot/dororong/releases/download/$VERSION/$BINARY_NAME"
    
    # 임시 디렉토리 생성
    TEMP_DIR=$(mktemp -d)
    trap "rm -rf $TEMP_DIR" EXIT
    
    print_colored "$BLUE" "다운로드 중: $DOWNLOAD_URL"
    
    # 다운로드
    if command -v curl >/dev/null 2>&1; then
        curl -L -o "$TEMP_DIR/dororong" "$DOWNLOAD_URL" || error_exit "다운로드 실패"
    elif command -v wget >/dev/null 2>&1; then
        wget -O "$TEMP_DIR/dororong" "$DOWNLOAD_URL" || error_exit "다운로드 실패"
    else
        error_exit "curl 또는 wget이 필요합니다"
    fi
    
    # 실행 권한 부여
    chmod +x "$TEMP_DIR/dororong"
    
    # 설치 위치 결정
    if [ -w "/usr/local/bin" ]; then
        INSTALL_DIR="/usr/local/bin"
    elif [ -w "$HOME/.local/bin" ]; then
        INSTALL_DIR="$HOME/.local/bin"
        mkdir -p "$INSTALL_DIR"
    else
        INSTALL_DIR="$HOME/bin"
        mkdir -p "$INSTALL_DIR"
    fi
    
    # 설치
    print_colored "$BLUE" "설치 중: $INSTALL_DIR/dororong"
    cp "$TEMP_DIR/dororong" "$INSTALL_DIR/dororong"
    
    # PATH 확인
    if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
        print_colored "$YELLOW" "경고: $INSTALL_DIR가 PATH에 없습니다."
        print_colored "$YELLOW" "다음 명령을 실행하여 PATH에 추가하세요:"
        print_colored "$YELLOW" "echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.bashrc"
        print_colored "$YELLOW" "source ~/.bashrc"
    fi
    
    print_colored "$GREEN" "✅ Dororong 설치가 완료되었습니다!"
    print_colored "$GREEN" "사용법: dororong --help"
    
    # 테스트 실행
    if "$INSTALL_DIR/dororong" --help >/dev/null 2>&1; then
        print_colored "$GREEN" "🎉 설치가 성공적으로 완료되었습니다!"
    else
        print_colored "$YELLOW" "설치는 완료되었지만 실행 테스트에 실패했습니다."
    fi
}

main "$@"