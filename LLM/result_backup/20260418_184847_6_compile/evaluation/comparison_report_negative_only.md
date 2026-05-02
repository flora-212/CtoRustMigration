# Concurrency Transformation Comparison Report (Negative Samples Only)

Analyzing **Original** and **LLM** for negative examples (expected to fail)

## Summary Overview

| # | Example | Type | Compiles (L) | Round | Pos | Pos Round | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_thread | lines |
|---|---------|------|:----------:|:---:|:--:|:----------:|------|------|------|------|------|------|------|------|
| 1 | [array_const____deadlock](#array_const____deadlock) | NEG | ✅ | 1 | ❌ | c2rust | 10→0 | 64→0 | 58→0 | 2→0 | 0→0 | 0→7 | 0→1 | 233→72 |
| 2 | [array_const____lock_mismatch](#array_const____lock_mismatch) | NEG | ✅ | 2 | ❌ | c2rust | 7→0 | 42→0 | 41→0 | 2→0 | 0→0 | 0→5 | 0→1 | 183→55 |
| 3 | [array_main____lock_leak](#array_main____lock_leak) | NEG | ❌ | c2rust | ✅ | 4 | 7→5 | 27→0 | 34→2 | 2→2 | 0→4 | 0→12 | 0→1 | 207→87 |
| 4 | [array_main____partial_critical_section](#array_main____partial_critical_section) | NEG | ❌ | c2rust | ✅ | 4 | 7→4 | 32→0 | 37→2 | 2→2 | 0→4 | 0→7 | 0→1 | 212→68 |
| 5 | [array_simple____partial_critical_section](#array_simple____partial_critical_section) | NEG | ✅ | 1 | ❌ | c2rust | 7→6 | 22→0 | 33→2 | 4→4 | 0→3 | 0→7 | 0→1 | 227→80 |
| 6 | [global_assume2____self_lock](#global_assume2____self_lock) | NEG | ✅ | 6 | ❌ | c2rust | 8→4 | 26→0 | 25→0 | 2→0 | 0→0 | 0→8 | 0→2 | 124→71 |
| 7 | [global_assume____lock_leak](#global_assume____lock_leak) | NEG | ❌ | c2rust | ✅ | 1 | 8→3 | 21→0 | 24→0 | 2→1 | 0→0 | 0→4 | 0→2 | 115→64 |
| 8 | [global_check____lock_leak](#global_check____lock_leak) | NEG | ❌ | c2rust | ✅ | 3 | 10→0 | 39→0 | 26→0 | 2→0 | 0→0 | 0→3 | 0→2 | 176→79 |
| 9 | [global_check____lock_mismatch](#global_check____lock_mismatch) | NEG | ❌ | c2rust | ✅ | 3 | 10→9 | 46→30 | 28→9 | 3→3 | 0→0 | 0→1 | 0→0 | 197→196 |
| 10 | [global_condvar____lost_wakeup](#global_condvar____lost_wakeup) | NEG | ❌ | c2rust | ✅ | 2 | 7→4 | 38→6 | 28→2 | 4→2 | 0→10 | 0→3 | 0→0 | 187→113 |
| 11 | [global_condvar____partial_critical_section](#global_condvar____partial_critical_section) | NEG | ✅ | 2 | ✅ | 2 | 7→0 | 40→0 | 29→0 | 4→0 | 0→0 | 0→7 | 0→2 | 190→61 |
| 12 | [global_custom____self_lock](#global_custom____self_lock) | NEG | ❌ | c2rust | ✅ | 3 | 12→4 | 26→0 | 25→5 | 2→0 | 0→6 | 0→5 | 0→1 | 144→86 |
| 13 | [global_main____self_lock](#global_main____self_lock) | NEG | ✅ | 1 | ✅ | 5 | 7→7 | 26→0 | 25→3 | 2→1 | 0→5 | 0→4 | 0→1 | 117→74 |
| 14 | [global_nested____deadlock](#global_nested____deadlock) | NEG | ✅ | 2 | ❌ | c2rust | 8→7 | 48→0 | 28→3 | 4→4 | 0→6 | 0→5 | 0→2 | 175→93 |
| 15 | [global_read____lock_mismatch](#global_read____lock_mismatch) | NEG | ❌ | c2rust | ✅ | 6 | 8→5 | 28→0 | 28→4 | 4→2 | 0→6 | 0→5 | 0→2 | 141→64 |
| 16 | [global_rwlock____lock_leak](#global_rwlock____lock_leak) | NEG | ❌ | c2rust | ✅ | 5 | 8→6 | 27→0 | 22→2 | 2→1 | 0→1 | 0→5 | 0→2 | 124→92 |
| 17 | [global_simple____partial_critical_section](#global_simple____partial_critical_section) | NEG | ✅ | 3 | ✅ | 7 | 7→2 | 24→0 | 25→1 | 4→0 | 0→3 | 0→9 | 0→2 | 125→64 |
| 18 | [global_while____lock_leak](#global_while____lock_leak) | NEG | ❌ | c2rust | ❌ | c2rust | 8→0 | 27→0 | 25→0 | 2→0 | 0→0 | 0→3 | 0→2 | 131→54 |
| 19 | [struct_alias____self_lock](#struct_alias____self_lock) | NEG | ❌ | c2rust | ❌ | c2rust | 10→7 | 28→0 | 32→6 | 3→0 | 0→2 | 0→14 | 0→2 | 187→100 |
| 20 | [struct_assume____deadlock](#struct_assume____deadlock) | NEG | ❌ | c2rust | ❌ | c2rust | 10→0 | 37→0 | 45→0 | 0→0 | 0→0 | 0→4 | 0→2 | 146→84 |
| 21 | [struct_condvar____lost_wakeup](#struct_condvar____lost_wakeup) | NEG | ❌ | c2rust | ✅ | 9 | 7→6 | 32→0 | 27→3 | 1→1 | 0→5 | 0→3 | 0→2 | 185→71 |
| 22 | [struct_dup____deadlock](#struct_dup____deadlock) | NEG | ❌ | c2rust | ✅ | 5 | 8→6 | 32→0 | 29→3 | 2→2 | 0→7 | 0→15 | 0→2 | 180→106 |
| 23 | [struct_init____partial_critical_section](#struct_init____partial_critical_section) | NEG | ❌ | c2rust | ❌ | c2rust | 7→6 | 29→0 | 34→7 | 2→0 | 0→7 | 0→10 | 0→2 | 157→66 |
| 24 | [struct_malloc2____lock_mismatch](#struct_malloc2____lock_mismatch) | NEG | ✅ | 4 | ❌ | c2rust | 8→11 | 35→0 | 34→2 | 2→1 | 0→4 | 0→6 | 0→2 | 145→79 |
| 25 | [struct_malloc____lost_wakeup](#struct_malloc____lost_wakeup) | NEG | ❌ | c2rust | ❌ | c2rust | 7→3 | 41→0 | 38→5 | 0→0 | 0→0 | 0→3 | 0→2 | 158→54 |
| 26 | [struct_multiple____deadlock](#struct_multiple____deadlock) | NEG | ✅ | 9 | ✅ | 3 | 11→3 | 32→0 | 37→1 | 3→0 | 0→2 | 0→13 | 0→3 | 198→90 |
| 27 | [struct_nested____self_lock](#struct_nested____self_lock) | NEG | ✅ | 4 | ✅ | 3 | 7→3 | 26→0 | 25→2 | 1→0 | 0→4 | 0→5 | 0→2 | 138→68 |
| 28 | [struct_simple____partial_critical_section](#struct_simple____partial_critical_section) | NEG | ✅ | 4 | ✅ | 3 | 7→5 | 28→0 | 27→3 | 1→1 | 0→5 | 0→5 | 0→2 | 158→86 |
| 29 | [struct_spin____lock_leak](#struct_spin____lock_leak) | NEG | ✅ | 1 | ❌ | c2rust | 7→3 | 47→0 | 33→5 | 0→0 | 0→0 | 0→7 | 0→2 | 199→61 |
| 30 | [struct_timedwait____deadlock](#struct_timedwait____deadlock) | NEG | ❌ | c2rust | ❌ | c2rust | 9→6 | 48→0 | 32→2 | 2→0 | 0→6 | 0→3 | 0→2 | 271→115 |
| 31 | [struct_timedwait____lost_wakeup](#struct_timedwait____lost_wakeup) | NEG | ❌ | c2rust | ❌ | c2rust | 9→5 | 37→0 | 29→0 | 1→1 | 0→4 | 0→3 | 0→1 | 238→124 |
| 32 | [unused_func____lock_mismatch](#unused_func____lock_mismatch) | NEG | ❌ | c2rust | ✅ | 2 | 8→6 | 28→0 | 27→3 | 3→1 | 0→5 | 0→4 | 0→1 | 135→71 |
| | **TOTAL** | (NEG) | 13/32 | — | 18/30 | — | 261→272 | 1083→72 | 990→154 | 70→58 | 0→198 | 0→390 | 0→104 | 5503→5296 |

> **Reading the table**: Each metric cell shows **Original → LLM**. **Pos** column shows if the corresponding positive sample (before `____`) compiles with LLM. **Pos Round** shows the last successful round (1-N) for the positive sample, or `c2rust` if none compiled successfully. Negative samples are expected to fail (used for validation).

## All Metrics Summary

This section displays all 15 metrics for each sample in a compact format.

| Example | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_arc | std\_rwlock | std\_condvar | std\_thread | move\_closure | arc\_clone | join\_handle | arc\_mutex\_combo | lines |
|---------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| array_const____deadlock | 10→0 | 64→0 | 58→0 | 2→0 | 0→0 | 0→7 | 0→4 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→1 | 233→72 |
| array_const____lock_mismatch | 7→0 | 42→0 | 41→0 | 2→0 | 0→0 | 0→5 | 0→5 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→0 | 183→55 |
| array_main____lock_leak | 7→5 | 27→0 | 34→2 | 2→2 | 0→4 | 0→12 | 0→12 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→1 | 207→87 |
| array_main____partial_critical_section | 7→4 | 32→0 | 37→2 | 2→2 | 0→4 | 0→7 | 0→7 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→1 | 212→68 |
| array_simple____partial_critical_section | 7→6 | 22→0 | 33→2 | 4→4 | 0→3 | 0→7 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 227→80 |
| global_assume2____self_lock | 8→4 | 26→0 | 25→0 | 2→0 | 0→0 | 0→8 | 0→8 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→4 | 124→71 |
| global_assume____lock_leak | 8→3 | 21→0 | 24→0 | 2→1 | 0→0 | 0→4 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→4 | 0→2 | 0→0 | 115→64 |
| global_check____lock_leak | 10→0 | 39→0 | 26→0 | 2→0 | 0→0 | 0→3 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→5 | 0→2 | 0→0 | 176→79 |
| global_check____lock_mismatch | 10→9 | 46→30 | 28→9 | 3→3 | 0→0 | 0→1 | 0→1 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 197→196 |
| global_condvar____lost_wakeup | 7→4 | 38→6 | 28→2 | 4→2 | 0→10 | 0→3 | 0→1 | 0→0 | 0→5 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 187→113 |
| global_condvar____partial_critical_section | 7→0 | 40→0 | 29→0 | 4→0 | 0→0 | 0→7 | 0→6 | 0→0 | 0→3 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 190→61 |
| global_custom____self_lock | 12→4 | 26→0 | 25→5 | 2→0 | 0→6 | 0→5 | 0→6 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→0 | 144→86 |
| global_main____self_lock | 7→7 | 26→0 | 25→3 | 2→1 | 0→5 | 0→4 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 117→74 |
| global_nested____deadlock | 8→7 | 48→0 | 28→3 | 4→4 | 0→6 | 0→5 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→2 | 175→93 |
| global_read____lock_mismatch | 8→5 | 28→0 | 28→4 | 4→2 | 0→6 | 0→5 | 0→5 | 0→0 | 0→0 | 0→2 | 0→0 | 0→0 | 0→1 | 0→2 | 141→64 |
| global_rwlock____lock_leak | 8→6 | 27→0 | 22→2 | 2→1 | 0→1 | 0→5 | 0→1 | 0→5 | 0→0 | 0→2 | 0→0 | 0→0 | 0→2 | 0→0 | 124→92 |
| global_simple____partial_critical_section | 7→2 | 24→0 | 25→1 | 4→0 | 0→3 | 0→9 | 0→6 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 125→64 |
| global_while____lock_leak | 8→0 | 27→0 | 25→0 | 2→0 | 0→0 | 0→3 | 0→5 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→0 | 131→54 |
| struct_alias____self_lock | 10→7 | 28→0 | 32→6 | 3→0 | 0→2 | 0→14 | 0→10 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→6 | 187→100 |
| struct_assume____deadlock | 10→0 | 37→0 | 45→0 | 0→0 | 0→0 | 0→4 | 0→10 | 0→0 | 0→0 | 0→2 | 0→2 | 0→7 | 0→2 | 0→0 | 146→84 |
| struct_condvar____lost_wakeup | 7→6 | 32→0 | 27→3 | 1→1 | 0→5 | 0→3 | 0→3 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 185→71 |
| struct_dup____deadlock | 8→6 | 32→0 | 29→3 | 2→2 | 0→7 | 0→15 | 0→15 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→8 | 180→106 |
| struct_init____partial_critical_section | 7→6 | 29→0 | 34→7 | 2→0 | 0→7 | 0→10 | 0→10 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→2 | 157→66 |
| struct_malloc2____lock_mismatch | 8→11 | 35→0 | 34→2 | 2→1 | 0→4 | 0→6 | 0→4 | 0→0 | 0→0 | 0→2 | 0→0 | 0→3 | 0→2 | 0→0 | 145→79 |
| struct_malloc____lost_wakeup | 7→3 | 41→0 | 38→5 | 0→0 | 0→0 | 0→3 | 0→8 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 158→54 |
| struct_multiple____deadlock | 11→3 | 32→0 | 37→1 | 3→0 | 0→2 | 0→13 | 0→9 | 0→0 | 0→0 | 0→3 | 0→0 | 0→0 | 0→3 | 0→5 | 198→90 |
| struct_nested____self_lock | 7→3 | 26→0 | 25→2 | 1→0 | 0→4 | 0→5 | 0→3 | 0→0 | 0→0 | 0→2 | 0→2 | 0→4 | 0→2 | 0→1 | 138→68 |
| struct_simple____partial_critical_section | 7→5 | 28→0 | 27→3 | 1→1 | 0→5 | 0→5 | 0→3 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 158→86 |
| struct_spin____lock_leak | 7→3 | 47→0 | 33→5 | 0→0 | 0→0 | 0→7 | 0→12 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→3 | 199→61 |
| struct_timedwait____deadlock | 9→6 | 48→0 | 32→2 | 2→0 | 0→6 | 0→3 | 0→3 | 0→0 | 0→2 | 0→2 | 0→2 | 0→4 | 0→2 | 0→0 | 271→115 |
| struct_timedwait____lost_wakeup | 9→5 | 37→0 | 29→0 | 1→1 | 0→4 | 0→3 | 0→8 | 0→0 | 0→2 | 0→1 | 0→1 | 0→4 | 0→1 | 0→0 | 238→124 |
| unused_func____lock_mismatch | 8→6 | 28→0 | 27→3 | 3→1 | 0→5 | 0→4 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 135→71 |
| **TOTAL** | 261→272 | 1083→72 | 990→154 | 70→58 | 0→198 | 0→390 | 0→378 | 0→10 | 0→32 | 0→104 | 0→76 | 0→120 | 0→102 | 0→72 | 5503→5296 |

> **All Metrics** table shows all 15 metrics (including std\_arc, std\_rwlock, std\_condvar, move\_closure, arc\_clone, join\_handle, arc\_mutex\_combo) for each sample. Format: **Original → ConCrat → LLM** (or **Original → LLM** for negative samples).

## Aggregate Statistics

| Metric | Original | LLM | vs Original |
|--------|----------|-----|:------------:|
| unsafe | 261 | 272 | -4.2% |
| pthread | 1083 | 72 | +93.4% |
| raw\_ptr | 990 | 154 | +84.4% |
| static\_mut | 70 | 58 | +17.1% |
| libc | 0 | 198 | — |
| lines | 5503 | 5296 | +3.8% |

| **LLM compile success** | — | 13/32 (41%) |  |

## Metric Categories Summary

Aggregate of metrics grouped by category (Lower is Better vs Higher is Better):

| Category | Original | LLM | vs Original |
|----------|----------|-----|:------------:|
| **Lower is Better** (∑unsafe, pthread, raw_ptr, static_mut, libc, lines) | 7907 | 6050 | +23.5% |
| **Higher is Better** (∑std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 1284 | — |

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
| lines | 233 | 72 | -161 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 367 | 72 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 16 |

---

### array_const____lock_mismatch

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 0 | -7 |
| pthread | 42 | 0 | -42 |
| raw_ptr | 41 | 0 | -41 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 183 | 55 | -128 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 275 | 55 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 14 |

---

### array_main____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 5 | -2 |
| pthread | 27 | 0 | -27 |
| raw_ptr | 34 | 2 | -32 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 4 | +4 |
| lines | 207 | 87 | -120 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 277 | 100 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 27 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, num_mutex; 5 unsafe keyword(s) remain

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
| lines | 212 | 68 | -144 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 290 | 80 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 17 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, num_mutex; 4 unsafe keyword(s) remain

---

### array_simple____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 6 | -1 |
| pthread | 22 | 0 | -22 |
| raw_ptr | 33 | 2 | -31 |
| static_mut | 4 | 4 | +0 |
| libc | 0 | 3 | +3 |
| lines | 227 | 80 | -147 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 293 | 95 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 10 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, n2, n3, num_mutex; 6 unsafe keyword(s) remain

---

### global_assume2____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 4 | -4 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 0 | -25 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 124 | 71 | -53 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 75 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 28 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### global_assume____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 3 | -5 |
| pthread | 21 | 0 | -21 |
| raw_ptr | 24 | 0 | -24 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 0 | +0 |
| lines | 115 | 64 | -51 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 170 | 68 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 21 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 3 unsafe keyword(s) remain

---

### global_check____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 0 | -10 |
| pthread | 39 | 0 | -39 |
| raw_ptr | 26 | 0 | -26 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 176 | 79 | -97 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 253 | 79 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 21 |

---

### global_check____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 9 | -1 |
| pthread | 46 | 30 | -16 |
| raw_ptr | 28 | 9 | -19 |
| static_mut | 3 | 3 | +0 |
| libc | 0 | 0 | +0 |
| lines | 197 | 196 | -1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 284 | 247 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 2 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (17 calls); static mut variables remain: n, m, r; 9 unsafe keyword(s) remain

---

### global_condvar____lost_wakeup

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 38 | 6 | -32 |
| raw_ptr | 28 | 2 | -26 |
| static_mut | 4 | 2 | -2 |
| libc | 0 | 10 | +10 |
| lines | 187 | 113 | -74 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 264 | 137 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 9 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1, N2; 4 unsafe keyword(s) remain

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
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 24 |

---

### global_custom____self_lock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 12 | 4 | -8 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 5 | -20 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 6 | +6 |
| lines | 144 | 86 | -58 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 209 | 101 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 15 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### global_main____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 3 | -22 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 5 | +5 |
| lines | 117 | 74 | -43 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 177 | 90 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 7 unsafe keyword(s) remain

---

### global_nested____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 7 | -1 |
| pthread | 48 | 0 | -48 |
| raw_ptr | 28 | 3 | -25 |
| static_mut | 4 | 4 | +0 |
| libc | 0 | 6 | +6 |
| lines | 175 | 93 | -82 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 263 | 113 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 22 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1, N2, N1_MUTEX, N2_MUTEX; 7 unsafe keyword(s) remain

---

### global_read____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 5 | -3 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 28 | 4 | -24 |
| static_mut | 4 | 2 | -2 |
| libc | 0 | 6 | +6 |
| lines | 141 | 64 | -77 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 209 | 81 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 15 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, n2; 5 unsafe keyword(s) remain

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
| lines | 124 | 92 | -32 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 183 | 102 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 15 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N; 6 unsafe keyword(s) remain

---

### global_simple____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 2 | -5 |
| pthread | 24 | 0 | -24 |
| raw_ptr | 25 | 1 | -24 |
| static_mut | 4 | 0 | -4 |
| libc | 0 | 3 | +3 |
| lines | 125 | 64 | -61 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 70 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 23 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

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
| lines | 131 | 54 | -77 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 193 | 54 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 17 |

---

### struct_alias____self_lock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 7 | -3 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 32 | 6 | -26 |
| static_mut | 3 | 0 | -3 |
| libc | 0 | 2 | +2 |
| lines | 187 | 100 | -87 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 260 | 115 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 39 |

**Remaining Issues:**

- **LLM**: 7 unsafe keyword(s) remain

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
| lines | 146 | 84 | -62 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 238 | 84 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 27 |

---

### struct_condvar____lost_wakeup

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 6 | -1 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 27 | 3 | -24 |
| static_mut | 1 | 1 | +0 |
| libc | 0 | 5 | +5 |
| lines | 185 | 71 | -114 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 252 | 86 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 16 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 6 unsafe keyword(s) remain

---

### struct_dup____deadlock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 6 | -2 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 29 | 3 | -26 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 7 | +7 |
| lines | 180 | 106 | -74 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 251 | 124 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 46 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S1, S2; 6 unsafe keyword(s) remain

---

### struct_init____partial_critical_section

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 6 | -1 |
| pthread | 29 | 0 | -29 |
| raw_ptr | 34 | 7 | -27 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 7 | +7 |
| lines | 157 | 66 | -91 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 229 | 86 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 30 |

**Remaining Issues:**

- **LLM**: 6 unsafe keyword(s) remain

---

### struct_malloc2____lock_mismatch

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 11 | +3 |
| pthread | 35 | 0 | -35 |
| raw_ptr | 34 | 2 | -32 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 4 | +4 |
| lines | 145 | 79 | -66 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 224 | 97 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 17 |

**Remaining Issues:**

- **LLM**: static mut variables remain: X; 11 unsafe keyword(s) remain

---

### struct_malloc____lost_wakeup

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 3 | -4 |
| pthread | 41 | 0 | -41 |
| raw_ptr | 38 | 5 | -33 |
| static_mut | 0 | 0 | +0 |
| libc | 0 | 0 | +0 |
| lines | 158 | 54 | -104 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 244 | 62 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 21 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### struct_multiple____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 11 | 3 | -8 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 37 | 1 | -36 |
| static_mut | 3 | 0 | -3 |
| libc | 0 | 2 | +2 |
| lines | 198 | 90 | -108 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 281 | 96 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 33 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### struct_nested____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 3 | -4 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 2 | -23 |
| static_mut | 1 | 0 | -1 |
| libc | 0 | 4 | +4 |
| lines | 138 | 68 | -70 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 197 | 77 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 19 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### struct_simple____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 5 | -2 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 27 | 3 | -24 |
| static_mut | 1 | 1 | +0 |
| libc | 0 | 5 | +5 |
| lines | 158 | 86 | -72 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 221 | 100 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 16 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 5 unsafe keyword(s) remain

---

### struct_spin____lock_leak

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 3 | -4 |
| pthread | 47 | 0 | -47 |
| raw_ptr | 33 | 5 | -28 |
| static_mut | 0 | 0 | +0 |
| libc | 0 | 0 | +0 |
| lines | 199 | 61 | -138 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 286 | 69 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 30 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### struct_timedwait____deadlock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 6 | -3 |
| pthread | 48 | 0 | -48 |
| raw_ptr | 32 | 2 | -30 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 6 | +6 |
| lines | 271 | 115 | -156 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 362 | 129 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 18 |

**Remaining Issues:**

- **LLM**: 6 unsafe keyword(s) remain

---

### struct_timedwait____lost_wakeup

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 5 | -4 |
| pthread | 37 | 0 | -37 |
| raw_ptr | 29 | 0 | -29 |
| static_mut | 1 | 1 | +0 |
| libc | 0 | 4 | +4 |
| lines | 238 | 124 | -114 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 314 | 134 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 5 unsafe keyword(s) remain

---

### unused_func____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 6 | -2 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 27 | 3 | -24 |
| static_mut | 3 | 1 | -2 |
| libc | 0 | 5 | +5 |
| lines | 135 | 71 | -64 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 201 | 86 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 6 unsafe keyword(s) remain

---
