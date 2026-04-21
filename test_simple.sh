#!/bin/bash
RESULT_DIR="/home/guoxy/concrat/LLM/result/20260410_201847_6_compile"
EXAMPLES_DIR="/home/guoxy/concrat/examples"

count=0
for example_dir in "$RESULT_DIR"/examples/*/; do
    ((count++))
    if (( count > 5 )); then
        break
    fi
    
    example_name=$(basename "$example_dir")
    final_file="$example_dir/final.rs"
    
    if [[ ! -f "$final_file" ]]; then
        echo "✗ $example_name: 找不到final.rs"
        continue
    fi
    
    # 查找最大round
    max_round=0
    max_round_file=""
    for round_file in "$example_dir"/round*.rs; do
        if [[ -f "$round_file" ]]; then
            round_num=$(basename "$round_file" .rs | sed 's/round//')
            if [[ "$round_num" =~ ^[0-9]+$ ]]; then
                if (( round_num > max_round )); then
                    max_round=$round_num
                    max_round_file="$round_file"
                fi
            fi
        fi
    done
    
    if [[ -z "$max_round_file" ]]; then
        echo "✗ $example_name: 找不到任何round*.rs"
        continue
    fi
    
    # 比较
    if cmp -s "$final_file" "$max_round_file"; then
        echo "✓ $example_name: final = $(basename $max_round_file .rs)"
    else
        echo "✗ $example_name: final ≠ $(basename $max_round_file .rs)"
    fi
done
