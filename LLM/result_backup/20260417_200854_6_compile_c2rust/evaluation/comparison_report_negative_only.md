# Concurrency Transformation Comparison Report (Negative Samples Only)

Analyzing **Original** and **LLM** for negative examples (expected to fail)

## Summary Overview

| # | Example | Type | Compiles (L) | Round | Pos | Pos Round | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_thread | lines |
|---|---------|------|:----------:|:---:|:--:|:----------:|------|------|------|------|------|------|------|------|
| 1 | [array_const____deadlock](#array_const____deadlock) | NEG | ✅ | 3 | ✅ | 2 | 10→0 | 64→0 | 58→0 | 2→0 | 0→0 | 0→9 | 0→1 | 233→70 |
| 2 | [array_const____lock_mismatch](#array_const____lock_mismatch) | NEG | ✅ | c2rust | ✅ | 2 | 7→7 | 42→42 | 41→41 | 2→2 | 0→0 | 0→0 | 0→0 | 183→183 |
| 3 | [array_main____lock_leak](#array_main____lock_leak) | NEG | ✅ | c2rust | ✅ | 2 | 7→7 | 27→32 | 34→37 | 2→2 | 0→0 | 0→0 | 0→0 | 207→211 |
| 4 | [array_main____partial_critical_section](#array_main____partial_critical_section) | NEG | ✅ | 3 | ✅ | 2 | 7→6 | 32→0 | 37→3 | 2→2 | 0→4 | 0→8 | 0→1 | 212→78 |
| 5 | [array_simple____partial_critical_section](#array_simple____partial_critical_section) | NEG | ✅ | 1 | ✅ | c2rust | 7→6 | 22→0 | 33→2 | 4→4 | 0→1 | 0→7 | 0→1 | 227→73 |
| 6 | [global_assume2____self_lock](#global_assume2____self_lock) | NEG | ✅ | 1 | ✅ | c2rust | 8→6 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→1 | 124→71 |
| 7 | [global_assume____lock_leak](#global_assume____lock_leak) | NEG | ✅ | 1 | ✅ | 2 | 8→0 | 21→0 | 24→0 | 2→0 | 0→0 | 0→3 | 0→1 | 115→49 |
| 8 | [global_check____lock_leak](#global_check____lock_leak) | NEG | ✅ | c2rust | ✅ | c2rust | 10→10 | 39→44 | 26→26 | 2→2 | 0→0 | 0→0 | 0→0 | 176→181 |
| 9 | [global_check____lock_mismatch](#global_check____lock_mismatch) | NEG | ✅ | c2rust | ✅ | c2rust | 10→10 | 46→44 | 28→26 | 3→2 | 0→0 | 0→0 | 0→0 | 197→181 |
| 10 | [global_condvar____lost_wakeup](#global_condvar____lost_wakeup) | NEG | ✅ | c2rust | ✅ | 2 | 7→7 | 38→40 | 28→29 | 4→4 | 0→0 | 0→0 | 0→0 | 187→188 |
| 11 | [global_condvar____partial_critical_section](#global_condvar____partial_critical_section) | NEG | ✅ | 2 | ✅ | 2 | 7→0 | 40→0 | 29→0 | 4→0 | 0→0 | 0→7 | 0→2 | 190→71 |
| 12 | [global_custom____self_lock](#global_custom____self_lock) | NEG | ✅ | 1 | ✅ | 12 | 12→0 | 26→0 | 25→0 | 2→0 | 0→0 | 0→3 | 0→2 | 144→76 |
| 13 | [global_main____self_lock](#global_main____self_lock) | NEG | ✅ | 1 | ✅ | 1 | 7→7 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→2 | 117→68 |
| 14 | [global_nested____deadlock](#global_nested____deadlock) | NEG | ✅ | 1 | ✅ | 1 | 8→2 | 48→0 | 28→3 | 4→0 | 0→6 | 0→5 | 0→2 | 175→71 |
| 15 | [global_read____lock_mismatch](#global_read____lock_mismatch) | NEG | ✅ | 3 | ✅ | 6 | 8→8 | 28→0 | 28→6 | 4→4 | 0→6 | 0→7 | 0→2 | 141→80 |
| 16 | [global_rwlock____lock_leak](#global_rwlock____lock_leak) | NEG | ✅ | 9 | ✅ | c2rust | 8→1 | 27→0 | 22→2 | 2→0 | 0→1 | 0→7 | 0→1 | 124→85 |
| 17 | [global_simple____partial_critical_section](#global_simple____partial_critical_section) | NEG | ✅ | 3 | ✅ | 18 | 7→7 | 24→0 | 25→4 | 4→4 | 0→2 | 0→4 | 0→2 | 125→65 |
| 18 | [global_while____lock_leak](#global_while____lock_leak) | NEG | ✅ | 2 | ✅ | 3 | 8→0 | 27→0 | 25→0 | 2→0 | 0→0 | 0→5 | 0→2 | 131→62 |
| 19 | [struct_alias____self_lock](#struct_alias____self_lock) | NEG | ✅ | c2rust | ✅ | c2rust | 10→10 | 28→26 | 32→32 | 3→3 | 0→0 | 0→0 | 0→0 | 187→185 |
| 20 | [struct_assume____deadlock](#struct_assume____deadlock) | NEG | ✅ | 20 | ✅ | 2 | 10→7 | 37→0 | 45→13 | 0→0 | 0→9 | 0→12 | 0→2 | 146→86 |
| 21 | [struct_condvar____lost_wakeup](#struct_condvar____lost_wakeup) | NEG | ✅ | 3 | ✅ | c2rust | 7→7 | 32→0 | 27→3 | 1→1 | 0→5 | 0→5 | 0→2 | 185→66 |
| 22 | [struct_dup____deadlock](#struct_dup____deadlock) | NEG | ✅ | c2rust | ✅ | 3 | 8→7 | 32→28 | 29→28 | 2→2 | 0→0 | 0→0 | 0→0 | 180→166 |
| 23 | [struct_init____partial_critical_section](#struct_init____partial_critical_section) | NEG | ✅ | c2rust | ✅ | c2rust | 7→7 | 29→35 | 34→36 | 2→2 | 0→0 | 0→0 | 0→0 | 157→160 |
| 24 | [struct_malloc2____lock_mismatch](#struct_malloc2____lock_mismatch) | NEG | ✅ | 4 | ✅ | c2rust | 8→8 | 35→0 | 34→2 | 2→1 | 0→3 | 0→6 | 0→2 | 145→64 |
| 25 | [struct_malloc____lost_wakeup](#struct_malloc____lost_wakeup) | NEG | ✅ | 1 | ✅ | c2rust | 7→3 | 41→0 | 38→5 | 0→0 | 0→0 | 0→3 | 0→2 | 158→55 |
| 26 | [struct_multiple____deadlock](#struct_multiple____deadlock) | NEG | ✅ | 4 | ✅ | 3 | 11→2 | 32→0 | 37→5 | 3→0 | 0→0 | 0→12 | 0→3 | 198→98 |
| 27 | [struct_nested____self_lock](#struct_nested____self_lock) | NEG | ✅ | 8 | ✅ | c2rust | 7→6 | 26→0 | 25→2 | 1→1 | 0→3 | 0→3 | 0→2 | 138→72 |
| 28 | [struct_simple____partial_critical_section](#struct_simple____partial_critical_section) | NEG | ✅ | c2rust | ✅ | 13 | 7→7 | 28→28 | 27→27 | 1→1 | 0→0 | 0→0 | 0→0 | 158→157 |
| 29 | [struct_spin____lock_leak](#struct_spin____lock_leak) | NEG | ✅ | 1 | ✅ | c2rust | 7→0 | 47→0 | 33→0 | 0→0 | 0→0 | 0→7 | 0→2 | 199→57 |
| 30 | [struct_timedwait____deadlock](#struct_timedwait____deadlock) | NEG | ✅ | c2rust | ✅ | 6 | 9→9 | 48→42 | 32→30 | 2→1 | 0→0 | 0→0 | 0→0 | 271→251 |
| 31 | [struct_timedwait____lost_wakeup](#struct_timedwait____lost_wakeup) | NEG | ✅ | c2rust | ✅ | 6 | 9→9 | 37→42 | 29→30 | 1→1 | 0→0 | 0→0 | 0→0 | 238→251 |
| 32 | [unused_func____lock_mismatch](#unused_func____lock_mismatch) | NEG | ✅ | 3 | ✅ | 2 | 8→6 | 28→0 | 27→2 | 3→0 | 0→3 | 0→3 | 0→1 | 135→68 |
| | **TOTAL** | (NEG) | 32/32 | — | 30/30 | — | 261→344 | 1083→806 | 990→800 | 70→82 | 0→102 | 0→248 | 0→72 | 5503→7198 |

> **Reading the table**: Each metric cell shows **Original → LLM**. **Pos** column shows if the corresponding positive sample (before `____`) compiles with LLM. **Pos Round** shows the last successful round (1-N) for the positive sample, or `c2rust` if none compiled successfully. Negative samples are expected to fail (used for validation).

## All Metrics Summary

This section displays all 15 metrics for each sample in a compact format.

| Example | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_arc | std\_rwlock | std\_condvar | std\_thread | move\_closure | arc\_clone | join\_handle | arc\_mutex\_combo | lines |
|---------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| array_const____deadlock | 10→0 | 64→0 | 58→0 | 2→0 | 0→0 | 0→9 | 0→3 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→0 | 233→70 |
| array_const____lock_mismatch | 7→7 | 42→42 | 41→41 | 2→2 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 183→183 |
| array_main____lock_leak | 7→7 | 27→32 | 34→37 | 2→2 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 207→211 |
| array_main____partial_critical_section | 7→6 | 32→0 | 37→3 | 2→2 | 0→4 | 0→8 | 0→8 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→2 | 212→78 |
| array_simple____partial_critical_section | 7→6 | 22→0 | 33→2 | 4→4 | 0→1 | 0→7 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 227→73 |
| global_assume2____self_lock | 8→6 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 124→71 |
| global_assume____lock_leak | 8→0 | 21→0 | 24→0 | 2→0 | 0→0 | 0→3 | 0→6 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→0 | 115→49 |
| global_check____lock_leak | 10→10 | 39→44 | 26→26 | 2→2 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 176→181 |
| global_check____lock_mismatch | 10→10 | 46→44 | 28→26 | 3→2 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 197→181 |
| global_condvar____lost_wakeup | 7→7 | 38→40 | 28→29 | 4→4 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 187→188 |
| global_condvar____partial_critical_section | 7→0 | 40→0 | 29→0 | 4→0 | 0→0 | 0→7 | 0→6 | 0→0 | 0→3 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 190→71 |
| global_custom____self_lock | 12→0 | 26→0 | 25→0 | 2→0 | 0→0 | 0→3 | 0→10 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 144→76 |
| global_main____self_lock | 7→7 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→1 | 0→0 | 0→0 | 0→2 | 0→0 | 0→0 | 0→2 | 0→0 | 117→68 |
| global_nested____deadlock | 8→2 | 48→0 | 28→3 | 4→0 | 0→6 | 0→5 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 175→71 |
| global_read____lock_mismatch | 8→8 | 28→0 | 28→6 | 4→4 | 0→6 | 0→7 | 0→7 | 0→0 | 0→0 | 0→2 | 0→0 | 0→0 | 0→2 | 0→4 | 141→80 |
| global_rwlock____lock_leak | 8→1 | 27→0 | 22→2 | 2→0 | 0→1 | 0→7 | 0→4 | 0→5 | 0→0 | 0→1 | 0→1 | 0→2 | 0→1 | 0→1 | 124→85 |
| global_simple____partial_critical_section | 7→7 | 24→0 | 25→4 | 4→4 | 0→2 | 0→4 | 0→5 | 0→0 | 0→0 | 0→2 | 0→2 | 0→1 | 0→2 | 0→2 | 125→65 |
| global_while____lock_leak | 8→0 | 27→0 | 25→0 | 2→0 | 0→0 | 0→5 | 0→5 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→0 | 131→62 |
| struct_alias____self_lock | 10→10 | 28→26 | 32→32 | 3→3 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 187→185 |
| struct_assume____deadlock | 10→7 | 37→0 | 45→13 | 0→0 | 0→9 | 0→12 | 0→9 | 0→0 | 0→0 | 0→2 | 0→2 | 0→4 | 0→2 | 0→6 | 146→86 |
| struct_condvar____lost_wakeup | 7→7 | 32→0 | 27→3 | 1→1 | 0→5 | 0→5 | 0→4 | 0→0 | 0→3 | 0→2 | 0→2 | 0→1 | 0→2 | 0→0 | 185→66 |
| struct_dup____deadlock | 8→7 | 32→28 | 29→28 | 2→2 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 180→166 |
| struct_init____partial_critical_section | 7→7 | 29→35 | 34→36 | 2→2 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 157→160 |
| struct_malloc2____lock_mismatch | 8→8 | 35→0 | 34→2 | 2→1 | 0→3 | 0→6 | 0→4 | 0→0 | 0→0 | 0→2 | 0→0 | 0→2 | 0→2 | 0→0 | 145→64 |
| struct_malloc____lost_wakeup | 7→3 | 41→0 | 38→5 | 0→0 | 0→0 | 0→3 | 0→7 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 158→55 |
| struct_multiple____deadlock | 11→2 | 32→0 | 37→5 | 3→0 | 0→0 | 0→12 | 0→15 | 0→0 | 0→0 | 0→3 | 0→3 | 0→3 | 0→3 | 0→8 | 198→98 |
| struct_nested____self_lock | 7→6 | 26→0 | 25→2 | 1→1 | 0→3 | 0→3 | 0→4 | 0→0 | 0→0 | 0→2 | 0→2 | 0→1 | 0→2 | 0→0 | 138→72 |
| struct_simple____partial_critical_section | 7→7 | 28→28 | 27→27 | 1→1 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 158→157 |
| struct_spin____lock_leak | 7→0 | 47→0 | 33→0 | 0→0 | 0→0 | 0→7 | 0→4 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 199→57 |
| struct_timedwait____deadlock | 9→9 | 48→42 | 32→30 | 2→1 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 271→251 |
| struct_timedwait____lost_wakeup | 9→9 | 37→42 | 29→30 | 1→1 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 238→251 |
| unused_func____lock_mismatch | 8→6 | 28→0 | 27→2 | 3→0 | 0→3 | 0→3 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 135→68 |
| **TOTAL** | 261→344 | 1083→806 | 990→800 | 70→82 | 0→102 | 0→248 | 0→224 | 0→10 | 0→16 | 0→72 | 0→52 | 0→58 | 0→72 | 0→46 | 5503→7198 |

> **All Metrics** table shows all 15 metrics (including std\_arc, std\_rwlock, std\_condvar, move\_closure, arc\_clone, join\_handle, arc\_mutex\_combo) for each sample. Format: **Original → ConCrat → LLM** (or **Original → LLM** for negative samples).

## Aggregate Statistics

| Metric | Original | LLM | vs Original |
|--------|----------|-----|:------------:|
| unsafe | 261 | 344 | -31.8% |
| pthread | 1083 | 806 | +25.6% |
| raw\_ptr | 990 | 800 | +19.2% |
| static\_mut | 70 | 82 | -17.1% |
| libc | 0 | 102 | — |
| lines | 5503 | 7198 | -30.8% |

| **LLM compile success** | — | 32/32 (100%) |  |

## Metric Categories Summary

Aggregate of metrics grouped by category (Lower is Better vs Higher is Better):

| Category | Original | LLM | vs Original |
|----------|----------|-----|:------------:|
| **Lower is Better** (∑unsafe, pthread, raw_ptr, static_mut, libc, lines) | 7907 | 9332 | -18.0% |
| **Higher is Better** (∑std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 798 | — |

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
| lines | 233 | 70 | -163 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 367 | 70 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 16 |

---

### array_const____lock_mismatch

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 42 | 42 | +0 |
| raw_ptr | 41 | 41 | +0 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 183 | 183 | +0 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 275 | 275 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (8 calls); static mut variables remain: n1, num_mutex; 7 unsafe keyword(s) remain

---

### array_main____lock_leak

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 27 | 32 | +5 |
| raw_ptr | 34 | 37 | +3 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 207 | 211 | +4 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 277 | 289 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (4 calls); static mut variables remain: n1, num_mutex; 7 unsafe keyword(s) remain

---

### array_main____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 6 | -1 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 37 | 3 | -34 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 4 | +4 |
| lines | 212 | 78 | -134 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 290 | 93 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, NUM_MUTEX_PTR; 6 unsafe keyword(s) remain

---

### array_simple____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 6 | -1 |
| pthread | 22 | 0 | -22 |
| raw_ptr | 33 | 2 | -31 |
| static_mut | 4 | 4 | +0 |
| libc | 0 | 1 | +1 |
| lines | 227 | 73 | -154 |

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
| unsafe | 8 | 6 | -2 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 3 | -22 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 4 | +4 |
| lines | 124 | 71 | -53 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 85 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 |

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

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 10 | +0 |
| pthread | 39 | 44 | +5 |
| raw_ptr | 26 | 26 | +0 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 176 | 181 | +5 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 253 | 263 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (19 calls); static mut variables remain: n, m; 10 unsafe keyword(s) remain

---

### global_check____lock_mismatch

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 10 | +0 |
| pthread | 46 | 44 | -2 |
| raw_ptr | 28 | 26 | -2 |
| static_mut | 3 | 2 | -1 |
| libc | 0 | 0 | +0 |
| lines | 197 | 181 | -16 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 284 | 263 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (19 calls); static mut variables remain: n, m; 10 unsafe keyword(s) remain

---

### global_condvar____lost_wakeup

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 38 | 40 | +2 |
| raw_ptr | 28 | 29 | +1 |
| static_mut | 4 | 4 | +0 |
| libc | 0 | 0 | +0 |
| lines | 187 | 188 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 264 | 268 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (6 calls); static mut variables remain: n1, n2, num_mutex, cond; 7 unsafe keyword(s) remain

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
| lines | 190 | 71 | -119 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 270 | 71 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 24 |

---

### global_custom____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 12 | 0 | -12 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 0 | -25 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 144 | 76 | -68 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 209 | 76 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 21 |

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
| lines | 175 | 71 | -104 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 263 | 82 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

---

### global_read____lock_mismatch

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 8 | +0 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 28 | 6 | -22 |
| static_mut | 4 | 4 | +0 |
| libc | 0 | 6 | +6 |
| lines | 141 | 80 | -61 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 209 | 104 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 22 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1, N2, NUM_MUTEX1_PTR, NUM_MUTEX2_PTR; 8 unsafe keyword(s) remain

---

### global_rwlock____lock_leak

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 1 | -7 |
| pthread | 27 | 0 | -27 |
| raw_ptr | 22 | 2 | -20 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 1 | +1 |
| lines | 124 | 85 | -39 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 183 | 89 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 22 |

**Remaining Issues:**

- **LLM**: 1 unsafe keyword(s) remain

---

### global_simple____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 24 | 0 | -24 |
| raw_ptr | 25 | 4 | -21 |
| static_mut | 4 | 4 | +0 |
| libc | 0 | 2 | +2 |
| lines | 125 | 65 | -60 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 82 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 18 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1, N2, N3, NUM_MUTEX_INSTANCE; 7 unsafe keyword(s) remain

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
| lines | 131 | 62 | -69 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 193 | 62 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 19 |

---

### struct_alias____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 10 | +0 |
| pthread | 28 | 26 | -2 |
| raw_ptr | 32 | 32 | +0 |
| static_mut | 3 | 3 | +0 |
| libc | 0 | 0 | +0 |
| lines | 187 | 185 | -2 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 260 | 256 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (4 calls); static mut variables remain: s1, s2, s3; 10 unsafe keyword(s) remain

---

### struct_assume____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 7 | -3 |
| pthread | 37 | 0 | -37 |
| raw_ptr | 45 | 13 | -32 |
| static_mut | 0 | 0 | +0 |
| libc | 0 | 9 | +9 |
| lines | 146 | 86 | -60 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 238 | 115 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 37 |

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
| lines | 185 | 66 | -119 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 252 | 82 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 19 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 7 unsafe keyword(s) remain

---

### struct_dup____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 7 | -1 |
| pthread | 32 | 28 | -4 |
| raw_ptr | 29 | 28 | -1 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 180 | 166 | -14 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 251 | 231 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (6 calls); static mut variables remain: s1, s2; 7 unsafe keyword(s) remain

---

### struct_init____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 29 | 35 | +6 |
| raw_ptr | 34 | 36 | +2 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 157 | 160 | +3 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 229 | 240 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (8 calls); static mut variables remain: s1, s2; 7 unsafe keyword(s) remain

---

### struct_malloc2____lock_mismatch

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 8 | +0 |
| pthread | 35 | 0 | -35 |
| raw_ptr | 34 | 2 | -32 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 3 | +3 |
| lines | 145 | 64 | -81 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 224 | 78 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 16 |

**Remaining Issues:**

- **LLM**: static mut variables remain: X; 8 unsafe keyword(s) remain

---

### struct_malloc____lost_wakeup

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 3 | -4 |
| pthread | 41 | 0 | -41 |
| raw_ptr | 38 | 5 | -33 |
| static_mut | 0 | 0 | +0 |
| libc | 0 | 0 | +0 |
| lines | 158 | 55 | -103 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 244 | 63 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### struct_multiple____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 11 | 2 | -9 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 37 | 5 | -32 |
| static_mut | 3 | 0 | -3 |
| libc | 0 | 0 | +0 |
| lines | 198 | 98 | -100 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 281 | 105 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 47 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

---

### struct_nested____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 6 | -1 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 2 | -23 |
| static_mut | 1 | 1 | +0 |
| libc | 0 | 3 | +3 |
| lines | 138 | 72 | -66 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 197 | 84 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 14 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 6 unsafe keyword(s) remain

---

### struct_simple____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 28 | 28 | +0 |
| raw_ptr | 27 | 27 | +0 |
| static_mut | 1 | 1 | +0 |
| libc | 0 | 0 | +0 |
| lines | 158 | 157 | -1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 221 | 220 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (6 calls); static mut variables remain: s; 7 unsafe keyword(s) remain

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
| lines | 199 | 57 | -142 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 286 | 57 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 19 |

---

### struct_timedwait____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 9 | +0 |
| pthread | 48 | 42 | -6 |
| raw_ptr | 32 | 30 | -2 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 0 | +0 |
| lines | 271 | 251 | -20 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 362 | 333 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (8 calls); static mut variables remain: s; 9 unsafe keyword(s) remain

---

### struct_timedwait____lost_wakeup

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 9 | +0 |
| pthread | 37 | 42 | +5 |
| raw_ptr | 29 | 30 | +1 |
| static_mut | 1 | 1 | +0 |
| libc | 0 | 0 | +0 |
| lines | 238 | 251 | +13 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 314 | 333 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (8 calls); static mut variables remain: s; 9 unsafe keyword(s) remain

---

### unused_func____lock_mismatch

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 6 | -2 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 27 | 2 | -25 |
| static_mut | 3 | 0 | -3 |
| libc | 0 | 3 | +3 |
| lines | 135 | 68 | -67 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 201 | 79 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 6 |

**Remaining Issues:**

- **LLM**: 6 unsafe keyword(s) remain

---
