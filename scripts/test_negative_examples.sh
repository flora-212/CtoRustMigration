#! /bin/bash

cargo build --release

failed_examples=()
successful_examples=()

for d in examples_negative/*; do
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
echo "✅ Done! Results saved to examples_negative/*/"
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