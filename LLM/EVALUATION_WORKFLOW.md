# Integrated Evaluation Workflow Guide

## Overview

The new evaluation workflow allows you to control which evaluation tools run during the full `run.sh` execution. By default, all evaluation tools are enabled (compare, clippy, safety, miri, loom).

## Quick Start

### Run with all evaluation tools (default)
```bash
./run.sh 3
```
- Runs code generation with validation
- Runs all evaluation tools: compare, clippy, miri, loom
- ⚠️ **Very slow!** Expect 2-4 hours with miri and loom

### Run with fast evaluation (compare + clippy only)
```bash
./run.sh 3 --eval-tools fast
```
- Excludes slow tools (miri, loom)
- ✅ Fast: ~30-60 minutes total

### Run without evaluation
```bash
./run.sh 3 --eval-tools none
```
- Only code generation, no evaluation
- ✅ Fastest: ~20-30 minutes

### Run with specific tools
```bash
./run.sh 3 --eval-tools "compare,clippy,miri"
```
- Only: compare, clippy, miri (excludes loom)
- Good balance: ~1-2 hours

## Available Evaluation Tools

| Tool | Speed | Purpose | Command |
|------|-------|---------|---------|
| compare | ✅ Fast | Baseline comparison | always in `all` and `full` |
| clippy | ✅ Fast | Code style & safety analysis | always in `all` and `full` |
| safety | ⚡ Very Fast | Static unsafe pattern detection | only in `all` and `full` |
| miri | 🐌 Slow | Runtime UB detection | 5+ min per example |
| loom | 🐢 Very Slow | Concurrency bug detection | 10+ min per example |

## Tool Aliases

```bash
--eval-tools all       # compare, clippy, safety, miri, loom (default if not specified)
--eval-tools full      # same as all
--eval-tools fast      # compare, clippy (fastest)
--eval-tools none      # skip all evaluations
```

## Custom Tool Combination

```bash
# Only compare and clippy
./run.sh 3 --eval-tools "compare,clippy"

# Only miri and loom (UB detection)
./run.sh 3 --eval-tools "miri,loom"

# Compare, clippy, and miri
./run.sh 3 --eval-tools "compare,clippy,miri"

# Custom timeout for slow tools
./run.sh 3 --eval-tools "compare,miri" --miri-timeout 600 --loom-timeout 1200
```

## Full Workflow Examples

### Fast Development Loop (30 minutes)
```bash
./run.sh 3 --eval-tools fast
```
Best for: Testing code generation changes, quick iterations

### Comprehensive Validation (2+ hours)
```bash
./run.sh 3 --eval-tools all
```
Best for: Final validation before publishing results

### UB Detection Only (1+ hour)
```bash
./run.sh 3 --eval-tools "miri,loom"
```
Best for: Focusing on undefined behavior and concurrency issues

### With Negative Samples (3+ hours)
```bash
./run.sh 3 --include-negative --eval-tools fast
```
Best for: Testing with both positive and negative examples

## Timeouts and Performance

### Default Timeouts
- Miri: 300 seconds (5 minutes) per example
- Loom: 600 seconds (10 minutes) per example

### Adjust Timeouts
```bash
# Increase miri timeout to 10 minutes per example
./run.sh 3 --eval-tools miri --miri-timeout 600

# Increase loom timeout to 20 minutes per example  
./run.sh 3 --eval-tools loom --loom-timeout 1200
```

## Output Structure

After running `./run.sh 3`, results are saved to:
```
result/{timestamp}_3/
├── config.json                      # Configuration parameters
├── examples/
│   ├── {example_name}/
│   │   ├── round1.rs, round2.rs     # Iteration history
│   │   └── final.rs                 # Final generated code
│   └── ...
└── evaluation/
    ├── comparison_report.json       # Baseline comparison
    ├── comparison_report.md
    ├── clippy_concurrency_report.json
    ├── clippy_concurrency_report.md
    ├── loom_test_results.json       # Loom results (if run)
    ├── loom_test_report.md
    ├── miri_results.json            # Miri results (if run)
    └── miri_report.md
```

## Expected Runtimes

### By Tool Configuration
- `--eval-tools none`: 20-30 min (generation only)
- `--eval-tools fast`: 30-60 min (compare + clippy)
- `--eval-tools "compare,miri"`: 1-2 hours
- `--eval-tools "compare,loom"`: 2-4 hours  
- `--eval-tools all`: 2-4 hours (miri + loom slowest)

### Factors Affecting Time
- Number of examples (62 by default)
- Number of iterations (validation loop: max 20)
- Tool complexity: miri > loom > clippy > compare
- System CPU/RAM availability

## Integration with CI/CD

### Fast check for regressions
```bash
./run.sh 3 --eval-tools fast --no-validate
```
- Code generation only with compare/clippy
- ~30-60 minutes

### Comprehensive nightly build
```bash
./run.sh 3 --eval-tools all --include-negative
```
- Full validation with all tools
- ~3-4 hours

## Troubleshooting

### Miri not found
```bash
# Install miri
rustup +nightly component add miri

# Test it works
cargo +nightly miri --version
```

### Loom tests timing out
- Increase timeout: `--loom-timeout 1200`
- Or reduce tools: `--eval-tools fast`

### Evaluation results not appearing
Check that results were saved:
```bash
ls -la result/*/evaluation/
```

## See Also

- [MIRI_EVALUATION.md](MIRI_EVALUATION.md) - Miri tool documentation
- [run.sh --help](run.sh) - Full command line options
