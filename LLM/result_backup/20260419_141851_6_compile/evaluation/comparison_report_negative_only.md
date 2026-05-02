# Concurrency Transformation Comparison Report (Negative Samples Only)

Analyzing **Original** and **LLM** for negative examples (expected to fail)

## Summary Overview

| # | Example | Type | Compiles (L) | Round | Pos | Pos Round | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_thread | lines |
|---|---------|------|:----------:|:---:|:--:|:----------:|------|------|------|------|------|------|------|------|
| 1 | [array_const____deadlock](#array_const____deadlock) | NEG | ✅ | 5 | ✅ | 2 | 10→0 | 64→0 | 58→0 | 2→0 | 0→0 | 0→12 | 0→1 | 233→99 |
| 2 | [array_const____lock_mismatch](#array_const____lock_mismatch) | NEG | ❌ | c2rust | ✅ | 2 | 7→3 | 42→0 | 41→5 | 2→0 | 0→0 | 0→5 | 0→2 | 183→62 |
| 3 | [array_main____lock_leak](#array_main____lock_leak) | NEG | ❌ | c2rust | ❌ | c2rust | 7→5 | 27→0 | 34→2 | 2→2 | 0→4 | 0→12 | 0→1 | 207→86 |
| 4 | [array_main____partial_critical_section](#array_main____partial_critical_section) | NEG | ❌ | c2rust | ❌ | c2rust | 7→5 | 32→0 | 37→2 | 2→2 | 0→4 | 0→7 | 0→1 | 212→74 |
| 5 | [array_simple____partial_critical_section](#array_simple____partial_critical_section) | NEG | ✅ | 2 | ❌ | c2rust | 7→6 | 22→0 | 33→2 | 4→4 | 0→4 | 0→7 | 0→1 | 227→70 |
| 6 | [global_assume2____self_lock](#global_assume2____self_lock) | NEG | ✅ | 1 | ✅ | 1 | 8→5 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→1 | 124→71 |
| 7 | [global_assume____lock_leak](#global_assume____lock_leak) | NEG | ✅ | 1 | ✅ | 1 | 8→0 | 21→0 | 24→0 | 2→0 | 0→0 | 0→3 | 0→1 | 115→49 |
| 8 | [global_check____lock_leak](#global_check____lock_leak) | NEG | ✅ | 3 | ❌ | c2rust | 10→8 | 39→0 | 26→3 | 2→2 | 0→5 | 0→10 | 0→2 | 176→124 |
| 9 | [global_check____lock_mismatch](#global_check____lock_mismatch) | NEG | ❌ | c2rust | ❌ | c2rust | 10→7 | 46→0 | 28→2 | 3→0 | 0→4 | 0→10 | 0→2 | 197→129 |
| 10 | [global_condvar____lost_wakeup](#global_condvar____lost_wakeup) | NEG | ✅ | 2 | ✅ | 2 | 7→0 | 38→0 | 28→0 | 4→0 | 0→0 | 0→7 | 0→2 | 187→60 |
| 11 | [global_condvar____partial_critical_section](#global_condvar____partial_critical_section) | NEG | ✅ | 2 | ✅ | 2 | 7→0 | 40→0 | 29→0 | 4→0 | 0→0 | 0→7 | 0→2 | 190→66 |
| 12 | [global_custom____self_lock](#global_custom____self_lock) | NEG | ❌ | c2rust | ❌ | c2rust | 12→0 | 26→0 | 25→0 | 2→0 | 0→0 | 0→3 | 0→1 | 144→70 |
| 13 | [global_main____self_lock](#global_main____self_lock) | NEG | ✅ | 1 | ✅ | 1 | 7→7 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→2 | 117→68 |
| 14 | [global_nested____deadlock](#global_nested____deadlock) | NEG | ✅ | 1 | ❌ | c2rust | 8→2 | 48→0 | 28→3 | 4→0 | 0→6 | 0→5 | 0→2 | 175→73 |
| 15 | [global_read____lock_mismatch](#global_read____lock_mismatch) | NEG | ❌ | c2rust | ✅ | 6 | 8→6 | 28→0 | 28→3 | 4→2 | 0→6 | 0→5 | 0→2 | 141→65 |
| 16 | [global_rwlock____lock_leak](#global_rwlock____lock_leak) | NEG | ❌ | c2rust | ❌ | c2rust | 8→7 | 27→17 | 22→9 | 2→2 | 0→0 | 0→1 | 0→0 | 124→123 |
| 17 | [global_simple____partial_critical_section](#global_simple____partial_critical_section) | NEG | ✅ | 2 | ✅ | 7 | 7→1 | 24→0 | 25→1 | 4→0 | 0→2 | 0→7 | 0→1 | 125→69 |
| 18 | [global_while____lock_leak](#global_while____lock_leak) | NEG | ✅ | 1 | ❌ | c2rust | 8→0 | 27→0 | 25→0 | 2→0 | 0→0 | 0→3 | 0→2 | 131→61 |
| 19 | [struct_alias____self_lock](#struct_alias____self_lock) | NEG | ❌ | c2rust | ❌ | c2rust | 10→7 | 28→0 | 32→5 | 3→0 | 0→4 | 0→5 | 0→2 | 187→95 |
| 20 | [struct_assume____deadlock](#struct_assume____deadlock) | NEG | ❌ | c2rust | ✅ | 1 | 10→7 | 37→0 | 45→13 | 0→0 | 0→9 | 0→4 | 0→2 | 146→84 |
| 21 | [struct_condvar____lost_wakeup](#struct_condvar____lost_wakeup) | NEG | ✅ | 3 | ✅ | 3 | 7→7 | 32→0 | 27→3 | 1→1 | 0→5 | 0→5 | 0→2 | 185→71 |
| 22 | [struct_dup____deadlock](#struct_dup____deadlock) | NEG | ✅ | 8 | ✅ | 3 | 8→10 | 32→0 | 29→3 | 2→2 | 0→2 | 0→5 | 0→2 | 180→112 |
| 23 | [struct_init____partial_critical_section](#struct_init____partial_critical_section) | NEG | ❌ | c2rust | ❌ | c2rust | 7→4 | 29→0 | 34→10 | 2→0 | 0→7 | 0→7 | 0→2 | 157→70 |
| 24 | [struct_malloc2____lock_mismatch](#struct_malloc2____lock_mismatch) | NEG | ❌ | c2rust | ❌ | c2rust | 8→8 | 35→0 | 34→3 | 2→1 | 0→5 | 0→6 | 0→2 | 145→65 |
| 25 | [struct_malloc____lost_wakeup](#struct_malloc____lost_wakeup) | NEG | ❌ | c2rust | ❌ | c2rust | 7→3 | 41→0 | 38→6 | 0→0 | 0→0 | 0→3 | 0→2 | 158→58 |
| 26 | [struct_multiple____deadlock](#struct_multiple____deadlock) | NEG | ✅ | 3 | ❌ | c2rust | 11→0 | 32→0 | 37→0 | 3→0 | 0→0 | 0→5 | 0→3 | 198→73 |
| 27 | [struct_nested____self_lock](#struct_nested____self_lock) | NEG | ✅ | 4 | ✅ | 6 | 7→5 | 26→0 | 25→2 | 1→0 | 0→1 | 0→5 | 0→1 | 138→69 |
| 28 | [struct_simple____partial_critical_section](#struct_simple____partial_critical_section) | NEG | ✅ | 3 | ❌ | c2rust | 7→3 | 28→0 | 27→2 | 1→1 | 0→4 | 0→7 | 0→2 | 158→85 |
| 29 | [struct_spin____lock_leak](#struct_spin____lock_leak) | NEG | ✅ | 1 | ❌ | c2rust | 7→0 | 47→0 | 33→0 | 0→0 | 0→0 | 0→7 | 0→2 | 199→58 |
| 30 | [struct_timedwait____deadlock](#struct_timedwait____deadlock) | NEG | ❌ | c2rust | ❌ | c2rust | 9→0 | 48→0 | 32→0 | 2→0 | 0→0 | 0→5 | 0→2 | 271→104 |
| 31 | [struct_timedwait____lost_wakeup](#struct_timedwait____lost_wakeup) | NEG | ❌ | c2rust | ❌ | c2rust | 9→2 | 37→0 | 29→0 | 1→0 | 0→0 | 0→11 | 0→2 | 238→87 |
| 32 | [unused_func____lock_mismatch](#unused_func____lock_mismatch) | NEG | ❌ | c2rust | ✅ | 1 | 8→6 | 28→0 | 27→2 | 3→0 | 0→3 | 0→3 | 0→1 | 135→66 |
| | **TOTAL** | (NEG) | 17/32 | — | 14/30 | — | 261→248 | 1083→34 | 990→174 | 70→42 | 0→166 | 0→380 | 0→106 | 5503→5032 |

> **Reading the table**: Each metric cell shows **Original → LLM**. **Pos** column shows if the corresponding positive sample (before `____`) compiles with LLM. **Pos Round** shows the last successful round (1-N) for the positive sample, or `c2rust` if none compiled successfully. Negative samples are expected to fail (used for validation).

## All Metrics Summary

This section displays all 15 metrics for each sample in a compact format.

| Example | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_arc | std\_rwlock | std\_condvar | std\_thread | move\_closure | arc\_clone | join\_handle | arc\_mutex\_combo | lines |
|---------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| array_const____deadlock | 10→0 | 64→0 | 58→0 | 2→0 | 0→0 | 0→12 | 0→13 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→7 | 233→99 |
| array_const____lock_mismatch | 7→3 | 42→0 | 41→5 | 2→0 | 0→0 | 0→5 | 0→6 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 183→62 |
| array_main____lock_leak | 7→5 | 27→0 | 34→2 | 2→2 | 0→4 | 0→12 | 0→12 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→1 | 207→86 |
| array_main____partial_critical_section | 7→5 | 32→0 | 37→2 | 2→2 | 0→4 | 0→7 | 0→7 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→1 | 212→74 |
| array_simple____partial_critical_section | 7→6 | 22→0 | 33→2 | 4→4 | 0→4 | 0→7 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 227→70 |
| global_assume2____self_lock | 8→5 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 124→71 |
| global_assume____lock_leak | 8→0 | 21→0 | 24→0 | 2→0 | 0→0 | 0→3 | 0→6 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→0 | 115→49 |
| global_check____lock_leak | 10→8 | 39→0 | 26→3 | 2→2 | 0→5 | 0→10 | 0→10 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→5 | 176→124 |
| global_check____lock_mismatch | 10→7 | 46→0 | 28→2 | 3→0 | 0→4 | 0→10 | 0→8 | 0→0 | 0→0 | 0→2 | 0→2 | 0→1 | 0→2 | 0→6 | 197→129 |
| global_condvar____lost_wakeup | 7→0 | 38→0 | 28→0 | 4→0 | 0→0 | 0→7 | 0→6 | 0→0 | 0→3 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 187→60 |
| global_condvar____partial_critical_section | 7→0 | 40→0 | 29→0 | 4→0 | 0→0 | 0→7 | 0→6 | 0→0 | 0→3 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 190→66 |
| global_custom____self_lock | 12→0 | 26→0 | 25→0 | 2→0 | 0→0 | 0→3 | 0→9 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→0 | 144→70 |
| global_main____self_lock | 7→7 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→1 | 0→0 | 0→0 | 0→2 | 0→0 | 0→0 | 0→2 | 0→0 | 117→68 |
| global_nested____deadlock | 8→2 | 48→0 | 28→3 | 4→0 | 0→6 | 0→5 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 175→73 |
| global_read____lock_mismatch | 8→6 | 28→0 | 28→3 | 4→2 | 0→6 | 0→5 | 0→5 | 0→0 | 0→0 | 0→2 | 0→0 | 0→0 | 0→2 | 0→2 | 141→65 |
| global_rwlock____lock_leak | 8→7 | 27→17 | 22→9 | 2→2 | 0→0 | 0→1 | 0→1 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 124→123 |
| global_simple____partial_critical_section | 7→1 | 24→0 | 25→1 | 4→0 | 0→2 | 0→7 | 0→5 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→0 | 125→69 |
| global_while____lock_leak | 8→0 | 27→0 | 25→0 | 2→0 | 0→0 | 0→3 | 0→5 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→0 | 131→61 |
| struct_alias____self_lock | 10→7 | 28→0 | 32→5 | 3→0 | 0→4 | 0→5 | 0→13 | 0→0 | 0→0 | 0→2 | 0→2 | 0→9 | 0→2 | 0→0 | 187→95 |
| struct_assume____deadlock | 10→7 | 37→0 | 45→13 | 0→0 | 0→9 | 0→4 | 0→9 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 146→84 |
| struct_condvar____lost_wakeup | 7→7 | 32→0 | 27→3 | 1→1 | 0→5 | 0→5 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→1 | 0→2 | 0→0 | 185→71 |
| struct_dup____deadlock | 8→10 | 32→0 | 29→3 | 2→2 | 0→2 | 0→5 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 180→112 |
| struct_init____partial_critical_section | 7→4 | 29→0 | 34→10 | 2→0 | 0→7 | 0→7 | 0→10 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→2 | 157→70 |
| struct_malloc2____lock_mismatch | 8→8 | 35→0 | 34→3 | 2→1 | 0→5 | 0→6 | 0→7 | 0→0 | 0→0 | 0→2 | 0→0 | 0→1 | 0→2 | 0→1 | 145→65 |
| struct_malloc____lost_wakeup | 7→3 | 41→0 | 38→6 | 0→0 | 0→0 | 0→3 | 0→7 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 158→58 |
| struct_multiple____deadlock | 11→0 | 32→0 | 37→0 | 3→0 | 0→0 | 0→5 | 0→12 | 0→0 | 0→0 | 0→3 | 0→3 | 0→3 | 0→3 | 0→0 | 198→73 |
| struct_nested____self_lock | 7→5 | 26→0 | 25→2 | 1→0 | 0→1 | 0→5 | 0→4 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→1 | 138→69 |
| struct_simple____partial_critical_section | 7→3 | 28→0 | 27→2 | 1→1 | 0→4 | 0→7 | 0→3 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→1 | 158→85 |
| struct_spin____lock_leak | 7→0 | 47→0 | 33→0 | 0→0 | 0→0 | 0→7 | 0→6 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 199→58 |
| struct_timedwait____deadlock | 9→0 | 48→0 | 32→0 | 2→0 | 0→0 | 0→5 | 0→7 | 0→0 | 0→2 | 0→2 | 0→2 | 0→3 | 0→2 | 0→0 | 271→104 |
| struct_timedwait____lost_wakeup | 9→2 | 37→0 | 29→0 | 1→0 | 0→0 | 0→11 | 0→9 | 0→0 | 0→2 | 0→2 | 0→2 | 0→3 | 0→2 | 0→5 | 238→87 |
| unused_func____lock_mismatch | 8→6 | 28→0 | 27→2 | 3→0 | 0→3 | 0→3 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 135→66 |
| **TOTAL** | 261→248 | 1083→34 | 990→174 | 70→42 | 0→166 | 0→380 | 0→422 | 0→0 | 0→24 | 0→106 | 0→84 | 0→106 | 0→106 | 0→64 | 5503→5032 |

> **All Metrics** table shows all 15 metrics (including std\_arc, std\_rwlock, std\_condvar, move\_closure, arc\_clone, join\_handle, arc\_mutex\_combo) for each sample. Format: **Original → ConCrat → LLM** (or **Original → LLM** for negative samples).

## Aggregate Statistics

| Metric | Original | LLM | vs Original |
|--------|----------|-----|:------------:|
| unsafe | 261 | 248 | +5.0% |
| pthread | 1083 | 34 | +96.9% |
| raw\_ptr | 990 | 174 | +82.4% |
| static\_mut | 70 | 42 | +40.0% |
| libc | 0 | 166 | — |
| lines | 5503 | 5032 | +8.6% |

| **LLM compile success** | — | 17/32 (53%) |  |

## Metric Categories Summary

Aggregate of metrics grouped by category (Lower is Better vs Higher is Better):

| Category | Original | LLM | vs Original |
|----------|----------|-----|:------------:|
| **Lower is Better** (∑unsafe, pthread, raw_ptr, static_mut, libc, lines) | 7907 | 5696 | +28.0% |
| **Higher is Better** (∑std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 1292 | — |

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
| lines | 233 | 99 | -134 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 367 | 99 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 36 |

---

### array_const____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 3 | -4 |
| pthread | 42 | 0 | -42 |
| raw_ptr | 41 | 5 | -36 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 183 | 62 | -121 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 275 | 70 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 19 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

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
| lines | 207 | 86 | -121 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 277 | 99 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 27 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, num_mutex; 5 unsafe keyword(s) remain

---

### array_main____partial_critical_section

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 5 | -2 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 37 | 2 | -35 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 4 | +4 |
| lines | 212 | 74 | -138 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 290 | 87 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 17 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, num_mutex; 5 unsafe keyword(s) remain

---

### array_simple____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 6 | -1 |
| pthread | 22 | 0 | -22 |
| raw_ptr | 33 | 2 | -31 |
| static_mut | 4 | 4 | +0 |
| libc | 0 | 4 | +4 |
| lines | 227 | 70 | -157 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 293 | 86 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 10 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, n2, n3, num_mutex; 6 unsafe keyword(s) remain

---

### global_assume2____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 5 | -3 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 3 | -22 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 4 | +4 |
| lines | 124 | 71 | -53 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 84 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 5 unsafe keyword(s) remain

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

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 8 | -2 |
| pthread | 39 | 0 | -39 |
| raw_ptr | 26 | 3 | -23 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 5 | +5 |
| lines | 176 | 124 | -52 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 253 | 142 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 34 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N, M; 8 unsafe keyword(s) remain

---

### global_check____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 7 | -3 |
| pthread | 46 | 0 | -46 |
| raw_ptr | 28 | 2 | -26 |
| static_mut | 3 | 0 | -3 |
| libc | 0 | 4 | +4 |
| lines | 197 | 129 | -68 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 284 | 142 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 31 |

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
| lines | 187 | 60 | -127 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 264 | 60 |
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
| lines | 190 | 66 | -124 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 270 | 66 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 24 |

---

### global_custom____self_lock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 12 | 0 | -12 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 0 | -25 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 144 | 70 | -74 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 209 | 70 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 16 |

---

### global_main____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 3 | -22 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 4 | +4 |
| lines | 117 | 68 | -49 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 177 | 83 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 9 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 7 unsafe keyword(s) remain

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
| lines | 175 | 73 | -102 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 263 | 84 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

---

### global_read____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 6 | -2 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 28 | 3 | -25 |
| static_mut | 4 | 2 | -2 |
| libc | 0 | 6 | +6 |
| lines | 141 | 65 | -76 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 209 | 82 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 16 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1, N2; 6 unsafe keyword(s) remain

---

### global_rwlock____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 7 | -1 |
| pthread | 27 | 17 | -10 |
| raw_ptr | 22 | 9 | -13 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 124 | 123 | -1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 183 | 158 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 2 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n, lock; 7 unsafe keyword(s) remain

---

### global_simple____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 1 | -6 |
| pthread | 24 | 0 | -24 |
| raw_ptr | 25 | 1 | -24 |
| static_mut | 4 | 0 | -4 |
| libc | 0 | 2 | +2 |
| lines | 125 | 69 | -56 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 73 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 16 |

**Remaining Issues:**

- **LLM**: 1 unsafe keyword(s) remain

---

### global_while____lock_leak

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 0 | -8 |
| pthread | 27 | 0 | -27 |
| raw_ptr | 25 | 0 | -25 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 131 | 61 | -70 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 193 | 61 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 17 |

---

### struct_alias____self_lock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 7 | -3 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 32 | 5 | -27 |
| static_mut | 3 | 0 | -3 |
| libc | 0 | 4 | +4 |
| lines | 187 | 95 | -92 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 260 | 111 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 33 |

**Remaining Issues:**

- **LLM**: 7 unsafe keyword(s) remain

---

### struct_assume____deadlock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 7 | -3 |
| pthread | 37 | 0 | -37 |
| raw_ptr | 45 | 13 | -32 |
| static_mut | 0 | 0 | +0 |
| libc | 0 | 9 | +9 |
| lines | 146 | 84 | -62 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 238 | 113 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 21 |

**Remaining Issues:**

- **LLM**: 7 unsafe keyword(s) remain

---

### struct_condvar____lost_wakeup

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 27 | 3 | -24 |
| static_mut | 1 | 1 | +0 |
| libc | 0 | 5 | +5 |
| lines | 185 | 71 | -114 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 252 | 87 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 19 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 7 unsafe keyword(s) remain

---

### struct_dup____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 10 | +2 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 29 | 3 | -26 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 2 | +2 |
| lines | 180 | 112 | -68 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 251 | 129 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S1, S2; 10 unsafe keyword(s) remain

---

### struct_init____partial_critical_section

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 29 | 0 | -29 |
| raw_ptr | 34 | 10 | -24 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 7 | +7 |
| lines | 157 | 70 | -87 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 229 | 91 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 27 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

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
| lines | 145 | 65 | -80 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 224 | 82 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 19 |

**Remaining Issues:**

- **LLM**: static mut variables remain: X; 8 unsafe keyword(s) remain

---

### struct_malloc____lost_wakeup

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 3 | -4 |
| pthread | 41 | 0 | -41 |
| raw_ptr | 38 | 6 | -32 |
| static_mut | 0 | 0 | +0 |
| libc | 0 | 0 | +0 |
| lines | 158 | 58 | -100 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 244 | 67 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

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
| unsafe | 7 | 5 | -2 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 2 | -23 |
| static_mut | 1 | 0 | -1 |
| libc | 0 | 1 | +1 |
| lines | 138 | 69 | -69 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 197 | 77 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 14 |

**Remaining Issues:**

- **LLM**: 5 unsafe keyword(s) remain

---

### struct_simple____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 3 | -4 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 27 | 2 | -25 |
| static_mut | 1 | 1 | +0 |
| libc | 0 | 4 | +4 |
| lines | 158 | 85 | -73 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 221 | 95 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 3 unsafe keyword(s) remain

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
| lines | 199 | 58 | -141 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 286 | 58 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 21 |

---

### struct_timedwait____deadlock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 0 | -9 |
| pthread | 48 | 0 | -48 |
| raw_ptr | 32 | 0 | -32 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 271 | 104 | -167 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 362 | 104 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 23 |

---

### struct_timedwait____lost_wakeup

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 2 | -7 |
| pthread | 37 | 0 | -37 |
| raw_ptr | 29 | 0 | -29 |
| static_mut | 1 | 0 | -1 |
| libc | 0 | 0 | +0 |
| lines | 238 | 87 | -151 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 314 | 89 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 36 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

---

### unused_func____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 6 | -2 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 27 | 2 | -25 |
| static_mut | 3 | 0 | -3 |
| libc | 0 | 3 | +3 |
| lines | 135 | 66 | -69 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 201 | 77 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 6 |

**Remaining Issues:**

- **LLM**: 6 unsafe keyword(s) remain

---
