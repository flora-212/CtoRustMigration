# Concurrency Transformation Comparison Report (Negative Samples Only)

Analyzing **Original** and **LLM** for negative examples (expected to fail)

## Summary Overview

| # | Example | Type | Compiles (L) | Round | Pos | Pos Round | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_thread | lines |
|---|---------|------|:----------:|:---:|:--:|:----------:|------|------|------|------|------|------|------|------|
| 1 | [array_const____deadlock](#array_const____deadlock) | NEG | ✅ | 2 | ❌ | c2rust | 10→0 | 64→0 | 58→0 | 2→0 | 0→0 | 0→7 | 0→1 | 233→79 |
| 2 | [array_const____lock_mismatch](#array_const____lock_mismatch) | NEG | ❌ | c2rust | ❌ | c2rust | 7→5 | 42→0 | 41→4 | 2→0 | 0→4 | 0→8 | 0→1 | 183→70 |
| 3 | [array_main____lock_leak](#array_main____lock_leak) | NEG | ❌ | c2rust | ✅ | 1 | 7→4 | 27→0 | 34→2 | 2→2 | 0→4 | 0→7 | 0→1 | 207→67 |
| 4 | [array_main____partial_critical_section](#array_main____partial_critical_section) | NEG | ❌ | c2rust | ✅ | 1 | 7→4 | 32→0 | 37→2 | 2→2 | 0→4 | 0→7 | 0→1 | 212→72 |
| 5 | [array_simple____partial_critical_section](#array_simple____partial_critical_section) | NEG | ❌ | c2rust | ✅ | 1 | 7→6 | 22→0 | 33→2 | 4→4 | 0→6 | 0→7 | 0→1 | 227→85 |
| 6 | [global_assume2____self_lock](#global_assume2____self_lock) | NEG | ❌ | c2rust | ❌ | c2rust | 8→7 | 26→15 | 25→10 | 2→2 | 0→0 | 0→1 | 0→0 | 124→132 |
| 7 | [global_assume____lock_leak](#global_assume____lock_leak) | NEG | ❌ | c2rust | ✅ | 1 | 8→6 | 21→0 | 24→3 | 2→1 | 0→5 | 0→3 | 0→1 | 115→64 |
| 8 | [global_check____lock_leak](#global_check____lock_leak) | NEG | ✅ | 3 | ✅ | 11 | 10→0 | 39→0 | 26→0 | 2→0 | 0→0 | 0→5 | 0→2 | 176→82 |
| 9 | [global_check____lock_mismatch](#global_check____lock_mismatch) | NEG | ❌ | c2rust | ✅ | 11 | 10→7 | 46→0 | 28→3 | 3→0 | 0→5 | 0→6 | 0→2 | 197→133 |
| 10 | [global_condvar____lost_wakeup](#global_condvar____lost_wakeup) | NEG | ✅ | 2 | ✅ | 2 | 7→0 | 38→0 | 28→0 | 4→0 | 0→0 | 0→7 | 0→2 | 187→62 |
| 11 | [global_condvar____partial_critical_section](#global_condvar____partial_critical_section) | NEG | ✅ | 2 | ✅ | 2 | 7→0 | 40→0 | 29→0 | 4→0 | 0→0 | 0→7 | 0→2 | 190→61 |
| 12 | [global_custom____self_lock](#global_custom____self_lock) | NEG | ✅ | 3 | ✅ | 2 | 12→4 | 26→0 | 25→4 | 2→2 | 0→4 | 0→4 | 0→2 | 144→92 |
| 13 | [global_main____self_lock](#global_main____self_lock) | NEG | ✅ | 2 | ✅ | 1 | 7→4 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→2 | 117→61 |
| 14 | [global_nested____deadlock](#global_nested____deadlock) | NEG | ✅ | 1 | ✅ | 3 | 8→2 | 48→0 | 28→3 | 4→0 | 0→6 | 0→5 | 0→2 | 175→72 |
| 15 | [global_read____lock_mismatch](#global_read____lock_mismatch) | NEG | ✅ | 1 | ✅ | 6 | 8→2 | 28→0 | 28→3 | 4→0 | 0→6 | 0→5 | 0→2 | 141→59 |
| 16 | [global_rwlock____lock_leak](#global_rwlock____lock_leak) | NEG | ❌ | c2rust | ❌ | c2rust | 8→6 | 27→0 | 22→2 | 2→1 | 0→1 | 0→5 | 0→2 | 124→89 |
| 17 | [global_simple____partial_critical_section](#global_simple____partial_critical_section) | NEG | ❌ | c2rust | ❌ | c2rust | 7→5 | 24→0 | 25→5 | 4→0 | 0→6 | 0→6 | 0→2 | 125→72 |
| 18 | [global_while____lock_leak](#global_while____lock_leak) | NEG | ❌ | c2rust | ✅ | 2 | 8→0 | 27→0 | 25→0 | 2→0 | 0→0 | 0→5 | 0→2 | 131→57 |
| 19 | [struct_alias____self_lock](#struct_alias____self_lock) | NEG | ❌ | c2rust | ✅ | 5 | 10→3 | 28→0 | 32→2 | 3→0 | 0→4 | 0→5 | 0→2 | 187→100 |
| 20 | [struct_assume____deadlock](#struct_assume____deadlock) | NEG | ❌ | c2rust | ❌ | c2rust | 10→0 | 37→0 | 45→0 | 0→0 | 0→0 | 0→4 | 0→2 | 146→67 |
| 21 | [struct_condvar____lost_wakeup](#struct_condvar____lost_wakeup) | NEG | ❌ | c2rust | ❌ | c2rust | 7→4 | 32→0 | 27→3 | 1→0 | 0→2 | 0→3 | 0→1 | 185→58 |
| 22 | [struct_dup____deadlock](#struct_dup____deadlock) | NEG | ✅ | 9 | ✅ | 6 | 8→6 | 32→0 | 29→4 | 2→0 | 0→2 | 0→9 | 0→2 | 180→107 |
| 23 | [struct_init____partial_critical_section](#struct_init____partial_critical_section) | NEG | ✅ | 5 | ❌ | c2rust | 7→5 | 29→0 | 34→6 | 2→2 | 0→5 | 0→10 | 0→2 | 157→77 |
| 24 | [struct_malloc2____lock_mismatch](#struct_malloc2____lock_mismatch) | NEG | ✅ | 4 | ✅ | 3 | 8→5 | 35→0 | 34→2 | 2→0 | 0→3 | 0→8 | 0→2 | 145→68 |
| 25 | [struct_malloc____lost_wakeup](#struct_malloc____lost_wakeup) | NEG | ❌ | c2rust | ❌ | c2rust | 7→4 | 41→0 | 38→5 | 0→0 | 0→0 | 0→3 | 0→2 | 158→55 |
| 26 | [struct_multiple____deadlock](#struct_multiple____deadlock) | NEG | ✅ | 3 | ❌ | c2rust | 11→0 | 32→0 | 37→0 | 3→0 | 0→0 | 0→5 | 0→3 | 198→73 |
| 27 | [struct_nested____self_lock](#struct_nested____self_lock) | NEG | ✅ | 3 | ✅ | 6 | 7→4 | 26→0 | 25→3 | 1→0 | 0→4 | 0→5 | 0→2 | 138→65 |
| 28 | [struct_simple____partial_critical_section](#struct_simple____partial_critical_section) | NEG | ✅ | 5 | ✅ | 3 | 7→3 | 28→0 | 27→1 | 1→1 | 0→3 | 0→10 | 0→2 | 158→90 |
| 29 | [struct_spin____lock_leak](#struct_spin____lock_leak) | NEG | ❌ | c2rust | ✅ | 1 | 7→2 | 47→0 | 33→6 | 0→0 | 0→0 | 0→7 | 0→2 | 199→64 |
| 30 | [struct_timedwait____deadlock](#struct_timedwait____deadlock) | NEG | ✅ | 3 | ❌ | c2rust | 9→6 | 48→0 | 32→0 | 2→1 | 0→2 | 0→9 | 0→2 | 271→117 |
| 31 | [struct_timedwait____lost_wakeup](#struct_timedwait____lost_wakeup) | NEG | ❌ | c2rust | ❌ | c2rust | 9→1 | 37→0 | 29→0 | 1→1 | 0→0 | 0→3 | 0→2 | 238→81 |
| 32 | [unused_func____lock_mismatch](#unused_func____lock_mismatch) | NEG | ✅ | 2 | ✅ | 1 | 8→6 | 28→0 | 27→3 | 3→1 | 0→5 | 0→5 | 0→1 | 135→70 |
| | **TOTAL** | (NEG) | 16/32 | — | 20/30 | — | 261→222 | 1083→30 | 990→162 | 70→42 | 0→170 | 0→370 | 0→110 | 5503→5002 |

> **Reading the table**: Each metric cell shows **Original → LLM**. **Pos** column shows if the corresponding positive sample (before `____`) compiles with LLM. **Pos Round** shows the last successful round (1-N) for the positive sample, or `c2rust` if none compiled successfully. Negative samples are expected to fail (used for validation).

## All Metrics Summary

This section displays all 15 metrics for each sample in a compact format.

| Example | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_arc | std\_rwlock | std\_condvar | std\_thread | move\_closure | arc\_clone | join\_handle | arc\_mutex\_combo | lines |
|---------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| array_const____deadlock | 10→0 | 64→0 | 58→0 | 2→0 | 0→0 | 0→7 | 0→4 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→1 | 233→79 |
| array_const____lock_mismatch | 7→5 | 42→0 | 41→4 | 2→0 | 0→4 | 0→8 | 0→9 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→3 | 183→70 |
| array_main____lock_leak | 7→4 | 27→0 | 34→2 | 2→2 | 0→4 | 0→7 | 0→7 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→1 | 207→67 |
| array_main____partial_critical_section | 7→4 | 32→0 | 37→2 | 2→2 | 0→4 | 0→7 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 212→72 |
| array_simple____partial_critical_section | 7→6 | 22→0 | 33→2 | 4→4 | 0→6 | 0→7 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 227→85 |
| global_assume2____self_lock | 8→7 | 26→15 | 25→10 | 2→2 | 0→0 | 0→1 | 0→1 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 124→132 |
| global_assume____lock_leak | 8→6 | 21→0 | 24→3 | 2→1 | 0→5 | 0→3 | 0→3 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→0 | 115→64 |
| global_check____lock_leak | 10→0 | 39→0 | 26→0 | 2→0 | 0→0 | 0→5 | 0→5 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 176→82 |
| global_check____lock_mismatch | 10→7 | 46→0 | 28→3 | 3→0 | 0→5 | 0→6 | 0→6 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→2 | 197→133 |
| global_condvar____lost_wakeup | 7→0 | 38→0 | 28→0 | 4→0 | 0→0 | 0→7 | 0→6 | 0→0 | 0→3 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 187→62 |
| global_condvar____partial_critical_section | 7→0 | 40→0 | 29→0 | 4→0 | 0→0 | 0→7 | 0→5 | 0→0 | 0→3 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 190→61 |
| global_custom____self_lock | 12→4 | 26→0 | 25→4 | 2→2 | 0→4 | 0→4 | 0→6 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→2 | 144→92 |
| global_main____self_lock | 7→4 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→1 | 0→0 | 0→0 | 0→2 | 0→0 | 0→0 | 0→2 | 0→0 | 117→61 |
| global_nested____deadlock | 8→2 | 48→0 | 28→3 | 4→0 | 0→6 | 0→5 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 175→72 |
| global_read____lock_mismatch | 8→2 | 28→0 | 28→3 | 4→0 | 0→6 | 0→5 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 141→59 |
| global_rwlock____lock_leak | 8→6 | 27→0 | 22→2 | 2→1 | 0→1 | 0→5 | 0→1 | 0→5 | 0→0 | 0→2 | 0→0 | 0→0 | 0→2 | 0→0 | 124→89 |
| global_simple____partial_critical_section | 7→5 | 24→0 | 25→5 | 4→0 | 0→6 | 0→6 | 0→8 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→1 | 125→72 |
| global_while____lock_leak | 8→0 | 27→0 | 25→0 | 2→0 | 0→0 | 0→5 | 0→5 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→0 | 131→57 |
| struct_alias____self_lock | 10→3 | 28→0 | 32→2 | 3→0 | 0→4 | 0→5 | 0→10 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→0 | 187→100 |
| struct_assume____deadlock | 10→0 | 37→0 | 45→0 | 0→0 | 0→0 | 0→4 | 0→10 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 146→67 |
| struct_condvar____lost_wakeup | 7→4 | 32→0 | 27→3 | 1→0 | 0→2 | 0→3 | 0→3 | 0→0 | 0→3 | 0→1 | 0→1 | 0→1 | 0→1 | 0→0 | 185→58 |
| struct_dup____deadlock | 8→6 | 32→0 | 29→4 | 2→0 | 0→2 | 0→9 | 0→5 | 0→0 | 0→0 | 0→2 | 0→0 | 0→0 | 0→2 | 0→2 | 180→107 |
| struct_init____partial_critical_section | 7→5 | 29→0 | 34→6 | 2→2 | 0→5 | 0→10 | 0→10 | 0→0 | 0→0 | 0→2 | 0→2 | 0→4 | 0→2 | 0→2 | 157→77 |
| struct_malloc2____lock_mismatch | 8→5 | 35→0 | 34→2 | 2→0 | 0→3 | 0→8 | 0→6 | 0→0 | 0→0 | 0→2 | 0→0 | 0→2 | 0→2 | 0→1 | 145→68 |
| struct_malloc____lost_wakeup | 7→4 | 41→0 | 38→5 | 0→0 | 0→0 | 0→3 | 0→8 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 158→55 |
| struct_multiple____deadlock | 11→0 | 32→0 | 37→0 | 3→0 | 0→0 | 0→5 | 0→12 | 0→0 | 0→0 | 0→3 | 0→3 | 0→3 | 0→3 | 0→0 | 198→73 |
| struct_nested____self_lock | 7→4 | 26→0 | 25→3 | 1→0 | 0→4 | 0→5 | 0→6 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→1 | 138→65 |
| struct_simple____partial_critical_section | 7→3 | 28→0 | 27→1 | 1→1 | 0→3 | 0→10 | 0→6 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→4 | 158→90 |
| struct_spin____lock_leak | 7→2 | 47→0 | 33→6 | 0→0 | 0→0 | 0→7 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→3 | 199→64 |
| struct_timedwait____deadlock | 9→6 | 48→0 | 32→0 | 2→1 | 0→2 | 0→9 | 0→7 | 0→0 | 0→2 | 0→2 | 0→2 | 0→3 | 0→2 | 0→0 | 271→117 |
| struct_timedwait____lost_wakeup | 9→1 | 37→0 | 29→0 | 1→1 | 0→0 | 0→3 | 0→8 | 0→0 | 0→2 | 0→2 | 0→2 | 0→5 | 0→2 | 0→0 | 238→81 |
| unused_func____lock_mismatch | 8→6 | 28→0 | 27→3 | 3→1 | 0→5 | 0→5 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 135→70 |
| **TOTAL** | 261→222 | 1083→30 | 990→162 | 70→42 | 0→170 | 0→370 | 0→364 | 0→10 | 0→30 | 0→110 | 0→86 | 0→112 | 0→110 | 0→46 | 5503→5002 |

> **All Metrics** table shows all 15 metrics (including std\_arc, std\_rwlock, std\_condvar, move\_closure, arc\_clone, join\_handle, arc\_mutex\_combo) for each sample. Format: **Original → ConCrat → LLM** (or **Original → LLM** for negative samples).

## Aggregate Statistics

| Metric | Original | LLM | vs Original |
|--------|----------|-----|:------------:|
| unsafe | 261 | 222 | +14.9% |
| pthread | 1083 | 30 | +97.2% |
| raw\_ptr | 990 | 162 | +83.6% |
| static\_mut | 70 | 42 | +40.0% |
| libc | 0 | 170 | — |
| lines | 5503 | 5002 | +9.1% |

| **LLM compile success** | — | 16/32 (50%) |  |

## Metric Categories Summary

Aggregate of metrics grouped by category (Lower is Better vs Higher is Better):

| Category | Original | LLM | vs Original |
|----------|----------|-----|:------------:|
| **Lower is Better** (∑unsafe, pthread, raw_ptr, static_mut, libc, lines) | 7907 | 5628 | +28.8% |
| **Higher is Better** (∑std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 1238 | — |

## Per-Example Details

### array_const____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 0 | -10 |
| pthread | 64 | 0 | -64 |
| raw_ptr | 58 | 0 | -58 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 233 | 79 | -154 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 367 | 79 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 16 |

---

### array_const____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 5 | -2 |
| pthread | 42 | 0 | -42 |
| raw_ptr | 41 | 4 | -37 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 4 | +4 |
| lines | 183 | 70 | -113 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 275 | 83 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 24 |

**Remaining Issues:**

- **LLM**: 5 unsafe keyword(s) remain

---

### array_main____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 27 | 0 | -27 |
| raw_ptr | 34 | 2 | -32 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 4 | +4 |
| lines | 207 | 67 | -140 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 277 | 79 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 17 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, num_mutex; 4 unsafe keyword(s) remain

---

### array_main____partial_critical_section

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 37 | 2 | -35 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 4 | +4 |
| lines | 212 | 72 | -140 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 290 | 84 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 10 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, num_mutex; 4 unsafe keyword(s) remain

---

### array_simple____partial_critical_section

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 6 | -1 |
| pthread | 22 | 0 | -22 |
| raw_ptr | 33 | 2 | -31 |
| static_mut | 4 | 4 | +0 |
| libc | 0 | 6 | +6 |
| lines | 227 | 85 | -142 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 293 | 103 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 10 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, n2, n3, num_mutex; 6 unsafe keyword(s) remain

---

### global_assume2____self_lock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 7 | -1 |
| pthread | 26 | 15 | -11 |
| raw_ptr | 25 | 10 | -15 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 124 | 132 | +8 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 166 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 2 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (4 calls); static mut variables remain: n1, num_mutex; 7 unsafe keyword(s) remain

---

### global_assume____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 6 | -2 |
| pthread | 21 | 0 | -21 |
| raw_ptr | 24 | 3 | -21 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 5 | +5 |
| lines | 115 | 64 | -51 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 170 | 79 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 10 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 6 unsafe keyword(s) remain

---

### global_check____lock_leak

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 0 | -10 |
| pthread | 39 | 0 | -39 |
| raw_ptr | 26 | 0 | -26 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 176 | 82 | -94 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 253 | 82 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 18 |

---

### global_check____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 7 | -3 |
| pthread | 46 | 0 | -46 |
| raw_ptr | 28 | 3 | -25 |
| static_mut | 3 | 0 | -3 |
| libc | 0 | 5 | +5 |
| lines | 197 | 133 | -64 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 284 | 148 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 23 |

**Remaining Issues:**

- **LLM**: 7 unsafe keyword(s) remain

---

### global_condvar____lost_wakeup

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 0 | -7 |
| pthread | 38 | 0 | -38 |
| raw_ptr | 28 | 0 | -28 |
| static_mut | 4 | 0 | -4 |
| libc | 0 | 0 | +0 |
| lines | 187 | 62 | -125 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 264 | 62 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 24 |

---

### global_condvar____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 0 | -7 |
| pthread | 40 | 0 | -40 |
| raw_ptr | 29 | 0 | -29 |
| static_mut | 4 | 0 | -4 |
| libc | 0 | 0 | +0 |
| lines | 190 | 61 | -129 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 270 | 61 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 23 |

---

### global_custom____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 12 | 4 | -8 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 4 | -21 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 4 | +4 |
| lines | 144 | 92 | -52 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 209 | 106 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1, NUM_MUTEX_ARC; 4 unsafe keyword(s) remain

---

### global_main____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 3 | -22 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 4 | +4 |
| lines | 117 | 61 | -56 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 177 | 73 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 9 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 4 unsafe keyword(s) remain

---

### global_nested____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 2 | -6 |
| pthread | 48 | 0 | -48 |
| raw_ptr | 28 | 3 | -25 |
| static_mut | 4 | 0 | -4 |
| libc | 0 | 6 | +6 |
| lines | 175 | 72 | -103 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 263 | 83 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

---

### global_read____lock_mismatch

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 2 | -6 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 28 | 3 | -25 |
| static_mut | 4 | 0 | -4 |
| libc | 0 | 6 | +6 |
| lines | 141 | 59 | -82 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 209 | 70 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

---

### global_rwlock____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 6 | -2 |
| pthread | 27 | 0 | -27 |
| raw_ptr | 22 | 2 | -20 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 1 | +1 |
| lines | 124 | 89 | -35 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 183 | 99 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 15 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N; 6 unsafe keyword(s) remain

---

### global_simple____partial_critical_section

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 5 | -2 |
| pthread | 24 | 0 | -24 |
| raw_ptr | 25 | 5 | -20 |
| static_mut | 4 | 0 | -4 |
| libc | 0 | 6 | +6 |
| lines | 125 | 72 | -53 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 88 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 23 |

**Remaining Issues:**

- **LLM**: 5 unsafe keyword(s) remain

---

### global_while____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 0 | -8 |
| pthread | 27 | 0 | -27 |
| raw_ptr | 25 | 0 | -25 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 131 | 57 | -74 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 193 | 57 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 19 |

---

### struct_alias____self_lock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 3 | -7 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 32 | 2 | -30 |
| static_mut | 3 | 0 | -3 |
| libc | 0 | 4 | +4 |
| lines | 187 | 100 | -87 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 260 | 109 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 24 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### struct_assume____deadlock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 0 | -10 |
| pthread | 37 | 0 | -37 |
| raw_ptr | 45 | 0 | -45 |
| static_mut | 0 | 0 | +0 |
| libc | 0 | 0 | +0 |
| lines | 146 | 67 | -79 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 238 | 67 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 22 |

---

### struct_condvar____lost_wakeup

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 27 | 3 | -24 |
| static_mut | 1 | 0 | -1 |
| libc | 0 | 2 | +2 |
| lines | 185 | 58 | -127 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 252 | 67 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 13 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_dup____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 6 | -2 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 29 | 4 | -25 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 2 | +2 |
| lines | 180 | 107 | -73 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 251 | 119 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: 6 unsafe keyword(s) remain

---

### struct_init____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 5 | -2 |
| pthread | 29 | 0 | -29 |
| raw_ptr | 34 | 6 | -28 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 5 | +5 |
| lines | 157 | 77 | -80 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 229 | 95 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 32 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S1, S2; 5 unsafe keyword(s) remain

---

### struct_malloc2____lock_mismatch

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 5 | -3 |
| pthread | 35 | 0 | -35 |
| raw_ptr | 34 | 2 | -32 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 3 | +3 |
| lines | 145 | 68 | -77 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 224 | 78 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 21 |

**Remaining Issues:**

- **LLM**: 5 unsafe keyword(s) remain

---

### struct_malloc____lost_wakeup

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 41 | 0 | -41 |
| raw_ptr | 38 | 5 | -33 |
| static_mut | 0 | 0 | +0 |
| libc | 0 | 0 | +0 |
| lines | 158 | 55 | -103 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 244 | 64 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 21 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_multiple____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 11 | 0 | -11 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 37 | 0 | -37 |
| static_mut | 3 | 0 | -3 |
| libc | 0 | 0 | +0 |
| lines | 198 | 73 | -125 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 281 | 73 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 29 |

---

### struct_nested____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 3 | -22 |
| static_mut | 1 | 0 | -1 |
| libc | 0 | 4 | +4 |
| lines | 138 | 65 | -73 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 197 | 76 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 21 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_simple____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 3 | -4 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 27 | 1 | -26 |
| static_mut | 1 | 1 | +0 |
| libc | 0 | 3 | +3 |
| lines | 158 | 90 | -68 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 221 | 98 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 29 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 3 unsafe keyword(s) remain

---

### struct_spin____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 2 | -5 |
| pthread | 47 | 0 | -47 |
| raw_ptr | 33 | 6 | -27 |
| static_mut | 0 | 0 | +0 |
| libc | 0 | 0 | +0 |
| lines | 199 | 64 | -135 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 286 | 72 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 25 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

---

### struct_timedwait____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 6 | -3 |
| pthread | 48 | 0 | -48 |
| raw_ptr | 32 | 0 | -32 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 2 | +2 |
| lines | 271 | 117 | -154 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 362 | 126 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 27 |

**Remaining Issues:**

- **LLM**: static mut variables remain: SHARED_DATA_INSTANCE; 6 unsafe keyword(s) remain

---

### struct_timedwait____lost_wakeup

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 1 | -8 |
| pthread | 37 | 0 | -37 |
| raw_ptr | 29 | 0 | -29 |
| static_mut | 1 | 1 | +0 |
| libc | 0 | 0 | +0 |
| lines | 238 | 81 | -157 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 314 | 83 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 24 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 1 unsafe keyword(s) remain

---

### unused_func____lock_mismatch

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 6 | -2 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 27 | 3 | -24 |
| static_mut | 3 | 1 | -2 |
| libc | 0 | 5 | +5 |
| lines | 135 | 70 | -65 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 201 | 85 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 8 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 6 unsafe keyword(s) remain

---
