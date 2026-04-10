#! /bin/bash

cargo build --release

# Enable saving results back to source directory
export SAVE_TO_SOURCE=yes

failed_examples=()
successful_examples=()

for d in examples/*; do
  if [ -d "$d" ]; then
    td=`mktemp -d`
    if ./scripts/test_dir.sh $d $td; then
      successful_examples+=("$d")
    else
      failed_examples+=("$d")
    fi
    rm -rf $td
  fi
done

echo ""
echo "✅ Done! Results saved to examples/*/"
echo "   - main.c2rust.rs: C2Rust original version"
echo "   - main.concrat.rs: Concrat processed version"
echo "   - main.rs: Final version"
echo ""

total=$((${#successful_examples[@]} + ${#failed_examples[@]}))

if [ ${#failed_examples[@]} -gt 0 ]; then
  echo "❌ Failed examples (${#failed_examples[@]}/$total):"
  for example in "${failed_examples[@]}"; do
    echo "   ✗ $example"
  done
  echo ""
  echo "Summary: ${#successful_examples[@]}/$total successful, ${#failed_examples[@]}/$total failed"
  exit 1
else
  echo "🎉 All examples compiled successfully! ($total/$total)"
  exit 0
fi
