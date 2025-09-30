#!/bin/bash

# Dororong 제거 스크립트
# Uninstall script for Dororong

set -e

# 색상 정의
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_colored() {
    echo -e "${1}${2}${NC}"
}

# Dororong 실행 파일 찾기
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
    
    # PATH에서 찾기
    if command -v dororong >/dev/null 2>&1; then
        which dororong
        return 0
    fi
    
    return 1
}

main() {
    print_colored "$BLUE" "🗑️  Dororong 제거를 시작합니다..."
    
    DORORONG_PATH=$(find_dororong)
    
    if [ -z "$DORORONG_PATH" ]; then
        print_colored "$YELLOW" "Dororong이 설치되어 있지 않습니다."
        exit 0
    fi
    
    print_colored "$YELLOW" "발견된 경로: $DORORONG_PATH"
    
    # 확인
    read -p "정말로 Dororong을 제거하시겠습니까? (y/N): " -r
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_colored "$YELLOW" "제거가 취소되었습니다."
        exit 0
    fi
    
    # 제거
    if rm "$DORORONG_PATH"; then
        print_colored "$GREEN" "✅ Dororong이 성공적으로 제거되었습니다."
    else
        print_colored "$RED" "제거 중 오류가 발생했습니다. sudo 권한이 필요할 수 있습니다."
        print_colored "$YELLOW" "수동으로 제거하세요: sudo rm $DORORONG_PATH"
        exit 1
    fi
}

main "$@"