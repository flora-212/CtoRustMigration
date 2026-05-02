# 🎯 Evaluation Results Summary - Executive Report

**Date**: 2026-04-21  
**Total Examples Analyzed**: 62  
**Experimental Runs**: 5 per group × 2 groups  
**Evaluation Metrics**: 5 types (LOOM, MIRI, OUTPUT, CLIPPY, 编译通过率)

---

## 📊 Overall Performance

### Summary Comparison

```
┌─────────────────┬──────────────┬──────────────┬──────────────┐
│ Metric          │ Non-C2RUST   │ C2RUST       │ Improvement  │
├─────────────────┼──────────────┼──────────────┼──────────────┤
│ Overall         │  412/1550    │  759/1550    │ +347 (+22%)  │
│                 │  (26.6%)     │  (49.0%)     │              │
├─────────────────┼──────────────┼──────────────┼──────────────┤
│ LOOM            │  42/310      │  42/310      │  No change   │
│                 │  (13.5%)     │  (13.5%)     │              │
├─────────────────┼──────────────┼──────────────┼──────────────┤
│ MIRI            │  166/310     │  308/310     │ +142 (+85%)  │
│                 │  (53.5%)     │  (99.4%) ✅  │              │
├─────────────────┼──────────────┼──────────────┼──────────────┤
│ OUTPUT          │  38/310      │  101/310     │ +63 (+166%)  │
│                 │  (12.3%)     │  (32.6%)     │              │
├─────────────────┼──────────────┼──────────────┼──────────────┤
│ CLIPPY          │  0/310       │  0/310       │  No change   │
│                 │  (0%)        │  (0%)        │              │
├─────────────────┼──────────────┼──────────────┼──────────────┤
│ 编译通过率       │  166/310     │  308/310     │ +142 (+85%)  │
│                 │  (53.5%)     │  (99.4%) ✅  │              │
└─────────────────┴──────────────┴──────────────┴──────────────┘
```

---

## ✨ Key Successes (C2RUST)

### Perfect Achievement (5/5 in all evaluations)
- **32 examples** show perfect 编译通过率 results
- **308 examples** pass MIRI (99.4% success rate)
- **Only 2 failures** in core compilation metrics

### Major Improvements (0/5 → 5/5)
1. `global_check____lock_mismatch` - MIRI improvement
2. `struct_init` - MIRI improvement
3. `struct_malloc` - MIRI improvement
4. `struct_timedwait____lost_wakeup` - MIRI improvement
5. `array_const____lock_mismatch` - MIRI 1/5 → 5/5

---

## ⚠️ Persistent Issues

### 1. **LOOM Testing** (13.5% - No improvement)
- **Status**: Critical - same failures in both groups
- **Root Cause**: Parser/API compatibility issues
- **Most Common Errors**:
  - `error: prefix 'pub' is unknown` (3 occurrences)
  - `cannot find value 'num_mutex' in this scope` (25+ occurrences)
  - `no method named 'lock' found for struct` (20+ occurrences)
- **Impact**: 268/310 examples fail
- **Recommendation**: Requires parser/API refactoring

### 2. **OUTPUT Verification** (32.6% - Partial improvement)
- **Status**: Warning - improved but still problematic
- **Improved**: 38/310 → 101/310 (+166%)
- **Common Failures**:
  - Output mismatches (expected `4 4 4 4 4`, got `2 2 2 2 2`)
  - Synchronization logic issues
  - Race condition manifestations
- **Examples**: `array_main`, `array_const____deadlock`
- **Recommendation**: Fix concurrency semantics

### 3. **CLIPPY Linting** (0% - Systematic Issue)
- **Status**: Critical - 100% failure rate
- **Pattern**: All 310 examples fail
- **Possible Causes**:
  - Configuration error
  - Missing dependencies
  - Incompatible Rust version
- **Impact**: No concurrency lint checking
- **Recommendation**: Investigate tool setup

---

## 💡 Strategic Recommendations

### Priority 1: URGENT (Do First)
1. **Investigate CLIPPY Failures**
   - 0% pass rate suggests configuration error
   - Verify tool installation and Rust toolchain
   - Expected: Should show warnings, not fail

2. **Fix Symbol Resolution Issues** 
   - 139 occurrences of "cannot find symbol"
   - Check code generation logic
   - Verify variable initialization sequence

### Priority 2: HIGH (Critical Path)
3. **Stabilize LOOM Testing**
   - Currently 13.5% success with high variability
   - Same failures in both groups suggest systematic issue
   - Consider parser/API compatibility layer

4. **Improve OUTPUT Verification**
   - Currently 32.6% with c2rust
   - Concurrency semantic issues affecting 200+ examples
   - Focus on synchronization correctness

### Priority 3: MEDIUM (Performance)
5. **Replicate C2RUST MIRI Success**
   - 99.4% success is excellent
   - Document what makes it work
   - Apply patterns to other evaluation types

---

## 📁 Report Files in `/result/summary/`

| File | Size | Purpose |
|------|------|---------|
| `comprehensive_summary.json` | 344K | Structured data for analysis |
| `comprehensive_summary.txt` | 135K | Human-readable detailed report |
| `results_visualization.html` | 281K | Interactive web dashboard |
| `README.md` | 6.3K | Usage guide and interpretation |
| `FINDINGS.md` | This file | Executive summary |

---

## 📞 Quick Access

**View interactive dashboard:**
```bash
open results_visualization.html
```

**View detailed text report:**
```bash
less comprehensive_summary.txt
```

**Analyze JSON programmatically:**
```python
import json
data = json.load(open('comprehensive_summary.json'))
print(data['groups']['c2rust']['summary'])
```

---

**Report Generated**: 2026-04-21  
**Analysis Date**: 2026-04-21
