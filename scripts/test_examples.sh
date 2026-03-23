#! /bin/bash

set -e

cargo build --release

# Enable saving results back to source directory
export SAVE_TO_SOURCE=yes

for d in examples/*; do
  if [ -d "$d" ]; then
    td=`mktemp -d`
    ./scripts/test_dir.sh $d $td
    rm -rf $td
  fi
done

echo ""
echo "✅ Done! Results saved to examples/*/"
echo "   - main.c2rust.rs: C2Rust original version"
echo "   - main.concrat.rs: Concrat processed version"
echo "   - main.rs: Final version"
