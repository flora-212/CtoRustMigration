# Concurrency Transformation Comparison Report (Negative Samples Only)

Analyzing **Original** and **LLM** for negative examples (expected to fail)

## Summary Overview

| # | Example | Type | Compiles (L) | Round | Pos | Pos Round | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_thread | lines |
|---|---------|------|:----------:|:---:|:--:|:----------:|------|------|------|------|------|------|------|------|
| 1 | [array_const____deadlock](#array_const____deadlock) | NEG | ❌ | c2rust | ❌ | c2rust | 10→0 | 64→0 | 58→0 | 2→0 | 0→0 | 0→9 | 0→1 | 233→81 |
| 2 | [array_const____lock_mismatch](#array_const____lock_mismatch) | NEG | ❌ | c2rust | ❌ | c2rust | 7→0 | 42→0 | 41→0 | 2→0 | 0→0 | 0→5 | 0→1 | 183→60 |
| 3 | [array_main____lock_leak](#array_main____lock_leak) | NEG | ✅ | 1 | ✅ | 14 | 7→4 | 27→0 | 34→2 | 2→2 | 0→4 | 0→7 | 0→1 | 207→66 |
| 4 | [array_main____partial_critical_section](#array_main____partial_critical_section) | NEG | ❌ | c2rust | ✅ | 14 | 7→7 | 32→0 | 37→2 | 2→1 | 0→3 | 0→7 | 0→2 | 212→84 |
| 5 | [array_simple____partial_critical_section](#array_simple____partial_critical_section) | NEG | ✅ | 1 | ❌ | c2rust | 7→5 | 22→0 | 33→2 | 4→4 | 0→3 | 0→7 | 0→1 | 227→66 |
| 6 | [global_assume2____self_lock](#global_assume2____self_lock) | NEG | ❌ | c2rust | ❌ | c2rust | 8→6 | 26→0 | 25→2 | 2→1 | 0→2 | 0→4 | 0→2 | 124→66 |
| 7 | [global_assume____lock_leak](#global_assume____lock_leak) | NEG | ✅ | 1 | ✅ | 4 | 8→0 | 21→0 | 24→0 | 2→0 | 0→0 | 0→3 | 0→1 | 115→49 |
| 8 | [global_check____lock_leak](#global_check____lock_leak) | NEG | ❌ | c2rust | ❌ | c2rust | 10→9 | 39→23 | 26→8 | 2→2 | 0→0 | 0→1 | 0→0 | 176→172 |
| 9 | [global_check____lock_mismatch](#global_check____lock_mismatch) | NEG | ❌ | c2rust | ❌ | c2rust | 10→7 | 46→0 | 28→3 | 3→0 | 0→5 | 0→5 | 0→2 | 197→103 |
| 10 | [global_condvar____lost_wakeup](#global_condvar____lost_wakeup) | NEG | ✅ | 2 | ✅ | 2 | 7→0 | 38→0 | 28→0 | 4→0 | 0→0 | 0→7 | 0→2 | 187→56 |
| 11 | [global_condvar____partial_critical_section](#global_condvar____partial_critical_section) | NEG | ✅ | 7 | ✅ | 2 | 7→1 | 40→0 | 29→1 | 4→0 | 0→2 | 0→6 | 0→2 | 190→65 |
| 12 | [global_custom____self_lock](#global_custom____self_lock) | NEG | ❌ | c2rust | ✅ | 3 | 12→2 | 26→0 | 25→3 | 2→0 | 0→3 | 0→3 | 0→2 | 144→97 |
| 13 | [global_main____self_lock](#global_main____self_lock) | NEG | ✅ | 2 | ✅ | 1 | 7→4 | 26→0 | 25→3 | 2→1 | 0→5 | 0→4 | 0→2 | 117→61 |
| 14 | [global_nested____deadlock](#global_nested____deadlock) | NEG | ✅ | 1 | ✅ | 1 | 8→0 | 48→0 | 28→0 | 4→0 | 0→0 | 0→5 | 0→2 | 175→68 |
| 15 | [global_read____lock_mismatch](#global_read____lock_mismatch) | NEG | ❌ | c2rust | ✅ | 3 | 8→5 | 28→0 | 28→4 | 4→4 | 0→7 | 0→5 | 0→2 | 141→56 |
| 16 | [global_rwlock____lock_leak](#global_rwlock____lock_leak) | NEG | ❌ | c2rust | ❌ | c2rust | 8→2 | 27→0 | 22→2 | 2→0 | 0→2 | 0→3 | 0→2 | 124→70 |
| 17 | [global_simple____partial_critical_section](#global_simple____partial_critical_section) | NEG | ❌ | c2rust | ❌ | c2rust | 7→4 | 24→0 | 25→4 | 4→0 | 0→5 | 0→4 | 0→1 | 125→61 |
| 18 | [global_while____lock_leak](#global_while____lock_leak) | NEG | ❌ | c2rust | ✅ | 2 | 8→8 | 27→13 | 25→9 | 2→2 | 0→0 | 0→1 | 0→0 | 131→135 |
| 19 | [struct_alias____self_lock](#struct_alias____self_lock) | NEG | ✅ | 7 | ❌ | c2rust | 10→2 | 28→0 | 32→2 | 3→0 | 0→0 | 0→5 | 0→2 | 187→86 |
| 20 | [struct_assume____deadlock](#struct_assume____deadlock) | NEG | ❌ | c2rust | ✅ | 2 | 10→0 | 37→0 | 45→0 | 0→0 | 0→0 | 0→4 | 0→2 | 146→62 |
| 21 | [struct_condvar____lost_wakeup](#struct_condvar____lost_wakeup) | NEG | ✅ | 11 | ✅ | 3 | 7→4 | 32→0 | 27→2 | 1→0 | 0→4 | 0→3 | 0→1 | 185→59 |
| 22 | [struct_dup____deadlock](#struct_dup____deadlock) | NEG | ❌ | c2rust | ✅ | 3 | 8→7 | 32→0 | 29→3 | 2→2 | 0→7 | 0→5 | 0→2 | 180→102 |
| 23 | [struct_init____partial_critical_section](#struct_init____partial_critical_section) | NEG | ❌ | c2rust | ❌ | c2rust | 7→7 | 29→0 | 34→7 | 2→2 | 0→7 | 0→10 | 0→2 | 157→78 |
| 24 | [struct_malloc2____lock_mismatch](#struct_malloc2____lock_mismatch) | NEG | ❌ | c2rust | ✅ | 1 | 8→8 | 35→0 | 34→3 | 2→1 | 0→5 | 0→6 | 0→2 | 145→63 |
| 25 | [struct_malloc____lost_wakeup](#struct_malloc____lost_wakeup) | NEG | ❌ | c2rust | ❌ | c2rust | 7→3 | 41→0 | 38→5 | 0→0 | 0→0 | 0→3 | 0→2 | 158→54 |
| 26 | [struct_multiple____deadlock](#struct_multiple____deadlock) | NEG | ✅ | 7 | ✅ | 3 | 11→3 | 32→0 | 37→4 | 3→0 | 0→8 | 0→13 | 0→3 | 198→90 |
| 27 | [struct_nested____self_lock](#struct_nested____self_lock) | NEG | ✅ | 8 | ❌ | c2rust | 7→4 | 26→0 | 25→2 | 1→0 | 0→4 | 0→3 | 0→2 | 138→62 |
| 28 | [struct_simple____partial_critical_section](#struct_simple____partial_critical_section) | NEG | ✅ | 8 | ❌ | c2rust | 7→6 | 28→0 | 27→3 | 1→1 | 0→5 | 0→9 | 0→2 | 158→88 |
| 29 | [struct_spin____lock_leak](#struct_spin____lock_leak) | NEG | ✅ | 1 | ❌ | c2rust | 7→0 | 47→0 | 33→0 | 0→0 | 0→0 | 0→7 | 0→2 | 199→55 |
| 30 | [struct_timedwait____deadlock](#struct_timedwait____deadlock) | NEG | ❌ | c2rust | ❌ | c2rust | 9→4 | 48→0 | 32→0 | 2→0 | 0→4 | 0→7 | 0→2 | 271→120 |
| 31 | [struct_timedwait____lost_wakeup](#struct_timedwait____lost_wakeup) | NEG | ❌ | c2rust | ❌ | c2rust | 9→4 | 37→0 | 29→0 | 1→0 | 0→4 | 0→3 | 0→2 | 238→109 |
| 32 | [unused_func____lock_mismatch](#unused_func____lock_mismatch) | NEG | ❌ | c2rust | ✅ | 2 | 8→5 | 28→0 | 27→3 | 3→1 | 0→5 | 0→7 | 0→2 | 135→58 |
| | **TOTAL** | (NEG) | 13/32 | — | 16/30 | — | 261→242 | 1083→72 | 990→158 | 70→48 | 0→188 | 0→342 | 0→108 | 5503→5004 |

> **Reading the table**: Each metric cell shows **Original → LLM**. **Pos** column shows if the corresponding positive sample (before `____`) compiles with LLM. **Pos Round** shows the last successful round (1-N) for the positive sample, or `c2rust` if none compiled successfully. Negative samples are expected to fail (used for validation).

## All Metrics Summary

This section displays all 15 metrics for each sample in a compact format.

| Example | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_arc | std\_rwlock | std\_condvar | std\_thread | move\_closure | arc\_clone | join\_handle | arc\_mutex\_combo | lines |
|---------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| array_const____deadlock | 10→0 | 64→0 | 58→0 | 2→0 | 0→0 | 0→9 | 0→4 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→0 | 233→81 |
| array_const____lock_mismatch | 7→0 | 42→0 | 41→0 | 2→0 | 0→0 | 0→5 | 0→5 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→0 | 183→60 |
| array_main____lock_leak | 7→4 | 27→0 | 34→2 | 2→2 | 0→4 | 0→7 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 207→66 |
| array_main____partial_critical_section | 7→7 | 32→0 | 37→2 | 2→1 | 0→3 | 0→7 | 0→7 | 0→0 | 0→0 | 0→2 | 0→0 | 0→0 | 0→1 | 0→1 | 212→84 |
| array_simple____partial_critical_section | 7→5 | 22→0 | 33→2 | 4→4 | 0→3 | 0→7 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 227→66 |
| global_assume2____self_lock | 8→6 | 26→0 | 25→2 | 2→1 | 0→2 | 0→4 | 0→4 | 0→0 | 0→0 | 0→2 | 0→2 | 0→4 | 0→2 | 0→1 | 124→66 |
| global_assume____lock_leak | 8→0 | 21→0 | 24→0 | 2→0 | 0→0 | 0→3 | 0→6 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→0 | 115→49 |
| global_check____lock_leak | 10→9 | 39→23 | 26→8 | 2→2 | 0→0 | 0→1 | 0→1 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 176→172 |
| global_check____lock_mismatch | 10→7 | 46→0 | 28→3 | 3→0 | 0→5 | 0→5 | 0→5 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→1 | 197→103 |
| global_condvar____lost_wakeup | 7→0 | 38→0 | 28→0 | 4→0 | 0→0 | 0→7 | 0→5 | 0→0 | 0→3 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 187→56 |
| global_condvar____partial_critical_section | 7→1 | 40→0 | 29→1 | 4→0 | 0→2 | 0→6 | 0→6 | 0→0 | 0→3 | 0→2 | 0→2 | 0→2 | 0→2 | 0→2 | 190→65 |
| global_custom____self_lock | 12→2 | 26→0 | 25→3 | 2→0 | 0→3 | 0→3 | 0→8 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→0 | 144→97 |
| global_main____self_lock | 7→4 | 26→0 | 25→3 | 2→1 | 0→5 | 0→4 | 0→1 | 0→0 | 0→0 | 0→2 | 0→0 | 0→0 | 0→2 | 0→0 | 117→61 |
| global_nested____deadlock | 8→0 | 48→0 | 28→0 | 4→0 | 0→0 | 0→5 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 175→68 |
| global_read____lock_mismatch | 8→5 | 28→0 | 28→4 | 4→4 | 0→7 | 0→5 | 0→5 | 0→0 | 0→0 | 0→2 | 0→0 | 0→0 | 0→2 | 0→2 | 141→56 |
| global_rwlock____lock_leak | 8→2 | 27→0 | 22→2 | 2→0 | 0→2 | 0→3 | 0→4 | 0→5 | 0→0 | 0→2 | 0→2 | 0→1 | 0→2 | 0→1 | 124→70 |
| global_simple____partial_critical_section | 7→4 | 24→0 | 25→4 | 4→0 | 0→5 | 0→4 | 0→6 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→1 | 125→61 |
| global_while____lock_leak | 8→8 | 27→13 | 25→9 | 2→2 | 0→0 | 0→1 | 0→1 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 131→135 |
| struct_alias____self_lock | 10→2 | 28→0 | 32→2 | 3→0 | 0→0 | 0→5 | 0→13 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→0 | 187→86 |
| struct_assume____deadlock | 10→0 | 37→0 | 45→0 | 0→0 | 0→0 | 0→4 | 0→10 | 0→0 | 0→0 | 0→2 | 0→2 | 0→4 | 0→2 | 0→0 | 146→62 |
| struct_condvar____lost_wakeup | 7→4 | 32→0 | 27→2 | 1→0 | 0→4 | 0→3 | 0→3 | 0→0 | 0→3 | 0→1 | 0→1 | 0→2 | 0→1 | 0→0 | 185→59 |
| struct_dup____deadlock | 8→7 | 32→0 | 29→3 | 2→2 | 0→7 | 0→5 | 0→5 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 180→102 |
| struct_init____partial_critical_section | 7→7 | 29→0 | 34→7 | 2→2 | 0→7 | 0→10 | 0→8 | 0→0 | 0→0 | 0→2 | 0→2 | 0→4 | 0→2 | 0→2 | 157→78 |
| struct_malloc2____lock_mismatch | 8→8 | 35→0 | 34→3 | 2→1 | 0→5 | 0→6 | 0→4 | 0→0 | 0→0 | 0→2 | 0→0 | 0→0 | 0→2 | 0→0 | 145→63 |
| struct_malloc____lost_wakeup | 7→3 | 41→0 | 38→5 | 0→0 | 0→0 | 0→3 | 0→8 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 158→54 |
| struct_multiple____deadlock | 11→3 | 32→0 | 37→4 | 3→0 | 0→8 | 0→13 | 0→9 | 0→0 | 0→0 | 0→3 | 0→0 | 0→0 | 0→3 | 0→5 | 198→90 |
| struct_nested____self_lock | 7→4 | 26→0 | 25→2 | 1→0 | 0→4 | 0→3 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→1 | 138→62 |
| struct_simple____partial_critical_section | 7→6 | 28→0 | 27→3 | 1→1 | 0→5 | 0→9 | 0→5 | 0→0 | 0→0 | 0→2 | 0→2 | 0→1 | 0→2 | 0→3 | 158→88 |
| struct_spin____lock_leak | 7→0 | 47→0 | 33→0 | 0→0 | 0→0 | 0→7 | 0→12 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→3 | 199→55 |
| struct_timedwait____deadlock | 9→4 | 48→0 | 32→0 | 2→0 | 0→4 | 0→7 | 0→3 | 0→0 | 0→2 | 0→2 | 0→0 | 0→3 | 0→2 | 0→1 | 271→120 |
| struct_timedwait____lost_wakeup | 9→4 | 37→0 | 29→0 | 1→0 | 0→4 | 0→3 | 0→7 | 0→0 | 0→2 | 0→2 | 0→2 | 0→5 | 0→2 | 0→0 | 238→109 |
| unused_func____lock_mismatch | 8→5 | 28→0 | 27→3 | 3→1 | 0→5 | 0→7 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→4 | 135→58 |
| **TOTAL** | 261→242 | 1083→72 | 990→158 | 70→48 | 0→188 | 0→342 | 0→356 | 0→10 | 0→30 | 0→108 | 0→78 | 0→106 | 0→106 | 0→56 | 5503→5004 |

> **All Metrics** table shows all 15 metrics (including std\_arc, std\_rwlock, std\_condvar, move\_closure, arc\_clone, join\_handle, arc\_mutex\_combo) for each sample. Format: **Original → ConCrat → LLM** (or **Original → LLM** for negative samples).

## Aggregate Statistics

| Metric | Original | LLM | vs Original |
|--------|----------|-----|:------------:|
| unsafe | 261 | 242 | +7.3% |
| pthread | 1083 | 72 | +93.4% |
| raw\_ptr | 990 | 158 | +84.0% |
| static\_mut | 70 | 48 | +31.4% |
| libc | 0 | 188 | — |
| lines | 5503 | 5004 | +9.1% |

| **LLM compile success** | — | 13/32 (41%) |  |

## Metric Categories Summary

Aggregate of metrics grouped by category (Lower is Better vs Higher is Better):

| Category | Original | LLM | vs Original |
|----------|----------|-----|:------------:|
| **Lower is Better** (∑unsafe, pthread, raw_ptr, static_mut, libc, lines) | 7907 | 5712 | +27.8% |
| **Higher is Better** (∑std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 1192 | — |

## Per-Example Details

### array_const____deadlock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 0 | -10 |
| pthread | 64 | 0 | -64 |
| raw_ptr | 58 | 0 | -58 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 233 | 81 | -152 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 367 | 81 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 17 |

---

### array_const____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 0 | -7 |
| pthread | 42 | 0 | -42 |
| raw_ptr | 41 | 0 | -41 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 183 | 60 | -123 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 275 | 60 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 14 |

---

### array_main____lock_leak

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 27 | 0 | -27 |
| raw_ptr | 34 | 2 | -32 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 4 | +4 |
| lines | 207 | 66 | -141 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 277 | 78 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 10 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, num_mutex; 4 unsafe keyword(s) remain

---

### array_main____partial_critical_section

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 37 | 2 | -35 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 3 | +3 |
| lines | 212 | 84 | -128 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 290 | 97 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 18 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1; 7 unsafe keyword(s) remain

---

### array_simple____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 5 | -2 |
| pthread | 22 | 0 | -22 |
| raw_ptr | 33 | 2 | -31 |
| static_mut | 4 | 4 | +0 |
| libc | 0 | 3 | +3 |
| lines | 227 | 66 | -161 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 293 | 80 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 10 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, n2, n3, num_mutex; 5 unsafe keyword(s) remain

---

### global_assume2____self_lock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 6 | -2 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 2 | -23 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 2 | +2 |
| lines | 124 | 66 | -58 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 77 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 19 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 6 unsafe keyword(s) remain

---

### global_assume____lock_leak

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 0 | -8 |
| pthread | 21 | 0 | -21 |
| raw_ptr | 24 | 0 | -24 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 115 | 49 | -66 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 170 | 49 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 13 |

---

### global_check____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 9 | -1 |
| pthread | 39 | 23 | -16 |
| raw_ptr | 26 | 8 | -18 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 176 | 172 | -4 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 253 | 214 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 2 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (12 calls); static mut variables remain: n, m; 9 unsafe keyword(s) remain

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
| lines | 197 | 103 | -94 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 284 | 118 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 19 |

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
| lines | 187 | 56 | -131 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 264 | 56 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 23 |

---

### global_condvar____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 1 | -6 |
| pthread | 40 | 0 | -40 |
| raw_ptr | 29 | 1 | -28 |
| static_mut | 4 | 0 | -4 |
| libc | 0 | 2 | +2 |
| lines | 190 | 65 | -125 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 270 | 69 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 25 |

**Remaining Issues:**

- **LLM**: 1 unsafe keyword(s) remain

---

### global_custom____self_lock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 12 | 2 | -10 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 3 | -22 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 3 | +3 |
| lines | 144 | 97 | -47 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 209 | 105 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

---

### global_main____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 3 | -22 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 5 | +5 |
| lines | 117 | 61 | -56 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 177 | 74 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 9 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 4 unsafe keyword(s) remain

---

### global_nested____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 0 | -8 |
| pthread | 48 | 0 | -48 |
| raw_ptr | 28 | 0 | -28 |
| static_mut | 4 | 0 | -4 |
| libc | 0 | 0 | +0 |
| lines | 175 | 68 | -107 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 263 | 68 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

---

### global_read____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 5 | -3 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 28 | 4 | -24 |
| static_mut | 4 | 4 | +0 |
| libc | 0 | 7 | +7 |
| lines | 141 | 56 | -85 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 209 | 76 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 16 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, n2, num_mutex1, num_mutex2; 5 unsafe keyword(s) remain

---

### global_rwlock____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 2 | -6 |
| pthread | 27 | 0 | -27 |
| raw_ptr | 22 | 2 | -20 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 2 | +2 |
| lines | 124 | 70 | -54 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 183 | 76 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

---

### global_simple____partial_critical_section

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 24 | 0 | -24 |
| raw_ptr | 25 | 4 | -21 |
| static_mut | 4 | 0 | -4 |
| libc | 0 | 5 | +5 |
| lines | 125 | 61 | -64 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 74 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 15 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### global_while____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 8 | +0 |
| pthread | 27 | 13 | -14 |
| raw_ptr | 25 | 9 | -16 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 131 | 135 | +4 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 193 | 167 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 2 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (5 calls); static mut variables remain: n1, num_mutex; 8 unsafe keyword(s) remain

---

### struct_alias____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 2 | -8 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 32 | 2 | -30 |
| static_mut | 3 | 0 | -3 |
| libc | 0 | 0 | +0 |
| lines | 187 | 86 | -101 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 260 | 90 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 27 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

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
| lines | 146 | 62 | -84 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 238 | 62 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 24 |

---

### struct_condvar____lost_wakeup

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 27 | 2 | -25 |
| static_mut | 1 | 0 | -1 |
| libc | 0 | 4 | +4 |
| lines | 185 | 59 | -126 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 252 | 69 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 14 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_dup____deadlock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 7 | -1 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 29 | 3 | -26 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 7 | +7 |
| lines | 180 | 102 | -78 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 251 | 121 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 18 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S1, S2; 7 unsafe keyword(s) remain

---

### struct_init____partial_critical_section

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 29 | 0 | -29 |
| raw_ptr | 34 | 7 | -27 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 7 | +7 |
| lines | 157 | 78 | -79 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 229 | 101 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 30 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S1, S2; 7 unsafe keyword(s) remain

---

### struct_malloc2____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 8 | +0 |
| pthread | 35 | 0 | -35 |
| raw_ptr | 34 | 3 | -31 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 5 | +5 |
| lines | 145 | 63 | -82 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 224 | 80 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 14 |

**Remaining Issues:**

- **LLM**: static mut variables remain: X; 8 unsafe keyword(s) remain

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
| raw_ptr | 37 | 4 | -33 |
| static_mut | 3 | 0 | -3 |
| libc | 0 | 8 | +8 |
| lines | 198 | 90 | -108 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 281 | 105 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 33 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### struct_nested____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 2 | -23 |
| static_mut | 1 | 0 | -1 |
| libc | 0 | 4 | +4 |
| lines | 138 | 62 | -76 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 197 | 72 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_simple____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 6 | -1 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 27 | 3 | -24 |
| static_mut | 1 | 1 | +0 |
| libc | 0 | 5 | +5 |
| lines | 158 | 88 | -70 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 221 | 103 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 24 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 6 unsafe keyword(s) remain

---

### struct_spin____lock_leak

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 0 | -7 |
| pthread | 47 | 0 | -47 |
| raw_ptr | 33 | 0 | -33 |
| static_mut | 0 | 0 | +0 |
| libc | 0 | 0 | +0 |
| lines | 199 | 55 | -144 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 286 | 55 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 30 |

---

### struct_timedwait____deadlock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 4 | -5 |
| pthread | 48 | 0 | -48 |
| raw_ptr | 32 | 0 | -32 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 4 | +4 |
| lines | 271 | 120 | -151 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 362 | 128 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_timedwait____lost_wakeup

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 4 | -5 |
| pthread | 37 | 0 | -37 |
| raw_ptr | 29 | 0 | -29 |
| static_mut | 1 | 0 | -1 |
| libc | 0 | 4 | +4 |
| lines | 238 | 109 | -129 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 314 | 117 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 23 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### unused_func____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 5 | -3 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 27 | 3 | -24 |
| static_mut | 3 | 1 | -2 |
| libc | 0 | 5 | +5 |
| lines | 135 | 58 | -77 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 201 | 72 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 26 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 5 unsafe keyword(s) remain

---
