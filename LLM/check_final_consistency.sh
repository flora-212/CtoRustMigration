#!/bin/bash

# 脚本用途：检查和更新failed examples的final.rs文件
# 用法：
#   ./check_final_consistency.sh check [result_dir]          - 检查模式
#   ./check_final_consistency.sh update-max-round [result_dir] - 更新failed examples为最大round
#   ./check_final_consistency.sh update-c2rust [result_dir]  - 更新failed examples为c2rust

MODE="${1:-check}"
RESULT_DIR="${2:-.}"

# 获取源examples目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [[ ! -d "$SCRIPT_DIR/examples" ]]; then
    SCRIPT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
fi
EXAMPLES_DIR="${SCRIPT_DIR}/examples"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 计数器
total_examples=0
matches_max_round=0
matches_c2rust=0
mismatches=0
errors=0

print_header() {
    echo -e "${BLUE}=====================================${NC}"
    echo -e "${BLUE}Final.rs 一致性检查工具${NC}"
    echo -e "${BLUE}模式: ${YELLOW}${MODE}${NC}"
    echo -e "${BLUE}扫描目录: ${YELLOW}${RESULT_DIR}/examples${NC}"
    echo -e "${BLUE}=====================================${NC}"
}

get_example_base_name() {
    local full_name="$1"
    echo "${full_name%%____*}"
}

find_max_round() {
    local example_dir="$1"
    local max_round=0
    local max_round_file=""
    
    for round_file in "$example_dir"/round*.rs; do
        if [[ -f "$round_file" ]]; then
            local round_num=$(basename "$round_file" .rs | sed 's/round//')
            if [[ "$round_num" =~ ^[0-9]+$ ]]; then
                if (( round_num > max_round )); then
                    max_round=$round_num
                    max_round_file="$round_file"
                fi
            fi
        fi
    done
    
    echo "$max_round_file"
}

compare_files() {
    local file1="$1"
    local file2="$2"
    
    if [[ ! -f "$file1" ]] || [[ ! -f "$file2" ]]; then
        return 1
    fi
    
    # 忽略空行，比较文件内容
    if diff -q <(grep -v '^$' "$file1") <(grep -v '^$' "$file2") > /dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# 从config.json中读取failed_examples列表
get_failed_examples() {
    local config_file="$RESULT_DIR/config.json"
    
    if [[ ! -f "$config_file" ]]; then
        echo -e "${RED}错误: 找不到config.json: $config_file${NC}"
        return 1
    fi
    
    # 提取failed_examples数组中的所有例子名称
    grep -A 100 '"failed_examples"' "$config_file" | grep -o '"[^"]*"' | grep -v failed_examples | sed 's/"//g' | sort | uniq
}

# 检查是否为failed example
is_failed_example() {
    local example_name="$1"
    
    # 使用grep进行精确行匹配，避免子字符串误匹配
    if echo "$FAILED_LIST" | grep -Fx "$example_name" > /dev/null 2>&1; then
        return 0
    fi
    return 1
}

process_example() {
    local example_dir="$1"
    local example_name=$(basename "$example_dir")
    local final_file="$example_dir/final.rs"
    
    if [[ ! -f "$final_file" ]]; then
        echo -e "${RED}✗${NC} $example_name: 找不到final.rs"
        ((errors++))
        return 0
    fi
    
    ((total_examples++))
    
    local base_name=$(get_example_base_name "$example_name")
    local c2rust_file="$EXAMPLES_DIR/$base_name/main.c2rust.rs"
    local c2rust_exists=false
    if [[ -f "$c2rust_file" ]]; then
        c2rust_exists=true
    fi
    
    local max_round_file=$(find_max_round "$example_dir")
    
    if [[ -z "$max_round_file" ]]; then
        echo -e "${RED}✗${NC} $example_name: 找不到任何round*.rs文件"
        ((errors++))
        return 0
    fi
    
    local matches_round=false
    local matches_c2rust_val=false
    
    if compare_files "$final_file" "$max_round_file"; then
        matches_round=true
        ((matches_max_round++))
    fi
    
    if $c2rust_exists && compare_files "$final_file" "$c2rust_file"; then
        matches_c2rust_val=true
        ((matches_c2rust++))
    fi
    
    if $matches_round && $matches_c2rust_val; then
        echo -e "${GREEN}✓${NC} $example_name: final = max_round 且 = c2rust"
    elif $matches_round; then
        echo -e "${GREEN}✓${NC} $example_name: final = max_round ($(basename $max_round_file .rs))"
    elif $matches_c2rust_val; then
        echo -e "${GREEN}✓${NC} $example_name: final = c2rust"
    else
        echo -e "${YELLOW}!${NC} $example_name: final ≠ max_round 且 ≠ c2rust"
        ((mismatches++))
        if $c2rust_exists; then
            echo -e "     max_round: $(basename $max_round_file .rs), c2rust: main.c2rust.rs"
        fi
    fi
    
    return 0
}

update_failed_max_round() {
    local example_dir="$1"
    local example_name=$(basename "$example_dir")
    
    # 只处理failed examples
    if ! is_failed_example "$example_name"; then
        return 0
    fi
    
    local final_file="$example_dir/final.rs"
    
    if [[ ! -f "$final_file" ]]; then
        echo -e "${RED}✗${NC} $example_name: 找不到final.rs"
        ((errors++))
        return 0
    fi
    
    ((total_examples++))
    
    local max_round_file=$(find_max_round "$example_dir")
    
    if [[ -z "$max_round_file" ]]; then
        echo -e "${RED}✗${NC} $example_name: 找不到任何round*.rs文件"
        ((errors++))
        return 0
    fi
    
    if compare_files "$final_file" "$max_round_file"; then
        echo -e "${GREEN}✓${NC} $example_name: 已经是最大round"
    else
        cp "$max_round_file" "$final_file"
        echo -e "${YELLOW}→${NC} $example_name: 更新为 $(basename $max_round_file .rs)"
    fi
    
    return 0
}

update_failed_c2rust() {
    local example_dir="$1"
    local example_name=$(basename "$example_dir")
    
    # 只处理failed examples
    if ! is_failed_example "$example_name"; then
        return 0
    fi
    
    local final_file="$example_dir/final.rs"
    
    if [[ ! -f "$final_file" ]]; then
        echo -e "${RED}✗${NC} $example_name: 找不到final.rs"
        ((errors++))
        return 0
    fi
    
    ((total_examples++))
    
    local base_name=$(get_example_base_name "$example_name")
    local c2rust_file="$EXAMPLES_DIR/$base_name/main.c2rust.rs"
    
    if [[ ! -f "$c2rust_file" ]]; then
        echo -e "${RED}✗${NC} $example_name: 找不到对应的c2rust文件"
        ((errors++))
        return 0
    fi
    
    if compare_files "$final_file" "$c2rust_file"; then
        echo -e "${GREEN}✓${NC} $example_name: 已经是c2rust代码"
    else
        cp "$c2rust_file" "$final_file"
        echo -e "${YELLOW}→${NC} $example_name: 更新为c2rust代码"
    fi
    
    return 0
}

print_summary() {
    echo ""
    echo -e "${BLUE}=====================================${NC}"
    echo -e "${BLUE}总结${NC}"
    echo -e "${BLUE}=====================================${NC}"
    
    case "$MODE" in
        "check")
            echo "总共检查例子数: $total_examples"
            echo -e "  ${GREEN}最大round一致: $matches_max_round${NC}"
            echo -e "  ${GREEN}c2rust一致: $matches_c2rust${NC}"
            if (( mismatches > 0 )); then
                echo -e "  ${YELLOW}不一致: $mismatches${NC}"
            fi
            if (( errors > 0 )); then
                echo -e "  ${RED}错误: $errors${NC}"
            fi
            ;;
        "update-max-round"|"update-c2rust")
            echo -e "总共处理failed examples: $total_examples"
            if (( errors > 0 )); then
                echo -e "  ${RED}错误: $errors${NC}"
            fi
            ;;
    esac
    echo ""
}

main() {
    if [[ ! -d "$RESULT_DIR/examples" ]]; then
        echo -e "${RED}错误: $RESULT_DIR/examples 目录不存在${NC}"
        exit 1
    fi
    
    case "$MODE" in
        "check"|"update-max-round"|"update-c2rust")
            ;;
        *)
            echo -e "${RED}错误: 未知的模式 '$MODE'${NC}"
            echo "支持的模式: check, update-max-round, update-c2rust"
            exit 1
            ;;
    esac
    
    # 如果是update模式，读取failed_examples列表
    if [[ "$MODE" == "update-max-round" ]] || [[ "$MODE" == "update-c2rust" ]]; then
        FAILED_LIST=$(get_failed_examples) || exit 1
        FAILED_COUNT=$(echo "$FAILED_LIST" | wc -w)
        echo -e "${BLUE}Failed examples: $FAILED_COUNT${NC}"
        echo ""
    fi
    
    print_header
    
    for example_dir in "$RESULT_DIR"/examples/*/; do
        if [[ -d "$example_dir" ]]; then
            case "$MODE" in
                "check")
                    process_example "$example_dir" || true
                    ;;
                "update-max-round")
                    update_failed_max_round "$example_dir" || true
                    ;;
                "update-c2rust")
                    update_failed_c2rust "$example_dir" || true
                    ;;
            esac
        fi
    done
    
    print_summary
}

if [[ "$MODE" == "-h" ]] || [[ "$MODE" == "--help" ]]; then
    cat << EOF
使用方法:
  $0 <mode> [result_dir]

模式:
  check              - 检查final.rs是否与最大round或c2rust一致
  update-max-round   - 更新failed_examples的final.rs为最大round
  update-c2rust      - 更新failed_examples的final.rs为c2rust

参数:
  result_dir         - 结果目录路径（必须包含examples和config.json）

示例:
  # 检查一致性
  $0 check /path/to/result

  # 更新failed examples为最大round
  $0 update-max-round /path/to/result

  # 更新failed examples为c2rust
  $0 update-c2rust /path/to/result
EOF
    exit 0
fi

main
