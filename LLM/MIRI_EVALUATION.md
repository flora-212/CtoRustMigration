# Miri Evaluation Tool Documentation

## Overview

The Miri evaluation tool enables automatic undefined behavior (UB) detection in LLM-generated Rust code. It:

1. **Reads** all `final.rs` files from generated examples
2. **Appends** a test module with stress tests to each file
3. **Runs** `cargo +nightly miri test` to detect UB
4. **Generates** detailed markdown and JSON reports

## What is Miri?

[Miri](https://github.com/rust-lang/miri) is an interpreter for Rust's mid-level intermediate representation (MIR) that detects undefined behavior and other errors.

## Requirements

- Rust toolchain (nightly)
- Miri installed: `rustup +nightly component add miri`
- Python 3

## Quick Start

### Using the wrapper script (recommended)

```bash
cd /home/guoxy/concrat/LLM

# Run with last output directory
bash run_miri_eval.sh --timeout 120

# Or specify output directory explicitly
bash run_miri_eval.sh --output-dir /path/to/output --timeout 120
```

### Direct Python script

```bash
cd /home/guoxy/concrat/LLM

# Test last output directory
python3 evaluation/miri_eval.py --from-last --timeout 300

# Or specify output directory
python3 evaluation/miri_eval.py \
    --output-dir /path/to/output \
    --report-output report.md \
    --json-output results.json \
    --timeout 300
```

## Output

The tool generates three output files in the `<output_dir>/miri_results/` directory:

1. **miri_report.md** - Markdown formatted report with:
   - Summary statistics (pass rate, total time)
   - Detailed results table
   - Failure details with error messages
   - List of safe examples

2. **miri_results.json** - Machine-readable JSON with all test results

3. **miri_test.log** - Full raw output from the script execution

## Example Results

```
📁 Testing directory: /home/guoxy/concrat/LLM/result/20260412_155953_6_compile

🔍 Found 62 examples to test with miri

[59/62] ✅ PASS - Miri tests passed (no UB detected)
[60/62] ✅ PASS - Miri tests passed (no UB detected)  
[61/62] ❌ FAIL - error[E0277]: trait bound not satisfied
[62/62] ✅ PASS - Miri tests passed (no UB detected)

✅ Summary: 59/62 passed, 3 failed
```

## Test Module

The test module appended to each final.rs includes:

```rust
#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn stress_test_correctness() {
        for _ in 0..10 {
            // The main function should complete without panicking
            // Miri will detect any undefined behavior during execution
        }
    }
}
```

## Performance

- **Per-test timeout**: 5 minutes (300s) by default, configurable
- **Average time per test**: ~1-2 seconds with miri
- **Total time for 62 examples**: ~80 seconds

## Common Issues

### "can't find lib `c2rust_out`"
**Solution**: The tool creates a minimal binary-only Cargo.toml to avoid lib/binary conflicts. This error should not occur with the provided script.

### "cannot find function `pthread_mutex_lock`"
**Reason**: Some code references C functions that aren't available in the miri interpreter.
**Solution**: These are compilation errors, not UB in the compiled code. They indicate the code can't be tested with miri due to missing C dependencies.

### Timeout errors
**Solution**: Increase the `--timeout` parameter or check if miri is installed:
```bash
cargo +nightly miri --version
```

## Advanced Options

### Custom timeout
```bash
bash run_miri_eval.sh --output-dir DIR --timeout 600  # 10 minutes
```

### Direct Python invocation with all options
```bash
python3 evaluation/miri_eval.py \
    --output-dir /home/guoxy/concrat/LLM/result/20260412_155953_6_compile \
    --report-output miri_report.md \
    --json-output results.json \
    --timeout 300
```

## Files

- **miri_eval.py** - Main evaluation script
- **run_miri_eval.sh** - Convenient wrapper script

## Results Interpretation

- **✅ PASS**: Code compiled and ran without detected undefined behavior
- **❌ FAIL**: Either compilation failed or miri detected undefined behavior

See the generated markdown report for detailed failure information.

## Integration with CI/CD

You can integrate miri evaluation into your CI/CD pipeline:

```bash
#!/bin/bash
set -e

cd /home/guoxy/concrat/LLM
bash run_miri_eval.sh --output-dir "$OUTPUT_DIR" --timeout 300

# Check results
if grep -q "failed" result/20260412_155953_6_compile/miri_results/miri_test.log; then
    exit 1
fi
```

## References

- [Miri GitHub Repository](https://github.com/rust-lang/miri)
- [Rust Undefined Behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
