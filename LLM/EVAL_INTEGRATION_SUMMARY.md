# Miri Evaluation Integration Summary

## What Changed

### 1. New Evaluation Tools Control

Both `run.sh` and `run_evaluation.sh` now support `--eval-tools` parameter to control which evaluation tools to run.

**New Parameters:**
- `--eval-tools TOOLS` - Specify evaluation tools (default: `all`)
- `--miri-timeout SEC` - Miri timeout in seconds (default: 300)
- `--loom-timeout SEC` - Loom timeout in seconds (default: 600)

### 2. Evaluation Tools Available

```
Tool          Speed         Used By
─────────────────────────────────────
compare       ✅ Fast       Always in 'all'
clippy        ✅ Fast       Always in 'all'
safety        ⚡ Very Fast  Always in 'all'
miri          🐌 Slow       In 'all' (5+ min per example)
loom          🐢 Very Slow  In 'all' (10+ min per example)
```

### 3. Tool Aliases

```bash
--eval-tools all       # All tools: compare, clippy, safety, miri, loom (default)
--eval-tools full      # Same as 'all'
--eval-tools fast      # Fast tools only: compare, clippy (recommended for development)
--eval-tools none      # No evaluation
```

### 4. Miri Evaluation Output

Results are now saved to `<output>/evaluation/`:
- `miri_report.md` - Human-readable report
- `miri_results.json` - Machine-readable results

### 5. Files Modified

| File | Change |
|------|--------|
| `run.sh` | Added `--eval-tools`, `--miri-timeout`, `--loom-timeout` parameters |
| `run_evaluation.sh` | Added evaluation tool selection logic and conditional execution |
| `evaluation/miri_eval.py` | ✅ Already created in previous setup |
| `run_miri_eval.sh` | ✅ Already created in previous setup |

### 6. New Documentation

- `EVALUATION_WORKFLOW.md` - Complete guide to using evaluation tools
- `MIRI_EVALUATION.md` - Miri-specific documentation

## Usage Examples

### Default (All Tools)
```bash
./run.sh 3
```
- Runs code generation + all evaluation tools
- ⚠️ Slow: 2-4 hours

### Fast Development
```bash
./run.sh 3 --eval-tools fast
```
- Code generation + compare + clippy only
- ✅ Fast: 30-60 minutes

### UB Detection
```bash
./run.sh 3 --eval-tools miri
```
- Code generation + miri
- ~1+ hour

### Custom Combination
```bash
./run.sh 3 --eval-tools "compare,clippy,miri" --miri-timeout 600
```
- Selected tools with custom timeout
- ~1-2 hours

### No Evaluation
```bash
./run.sh 3 --eval-tools none
```
- Code generation only, no evaluation
- ✅ Fastest: 20-30 minutes

## Workflow Integration

### Step 1: Code Generation with Validation (unchanged)
```bash
./run.sh 3 --validate --tools "compile,clippy"
```

### Step 2: Comprehensive Evaluation (new)
The evaluation step now automatically runs based on `--eval-tools`:
```
✅ Running compare_all.py
✅ Running clippy_concurrency_eval.py  
⏭️  Skipped miri (not in EVAL_TOOLS)
⏭️  Skipped loom (not in EVAL_TOOLS)
```

### Step 3: Results Aggregation
All results saved to:
```
result/{timestamp}_3/evaluation/
├── comparison_report.json
├── clippy_concurrency_report.json
├── miri_report.md          # ✨ NEW
├── miri_results.json       # ✨ NEW
├── loom_test_report.md
└── loom_test_results.json
```

## Backward Compatibility

All existing commands still work:
```bash
# Old way - still works
./run.sh 3

# New way - same result
./run.sh 3 --eval-tools all
```

## Direct Evaluation API

You can also run evaluation directly:

```bash
# Fast evaluation only
bash run_evaluation.sh 3 --eval-tools fast --output-dir result/20260412_155953_6_compile

# With miri
bash run_evaluation.sh 3 --eval-tools "compare,miri" --output-dir result/20260412_155953_6_compile

# Just miri with custom timeout
python3 evaluation/miri_eval.py --output-dir result/20260412_155953_6_compile --timeout 600
```

## Performance Expectations

| Configuration | Time | Best For |
|---------------|------|----------|
| None | 20-30 min | Testing generation logic |
| Fast | 30-60 min | Development iterations |
| Compare only | ~15 min | Quick regression check |
| Miri only | 1+ hour | UB detection |
| Loom only | 2+ hours | Concurrency bugs |
| All tools | 2-4 hours | Final validation |

## What Happens During Evaluation

For each evaluation tool (if enabled):

1. **Compare**: Runs baseline metrics on all examples
2. **Clippy**: Static analysis for code safety
3. **Miri** (if enabled):
   - Reads each `final.rs`
   - Appends test module
   - Creates temporary Cargo project
   - Runs `cargo +nightly miri test`
   - Saves results to `evaluation/miri_*.{md,json}`
4. **Loom** (if enabled): Similar process for concurrency testing

## Environment Checklist

Ensure these are available:
```bash
# Rust toolchain
rustup +nightly --version

# Miri (if using --eval-tools with miri)
rustup +nightly component add miri
cargo +nightly miri --version

# Python
python3 --version

# Ollama (for code generation)
ollama --version
```

## Troubleshooting

### Issue: "Miri not found"
```bash
rustup +nightly component add miri
```

### Issue: Evaluation takes too long
```bash
# Use faster tools
./run.sh 3 --eval-tools fast

# Or skip evaluation
./run.sh 3 --eval-tools none
```

### Issue: Some examples fail miri
This is normal! Check the report:
```bash
cat result/*/evaluation/miri_report.md
```

## See Also

- `./run.sh --help` - Full command reference
- `EVALUATION_WORKFLOW.md` - Comprehensive workflow guide
- `MIRI_EVALUATION.md` - Miri tool documentation
