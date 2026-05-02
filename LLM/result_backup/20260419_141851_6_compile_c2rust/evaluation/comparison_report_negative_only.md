# Concurrency Transformation Comparison Report (Negative Samples Only)

Analyzing **Original** and **LLM** for negative examples (expected to fail)

## Summary Overview

| # | Example | Type | Compiles (L) | Round | Pos | Pos Round | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_thread | lines |
|---|---------|------|:----------:|:---:|:--:|:----------:|------|------|------|------|------|------|------|------|
| 1 | [array_const____deadlock](#array_const____deadlock) | NEG | ✅ | 5 | ✅ | 2 | 10→0 | 64→0 | 58→0 | 2→0 | 0→0 | 0→12 | 0→1 | 233→99 |
| 2 | [array_const____lock_mismatch](#array_const____lock_mismatch) | NEG | ✅ | c2rust | ✅ | 2 | 7→7 | 42→42 | 41→41 | 2→2 | 0→0 | 0→0 | 0→0 | 183→184 |
| 3 | [array_main____lock_leak](#array_main____lock_leak) | NEG | ✅ | c2rust | ✅ | c2rust | 7→7 | 27→27 | 34→34 | 2→2 | 0→0 | 0→0 | 0→0 | 207→208 |
| 4 | [array_main____partial_critical_section](#array_main____partial_critical_section) | NEG | ✅ | c2rust | ✅ | c2rust | 7→7 | 32→32 | 37→37 | 2→2 | 0→0 | 0→0 | 0→0 | 212→213 |
| 5 | [array_simple____partial_critical_section](#array_simple____partial_critical_section) | NEG | ✅ | 2 | ✅ | c2rust | 7→6 | 22→0 | 33→2 | 4→4 | 0→4 | 0→7 | 0→1 | 227→70 |
| 6 | [global_assume2____self_lock](#global_assume2____self_lock) | NEG | ✅ | 1 | ✅ | 1 | 8→5 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→1 | 124→71 |
| 7 | [global_assume____lock_leak](#global_assume____lock_leak) | NEG | ✅ | 1 | ✅ | 1 | 8→0 | 21→0 | 24→0 | 2→0 | 0→0 | 0→3 | 0→1 | 115→49 |
| 8 | [global_check____lock_leak](#global_check____lock_leak) | NEG | ✅ | 3 | ✅ | c2rust | 10→8 | 39→0 | 26→3 | 2→2 | 0→5 | 0→10 | 0→2 | 176→124 |
| 9 | [global_check____lock_mismatch](#global_check____lock_mismatch) | NEG | ✅ | c2rust | ✅ | c2rust | 10→10 | 46→46 | 28→28 | 3→3 | 0→0 | 0→0 | 0→0 | 197→198 |
| 10 | [global_condvar____lost_wakeup](#global_condvar____lost_wakeup) | NEG | ✅ | 2 | ✅ | 2 | 7→0 | 38→0 | 28→0 | 4→0 | 0→0 | 0→7 | 0→2 | 187→60 |
| 11 | [global_condvar____partial_critical_section](#global_condvar____partial_critical_section) | NEG | ✅ | 2 | ✅ | 2 | 7→0 | 40→0 | 29→0 | 4→0 | 0→0 | 0→7 | 0→2 | 190→66 |
| 12 | [global_custom____self_lock](#global_custom____self_lock) | NEG | ✅ | c2rust | ✅ | c2rust | 12→12 | 26→26 | 25→25 | 2→2 | 0→0 | 0→0 | 0→0 | 144→145 |
| 13 | [global_main____self_lock](#global_main____self_lock) | NEG | ✅ | 1 | ✅ | 1 | 7→7 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→2 | 117→68 |
| 14 | [global_nested____deadlock](#global_nested____deadlock) | NEG | ✅ | 1 | ✅ | c2rust | 8→2 | 48→0 | 28→3 | 4→0 | 0→6 | 0→5 | 0→2 | 175→73 |
| 15 | [global_read____lock_mismatch](#global_read____lock_mismatch) | NEG | ✅ | c2rust | ✅ | 6 | 8→8 | 28→28 | 28→28 | 4→4 | 0→0 | 0→0 | 0→0 | 141→142 |
| 16 | [global_rwlock____lock_leak](#global_rwlock____lock_leak) | NEG | ✅ | c2rust | ✅ | c2rust | 8→8 | 27→27 | 22→22 | 2→2 | 0→0 | 0→0 | 0→0 | 124→125 |
| 17 | [global_simple____partial_critical_section](#global_simple____partial_critical_section) | NEG | ✅ | 2 | ✅ | 7 | 7→1 | 24→0 | 25→1 | 4→0 | 0→2 | 0→7 | 0→1 | 125→69 |
| 18 | [global_while____lock_leak](#global_while____lock_leak) | NEG | ✅ | 1 | ✅ | c2rust | 8→0 | 27→0 | 25→0 | 2→0 | 0→0 | 0→3 | 0→2 | 131→61 |
| 19 | [struct_alias____self_lock](#struct_alias____self_lock) | NEG | ✅ | c2rust | ✅ | c2rust | 10→10 | 28→28 | 32→32 | 3→3 | 0→0 | 0→0 | 0→0 | 187→188 |
| 20 | [struct_assume____deadlock](#struct_assume____deadlock) | NEG | ✅ | c2rust | ✅ | 1 | 10→10 | 37→37 | 45→45 | 0→0 | 0→0 | 0→0 | 0→0 | 146→147 |
| 21 | [struct_condvar____lost_wakeup](#struct_condvar____lost_wakeup) | NEG | ✅ | 3 | ✅ | 3 | 7→7 | 32→0 | 27→3 | 1→1 | 0→5 | 0→5 | 0→2 | 185→71 |
| 22 | [struct_dup____deadlock](#struct_dup____deadlock) | NEG | ✅ | 8 | ✅ | 3 | 8→10 | 32→0 | 29→3 | 2→2 | 0→2 | 0→5 | 0→2 | 180→112 |
| 23 | [struct_init____partial_critical_section](#struct_init____partial_critical_section) | NEG | ✅ | c2rust | ✅ | c2rust | 7→7 | 29→29 | 34→34 | 2→2 | 0→0 | 0→0 | 0→0 | 157→158 |
| 24 | [struct_malloc2____lock_mismatch](#struct_malloc2____lock_mismatch) | NEG | ✅ | c2rust | ✅ | c2rust | 8→8 | 35→35 | 34→34 | 2→2 | 0→0 | 0→0 | 0→0 | 145→146 |
| 25 | [struct_malloc____lost_wakeup](#struct_malloc____lost_wakeup) | NEG | ✅ | c2rust | ✅ | c2rust | 7→7 | 41→41 | 38→38 | 0→0 | 0→0 | 0→0 | 0→0 | 158→159 |
| 26 | [struct_multiple____deadlock](#struct_multiple____deadlock) | NEG | ✅ | 3 | ❌ | c2rust | 11→0 | 32→0 | 37→0 | 3→0 | 0→0 | 0→5 | 0→3 | 198→73 |
| 27 | [struct_nested____self_lock](#struct_nested____self_lock) | NEG | ✅ | 4 | ✅ | 6 | 7→5 | 26→0 | 25→2 | 1→0 | 0→1 | 0→5 | 0→1 | 138→69 |
| 28 | [struct_simple____partial_critical_section](#struct_simple____partial_critical_section) | NEG | ✅ | 3 | ✅ | c2rust | 7→3 | 28→0 | 27→2 | 1→1 | 0→4 | 0→7 | 0→2 | 158→85 |
| 29 | [struct_spin____lock_leak](#struct_spin____lock_leak) | NEG | ✅ | 1 | ✅ | c2rust | 7→0 | 47→0 | 33→0 | 0→0 | 0→0 | 0→7 | 0→2 | 199→58 |
| 30 | [struct_timedwait____deadlock](#struct_timedwait____deadlock) | NEG | ✅ | c2rust | ✅ | c2rust | 9→9 | 48→48 | 32→32 | 2→2 | 0→0 | 0→0 | 0→0 | 271→272 |
| 31 | [struct_timedwait____lost_wakeup](#struct_timedwait____lost_wakeup) | NEG | ✅ | c2rust | ✅ | c2rust | 9→9 | 37→37 | 29→29 | 1→1 | 0→0 | 0→0 | 0→0 | 238→239 |
| 32 | [unused_func____lock_mismatch](#unused_func____lock_mismatch) | NEG | ✅ | c2rust | ✅ | 1 | 8→8 | 28→28 | 27→27 | 3→3 | 0→0 | 0→0 | 0→0 | 135→136 |
| | **TOTAL** | (NEG) | 32/32 | — | 29/30 | — | 261→362 | 1083→1022 | 990→1022 | 70→84 | 0→74 | 0→206 | 0→58 | 5503→7876 |

> **Reading the table**: Each metric cell shows **Original → LLM**. **Pos** column shows if the corresponding positive sample (before `____`) compiles with LLM. **Pos Round** shows the last successful round (1-N) for the positive sample, or `c2rust` if none compiled successfully. Negative samples are expected to fail (used for validation).

## All Metrics Summary

This section displays all 15 metrics for each sample in a compact format.

| Example | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_arc | std\_rwlock | std\_condvar | std\_thread | move\_closure | arc\_clone | join\_handle | arc\_mutex\_combo | lines |
|---------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| array_const____deadlock | 10→0 | 64→0 | 58→0 | 2→0 | 0→0 | 0→12 | 0→13 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→7 | 233→99 |
| array_const____lock_mismatch | 7→7 | 42→42 | 41→41 | 2→2 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 183→184 |
| array_main____lock_leak | 7→7 | 27→27 | 34→34 | 2→2 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 207→208 |
| array_main____partial_critical_section | 7→7 | 32→32 | 37→37 | 2→2 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 212→213 |
| array_simple____partial_critical_section | 7→6 | 22→0 | 33→2 | 4→4 | 0→4 | 0→7 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 227→70 |
| global_assume2____self_lock | 8→5 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 0→0 | 0→1 | 0→0 | 124→71 |
| global_assume____lock_leak | 8→0 | 21→0 | 24→0 | 2→0 | 0→0 | 0→3 | 0→6 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→0 | 115→49 |
| global_check____lock_leak | 10→8 | 39→0 | 26→3 | 2→2 | 0→5 | 0→10 | 0→10 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→5 | 176→124 |
| global_check____lock_mismatch | 10→10 | 46→46 | 28→28 | 3→3 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 197→198 |
| global_condvar____lost_wakeup | 7→0 | 38→0 | 28→0 | 4→0 | 0→0 | 0→7 | 0→6 | 0→0 | 0→3 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 187→60 |
| global_condvar____partial_critical_section | 7→0 | 40→0 | 29→0 | 4→0 | 0→0 | 0→7 | 0→6 | 0→0 | 0→3 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 190→66 |
| global_custom____self_lock | 12→12 | 26→26 | 25→25 | 2→2 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 144→145 |
| global_main____self_lock | 7→7 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→1 | 0→0 | 0→0 | 0→2 | 0→0 | 0→0 | 0→2 | 0→0 | 117→68 |
| global_nested____deadlock | 8→2 | 48→0 | 28→3 | 4→0 | 0→6 | 0→5 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 175→73 |
| global_read____lock_mismatch | 8→8 | 28→28 | 28→28 | 4→4 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 141→142 |
| global_rwlock____lock_leak | 8→8 | 27→27 | 22→22 | 2→2 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 124→125 |
| global_simple____partial_critical_section | 7→1 | 24→0 | 25→1 | 4→0 | 0→2 | 0→7 | 0→5 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→0 | 125→69 |
| global_while____lock_leak | 8→0 | 27→0 | 25→0 | 2→0 | 0→0 | 0→3 | 0→5 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→0 | 131→61 |
| struct_alias____self_lock | 10→10 | 28→28 | 32→32 | 3→3 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 187→188 |
| struct_assume____deadlock | 10→10 | 37→37 | 45→45 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 146→147 |
| struct_condvar____lost_wakeup | 7→7 | 32→0 | 27→3 | 1→1 | 0→5 | 0→5 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→1 | 0→2 | 0→0 | 185→71 |
| struct_dup____deadlock | 8→10 | 32→0 | 29→3 | 2→2 | 0→2 | 0→5 | 0→7 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 180→112 |
| struct_init____partial_critical_section | 7→7 | 29→29 | 34→34 | 2→2 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 157→158 |
| struct_malloc2____lock_mismatch | 8→8 | 35→35 | 34→34 | 2→2 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 145→146 |
| struct_malloc____lost_wakeup | 7→7 | 41→41 | 38→38 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 158→159 |
| struct_multiple____deadlock | 11→0 | 32→0 | 37→0 | 3→0 | 0→0 | 0→5 | 0→12 | 0→0 | 0→0 | 0→3 | 0→3 | 0→3 | 0→3 | 0→0 | 198→73 |
| struct_nested____self_lock | 7→5 | 26→0 | 25→2 | 1→0 | 0→1 | 0→5 | 0→4 | 0→0 | 0→0 | 0→1 | 0→1 | 0→1 | 0→1 | 0→1 | 138→69 |
| struct_simple____partial_critical_section | 7→3 | 28→0 | 27→2 | 1→1 | 0→4 | 0→7 | 0→3 | 0→0 | 0→0 | 0→2 | 0→2 | 0→3 | 0→2 | 0→1 | 158→85 |
| struct_spin____lock_leak | 7→0 | 47→0 | 33→0 | 0→0 | 0→0 | 0→7 | 0→6 | 0→0 | 0→0 | 0→2 | 0→2 | 0→2 | 0→2 | 0→0 | 199→58 |
| struct_timedwait____deadlock | 9→9 | 48→48 | 32→32 | 2→2 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 271→272 |
| struct_timedwait____lost_wakeup | 9→9 | 37→37 | 29→29 | 1→1 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 238→239 |
| unused_func____lock_mismatch | 8→8 | 28→28 | 27→27 | 3→3 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 0→0 | 135→136 |
| **TOTAL** | 261→362 | 1083→1022 | 990→1022 | 70→84 | 0→74 | 0→206 | 0→200 | 0→0 | 0→12 | 0→58 | 0→50 | 0→54 | 0→58 | 0→28 | 5503→7876 |

> **All Metrics** table shows all 15 metrics (including std\_arc, std\_rwlock, std\_condvar, move\_closure, arc\_clone, join\_handle, arc\_mutex\_combo) for each sample. Format: **Original → ConCrat → LLM** (or **Original → LLM** for negative samples).

## Aggregate Statistics

| Metric | Original | LLM | vs Original |
|--------|----------|-----|:------------:|
| unsafe | 261 | 362 | -38.7% |
| pthread | 1083 | 1022 | +5.6% |
| raw\_ptr | 990 | 1022 | -3.2% |
| static\_mut | 70 | 84 | -20.0% |
| libc | 0 | 74 | — |
| lines | 5503 | 7876 | -43.1% |

| **LLM compile success** | — | 32/32 (100%) |  |

## Metric Categories Summary

Aggregate of metrics grouped by category (Lower is Better vs Higher is Better):

| Category | Original | LLM | vs Original |
|----------|----------|-----|:------------:|
| **Lower is Better** (∑unsafe, pthread, raw_ptr, static_mut, libc, lines) | 7907 | 10440 | -32.0% |
| **Higher is Better** (∑std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 666 | — |

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

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 42 | 42 | +0 |
| raw_ptr | 41 | 41 | +0 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 183 | 184 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 275 | 276 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (8 calls); static mut variables remain: n1, num_mutex; 7 unsafe keyword(s) remain

---

### array_main____lock_leak

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 27 | 27 | +0 |
| raw_ptr | 34 | 34 | +0 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 207 | 208 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 277 | 278 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (2 calls); Lock without matching unlock (potential deadlock); static mut variables remain: n1, num_mutex; 7 unsafe keyword(s) remain

---

### array_main____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 32 | 32 | +0 |
| raw_ptr | 37 | 37 | +0 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 212 | 213 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 290 | 291 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (4 calls); static mut variables remain: n1, num_mutex; 7 unsafe keyword(s) remain

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

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 10 | +0 |
| pthread | 46 | 46 | +0 |
| raw_ptr | 28 | 28 | +0 |
| static_mut | 3 | 3 | +0 |
| libc | 0 | 0 | +0 |
| lines | 197 | 198 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 284 | 285 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (19 calls); static mut variables remain: n, m, r; 10 unsafe keyword(s) remain

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

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 12 | 12 | +0 |
| pthread | 26 | 26 | +0 |
| raw_ptr | 25 | 25 | +0 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 144 | 145 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 209 | 210 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (6 calls); static mut variables remain: n1, num_mutex; 12 unsafe keyword(s) remain

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

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 8 | +0 |
| pthread | 28 | 28 | +0 |
| raw_ptr | 28 | 28 | +0 |
| static_mut | 4 | 4 | +0 |
| libc | 0 | 0 | +0 |
| lines | 141 | 142 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 209 | 210 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (6 calls); static mut variables remain: n1, n2, num_mutex1, num_mutex2; 8 unsafe keyword(s) remain

---

### global_rwlock____lock_leak

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 8 | +0 |
| pthread | 27 | 27 | +0 |
| raw_ptr | 22 | 22 | +0 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 124 | 125 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 183 | 184 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n, lock; 8 unsafe keyword(s) remain

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

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 10 | +0 |
| pthread | 28 | 28 | +0 |
| raw_ptr | 32 | 32 | +0 |
| static_mut | 3 | 3 | +0 |
| libc | 0 | 0 | +0 |
| lines | 187 | 188 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 260 | 261 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (6 calls); static mut variables remain: s1, s2, s3; 10 unsafe keyword(s) remain

---

### struct_assume____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 10 | +0 |
| pthread | 37 | 37 | +0 |
| raw_ptr | 45 | 45 | +0 |
| static_mut | 0 | 0 | +0 |
| libc | 0 | 0 | +0 |
| lines | 146 | 147 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 238 | 239 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (13 calls); 10 unsafe keyword(s) remain

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

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 29 | 29 | +0 |
| raw_ptr | 34 | 34 | +0 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 157 | 158 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 229 | 230 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (4 calls); static mut variables remain: s1, s2; 7 unsafe keyword(s) remain

---

### struct_malloc2____lock_mismatch

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 8 | +0 |
| pthread | 35 | 35 | +0 |
| raw_ptr | 34 | 34 | +0 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 145 | 146 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 224 | 225 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (9 calls); static mut variables remain: lock, x; 8 unsafe keyword(s) remain

---

### struct_malloc____lost_wakeup

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 41 | 41 | +0 |
| raw_ptr | 38 | 38 | +0 |
| static_mut | 0 | 0 | +0 |
| libc | 0 | 0 | +0 |
| lines | 158 | 159 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 244 | 245 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (6 calls); 7 unsafe keyword(s) remain

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

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 9 | +0 |
| pthread | 48 | 48 | +0 |
| raw_ptr | 32 | 32 | +0 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 271 | 272 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 362 | 363 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (12 calls); static mut variables remain: s, m2; 9 unsafe keyword(s) remain

---

### struct_timedwait____lost_wakeup

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 9 | +0 |
| pthread | 37 | 37 | +0 |
| raw_ptr | 29 | 29 | +0 |
| static_mut | 1 | 1 | +0 |
| libc | 0 | 0 | +0 |
| lines | 238 | 239 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 314 | 315 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (8 calls); static mut variables remain: s; 9 unsafe keyword(s) remain

---

### unused_func____lock_mismatch

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 8 | +0 |
| pthread | 28 | 28 | +0 |
| raw_ptr | 27 | 27 | +0 |
| static_mut | 3 | 3 | +0 |
| libc | 0 | 0 | +0 |
| lines | 135 | 136 | +1 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 201 | 202 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (6 calls); static mut variables remain: n1, num_mutex1, num_mutex2; 8 unsafe keyword(s) remain

---
