# Concurrency Transformation Comparison Report (Positive Samples Only)

Comparing **Original** vs **ConCrat** vs **LLM** for positive examples

## Summary Overview

| # | Example | Compiles (C / L) | Round | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_thread | lines |
|---|---------|:----------------:|:---:|------|------|------|------|------|------|------|------|
| 1 | [array_const](#array_const) | ✅ / ✅ | c2rust | 7→7→7 | 42→20→42 | 41→23→41 | 2→1→2 | 0→0→0 | 0→5→0 | 0→0→0 | 183→132→184 |
| 2 | [array_main](#array_main) | ✅ / ✅ | 4 | 7→7→4 | 32→20→0 | 37→23→2 | 2→1→1 | 0→0→3 | 0→7→7 | 0→0→1 | 211→151→77 |
| 3 | [array_simple](#array_simple) | ✅ / ✅ | c2rust | 7→7→7 | 32→20→32 | 39→25→39 | 4→2→4 | 0→0→0 | 0→7→0 | 0→0→0 | 235→197→236 |
| 4 | [global_assume](#global_assume) | ✅ / ✅ | 1 | 8→8→0 | 24→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→1 | 117→118→49 |
| 5 | [global_assume2](#global_assume2) | ✅ / ✅ | c2rust | 8→8→8 | 24→20→24 | 25→23→25 | 2→1→2 | 0→0→0 | 0→3→0 | 0→0→0 | 122→129→123 |
| 6 | [global_check](#global_check) | ✅ / ✅ | 3 | 10→10→13 | 44→22→0 | 26→24→2 | 2→1→1 | 0→6→4 | 0→3→4 | 0→0→2 | 181→243→101 |
| 7 | [global_condvar](#global_condvar) | ✅ / ✅ | 2 | 7→7→0 | 40→28→0 | 29→27→0 | 4→2→0 | 0→0→0 | 0→3→7 | 0→0→2 | 188→172→55 |
| 8 | [global_custom](#global_custom) | ✅ / ✅ | 3 | 12→12→11 | 26→20→0 | 25→23→4 | 2→1→2 | 0→0→5 | 0→3→4 | 0→0→2 | 142→156→92 |
| 9 | [global_main](#global_main) | ✅ / ✅ | 5 | 7→7→6 | 24→20→0 | 25→23→3 | 2→1→2 | 0→0→2 | 0→3→3 | 0→0→2 | 115→113→62 |
| 10 | [global_nested](#global_nested) | ✅ / ✅ | c2rust | 7→7→7 | 36→20→36 | 27→23→27 | 4→2→4 | 0→0→0 | 0→5→0 | 0→0→0 | 148→137→149 |
| 11 | [global_read](#global_read) | ✅ / ✅ | 6 | 7→7→2 | 24→20→0 | 25→23→0 | 3→2→0 | 0→0→2 | 0→3→6 | 0→0→2 | 115→115→57 |
| 12 | [global_rwlock](#global_rwlock) | ✅ / ✅ | 5 | 8→8→3 | 28→22→0 | 22→22→3 | 2→1→0 | 0→0→2 | 0→1→3 | 0→0→2 | 125→116→72 |
| 13 | [global_simple](#global_simple) | ✅ / ✅ | 7 | 7→7→0 | 24→20→0 | 25→23→0 | 4→2→0 | 0→0→0 | 0→3→6 | 0→0→1 | 124→120→56 |
| 14 | [global_trylock](#global_trylock) | ✅ / ✅ | 2 | 7→7→0 | 38→38→0 | 26→26→0 | 2→2→0 | 0→0→0 | 0→0→5 | 0→0→2 | 153→153→61 |
| 15 | [global_while](#global_while) | ✅ / ✅ | c2rust | 8→8→8 | 29→20→29 | 25→23→25 | 2→1→2 | 0→0→0 | 0→3→0 | 0→0→0 | 133→133→134 |
| 16 | [struct_alias](#struct_alias) | ✅ / ✅ | c2rust | 10→10→10 | 26→20→26 | 32→26→32 | 3→3→3 | 0→0→0 | 0→5→0 | 0→0→0 | 185→156→186 |
| 17 | [struct_assume](#struct_assume) | ✅ / ✅ | c2rust | 8→8→8 | 29→24→29 | 33→33→33 | 0→0→0 | 0→0→0 | 0→3→0 | 0→0→0 | 112→129→113 |
| 18 | [struct_condvar](#struct_condvar) | ✅ / ✅ | 9 | 7→7→4 | 34→26→0 | 28→26→3 | 1→1→0 | 0→0→5 | 0→3→5 | 0→0→1 | 186→164→60 |
| 19 | [struct_dup](#struct_dup) | ✅ / ✅ | 5 | 7→7→4 | 28→20→0 | 28→24→2 | 2→2→0 | 0→0→2 | 0→5→9 | 0→0→2 | 166→147→81 |
| 20 | [struct_empty](#struct_empty) | ✅ / ✅ | 4 | 7→7→5 | 30→24→0 | 34→32→7 | 1→1→0 | 0→0→5 | 0→4→9 | 0→0→2 | 142→133→62 |
| 21 | [struct_init](#struct_init) | ✅ / ✅ | c2rust | 7→7→7 | 35→24→35 | 36→32→36 | 2→2→2 | 0→0→0 | 0→5→0 | 0→0→0 | 160→136→161 |
| 22 | [struct_main](#struct_main) | ✅ / ✅ | 4 | 7→7→7 | 24→20→0 | 25→23→2 | 1→1→1 | 0→0→4 | 0→3→4 | 0→0→2 | 124→120→68 |
| 23 | [struct_malloc](#struct_malloc) | ✅ / ✅ | c2rust | 7→7→7 | 44→34→44 | 39→39→39 | 0→0→0 | 0→0→0 | 0→3→0 | 0→0→0 | 163→179→164 |
| 24 | [struct_malloc2](#struct_malloc2) | ✅ / ✅ | c2rust | 8→8→8 | 33→24→33 | 32→32→32 | 1→1→1 | 0→0→0 | 0→4→0 | 0→0→0 | 120→135→121 |
| 25 | [struct_multiple](#struct_multiple) | ✅ / ✅ | 3 | 8→8→10 | 26→20→0 | 30→24→3 | 3→3→3 | 0→0→2 | 0→5→5 | 0→0→2 | 175→138→90 |
| 26 | [struct_nested](#struct_nested) | ✅ / ✅ | 3 | 7→7→6 | 24→20→0 | 25→23→2 | 1→1→0 | 0→0→4 | 0→3→3 | 0→0→2 | 136→129→64 |
| 27 | [struct_simple](#struct_simple) | ✅ / ✅ | 3 | 7→7→6 | 28→20→0 | 27→23→3 | 1→1→1 | 0→0→5 | 0→5→5 | 0→0→2 | 157→138→78 |
| 28 | [struct_spin](#struct_spin) | ✅ / ✅ | c2rust | 7→7→7 | 56→40→56 | 36→36→36 | 0→0→0 | 0→0→0 | 0→7→0 | 0→0→0 | 205→197→206 |
| 29 | [struct_timedwait](#struct_timedwait) | ✅ / ✅ | c2rust | 9→9→9 | 42→26→42 | 30→28→30 | 1→1→1 | 0→3→0 | 0→3→0 | 0→0→0 | 251→278→252 |
| 30 | [unused_func](#unused_func) | ✅ / ✅ | 2 | 8→8→0 | 26→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→5 | 0→0→2 | 119→119→49 |
| | **TOTAL** | 30/30 / 30/30 | — | 231→231→174 | 952→692→428 | 882→778→431 | 58→39→32 | 0→9→45 | 0→113→93 | 0→0→32 | 4693→4483→3263 |

> **Reading the table**: Each metric cell shows **Original → ConCrat → LLM**. Compiles column shows **ConCrat / LLM**.

## All Metrics Summary

This section displays all 15 metrics for each sample in a compact format.

| Example | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_arc | std\_rwlock | std\_condvar | std\_thread | move\_closure | arc\_clone | join\_handle | arc\_mutex\_combo | lines |
|---------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| array_const | 7→7→7 | 42→20→42 | 41→23→41 | 2→1→2 | 0→0→0 | 0→5→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 183→132→184 |
| array_main | 7→7→4 | 32→20→0 | 37→23→2 | 2→1→1 | 0→0→3 | 0→7→7 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→0 | 0→0→0 | 0→0→1 | 0→0→1 | 211→151→77 |
| array_simple | 7→7→7 | 32→20→32 | 39→25→39 | 4→2→4 | 0→0→0 | 0→7→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 235→197→236 |
| global_assume | 8→8→0 | 24→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 117→118→49 |
| global_assume2 | 8→8→8 | 24→20→24 | 25→23→25 | 2→1→2 | 0→0→0 | 0→3→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 122→129→123 |
| global_check | 10→10→13 | 44→22→0 | 26→24→2 | 2→1→1 | 0→6→4 | 0→3→4 | 0→0→3 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→1 | 0→0→2 | 0→0→0 | 181→243→101 |
| global_condvar | 7→7→0 | 40→28→0 | 29→27→0 | 4→2→0 | 0→0→0 | 0→3→7 | 0→0→5 | 0→1→0 | 0→3→3 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 188→172→55 |
| global_custom | 12→12→11 | 26→20→0 | 25→23→4 | 2→1→2 | 0→0→5 | 0→3→4 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 142→156→92 |
| global_main | 7→7→6 | 24→20→0 | 25→23→3 | 2→1→2 | 0→0→2 | 0→3→3 | 0→0→3 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→1 | 115→113→62 |
| global_nested | 7→7→7 | 36→20→36 | 27→23→27 | 4→2→4 | 0→0→0 | 0→5→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 148→137→149 |
| global_read | 7→7→2 | 24→20→0 | 25→23→0 | 3→2→0 | 0→0→2 | 0→3→6 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 115→115→57 |
| global_rwlock | 8→8→3 | 28→22→0 | 22→22→3 | 2→1→0 | 0→0→2 | 0→1→3 | 0→0→3 | 0→3→5 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→1 | 0→0→2 | 0→0→1 | 125→116→72 |
| global_simple | 7→7→0 | 24→20→0 | 25→23→0 | 4→2→0 | 0→0→0 | 0→3→6 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→2 | 124→120→56 |
| global_trylock | 7→7→0 | 38→38→0 | 26→26→0 | 2→2→0 | 0→0→0 | 0→0→5 | 0→0→6 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 153→153→61 |
| global_while | 8→8→8 | 29→20→29 | 25→23→25 | 2→1→2 | 0→0→0 | 0→3→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 133→133→134 |
| struct_alias | 10→10→10 | 26→20→26 | 32→26→32 | 3→3→3 | 0→0→0 | 0→5→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 185→156→186 |
| struct_assume | 8→8→8 | 29→24→29 | 33→33→33 | 0→0→0 | 0→0→0 | 0→3→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 112→129→113 |
| struct_condvar | 7→7→4 | 34→26→0 | 28→26→3 | 1→1→0 | 0→0→5 | 0→3→5 | 0→0→3 | 0→1→0 | 0→3→3 | 0→0→1 | 0→0→1 | 0→0→2 | 0→0→1 | 0→0→0 | 186→164→60 |
| struct_dup | 7→7→4 | 28→20→0 | 28→24→2 | 2→2→0 | 0→0→2 | 0→5→9 | 0→0→9 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→4 | 166→147→81 |
| struct_empty | 7→7→5 | 30→24→0 | 34→32→7 | 1→1→0 | 0→0→5 | 0→4→9 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→3 | 142→133→62 |
| struct_init | 7→7→7 | 35→24→35 | 36→32→36 | 2→2→2 | 0→0→0 | 0→5→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 160→136→161 |
| struct_main | 7→7→7 | 24→20→0 | 25→23→2 | 1→1→1 | 0→0→4 | 0→3→4 | 0→0→4 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→2 | 124→120→68 |
| struct_malloc | 7→7→7 | 44→34→44 | 39→39→39 | 0→0→0 | 0→0→0 | 0→3→0 | 0→0→0 | 0→1→0 | 0→3→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 163→179→164 |
| struct_malloc2 | 8→8→8 | 33→24→33 | 32→32→32 | 1→1→1 | 0→0→0 | 0→4→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 120→135→121 |
| struct_multiple | 8→8→10 | 26→20→0 | 30→24→3 | 3→3→3 | 0→0→2 | 0→5→5 | 0→0→11 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→0 | 175→138→90 |
| struct_nested | 7→7→6 | 24→20→0 | 25→23→2 | 1→1→0 | 0→0→4 | 0→3→3 | 0→0→3 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→0 | 136→129→64 |
| struct_simple | 7→7→6 | 28→20→0 | 27→23→3 | 1→1→1 | 0→0→5 | 0→5→5 | 0→0→4 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→1 | 0→0→2 | 0→0→0 | 157→138→78 |
| struct_spin | 7→7→7 | 56→40→56 | 36→36→36 | 0→0→0 | 0→0→0 | 0→7→0 | 0→0→0 | 0→4→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 205→197→206 |
| struct_timedwait | 9→9→9 | 42→26→42 | 30→28→30 | 1→1→1 | 0→3→0 | 0→3→0 | 0→0→0 | 0→1→0 | 0→3→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 251→278→252 |
| unused_func | 8→8→0 | 26→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→5 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 119→119→49 |
| **TOTAL** | 231→231→174 | 952→692→428 | 882→778→431 | 58→39→32 | 0→9→45 | 0→113→93 | 0→0→97 | 0→34→5 | 0→37→6 | 0→0→32 | 0→0→29 | 0→0→31 | 0→0→32 | 0→0→18 | 4693→4483→3263 |

> **All Metrics** table shows all 15 metrics (including std\_arc, std\_rwlock, std\_condvar, move\_closure, arc\_clone, join\_handle, arc\_mutex\_combo) for each sample. Format: **Original → ConCrat → LLM** (or **Original → LLM** for negative samples).

## Aggregate Statistics

| Metric | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|--------|----------|---------|-----|:----------------:|:----------------:|
| unsafe | 231 | 231 | 174 | 0.0% | 24.7% |
| pthread | 952 | 692 | 428 | 27.3% | 55.0% |
| raw\_ptr | 882 | 778 | 431 | 11.8% | 51.1% |
| static\_mut | 58 | 39 | 32 | 32.8% | 44.8% |
| libc | 0 | 9 | 45 | — | — |
| std\_mutex | 0 | 113 | 93 | — | — |
| std\_arc | 0 | 0 | 97 | — | — |
| std\_rwlock | 0 | 34 | 5 | — | — |
| std\_condvar | 0 | 37 | 6 | — | — |
| std\_thread | 0 | 0 | 32 | — | — |
| move\_closure | 0 | 0 | 29 | — | — |
| arc\_clone | 0 | 0 | 31 | — | — |
| join\_handle | 0 | 0 | 32 | — | — |
| arc\_mutex\_combo | 0 | 0 | 18 | — | — |
| lines | 4693 | 4483 | 3263 | 4.5% | 30.5% |

| **Compile success** | — | 30/30 (100%) | 30/30 (100%) | | |

## Metric Categories Summary

Aggregate of metrics grouped by category (Lower is Better vs Higher is Better):

| Category | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|----------|----------|---------|-----|:----------------:|:----------------:|
| **Lower is Better** (∑unsafe, pthread, raw_ptr, static_mut, libc, lines) | 6816 | 6232 | 4373 | 8.6% | 35.8% |
| **Higher is Better** (∑std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 184 | 343 | — | — |

## Safety Features Adoption

| Example | Round | std::sync::Mutex | Arc<Mutex> | RwLock | Condvar | std::thread | join() |
|---------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| array_const | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| array_main | 4 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| array_simple | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| global_assume | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_assume2 | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| global_check | 3 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_condvar | 2 | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| global_custom | 3 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_main | 5 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_nested | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| global_read | 6 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_rwlock | 5 | ·,L | ·,L | C,L | C,· | ·,L | ·,L |
| global_simple | 7 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_trylock | 2 | ·,L | ·,· | ·,· | ·,· | ·,L | ·,L |
| global_while | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_alias | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_assume | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_condvar | 9 | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| struct_dup | 5 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_empty | 4 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_init | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_main | 4 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_malloc | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_malloc2 | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_multiple | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_nested | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_simple | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_spin | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_timedwait | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| unused_func | 2 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |

> **C** = ConCrat uses it, **L** = LLM uses it, **·** = not used

## Per-Example Details

### array_const

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 7 | tie |
| pthread | 42 | 20 | 42 | ConCrat |
| raw\_ptr | 41 | 23 | 41 | ConCrat |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 5 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 183 | 132 | 184 | ConCrat |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 275 | 183 | 276 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (8 calls); static mut variables remain: n1, num_mutex; 7 unsafe keyword(s) remain

---

### array_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 32 | 20 | 0 | LLM |
| raw\_ptr | 37 | 23 | 2 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 0 | 0 | 3 | ConCrat |
| std\_mutex | 0 | 7 | 7 | tie |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 211 | 151 | 77 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 289 | 202 | 87 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 9 | 17 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1; 4 unsafe keyword(s) remain

---

### array_simple

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 7 | tie |
| pthread | 32 | 20 | 32 | ConCrat |
| raw\_ptr | 39 | 25 | 39 | ConCrat |
| static\_mut | 4 | 2 | 4 | ConCrat |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 7 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 235 | 197 | 236 | ConCrat |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 317 | 251 | 318 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 9 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (4 calls); static mut variables remain: n1, n2, n3, num_mutex; 7 unsafe keyword(s) remain

---

### global_assume

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 0 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 0 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 117 | 118 | 49 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 176 | 170 | 49 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 13 |

---

### global_assume2

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 8 | tie |
| pthread | 24 | 20 | 24 | ConCrat |
| raw\_ptr | 25 | 23 | 25 | ConCrat |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 122 | 129 | 123 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 181 | 181 | 182 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (4 calls); static mut variables remain: n1, num_mutex; 8 unsafe keyword(s) remain

---

### global_check

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 10 | 10 | 13 | ConCrat |
| pthread | 44 | 22 | 0 | LLM |
| raw\_ptr | 26 | 24 | 2 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 0 | 6 | 4 | LLM |
| std\_mutex | 0 | 3 | 4 | LLM |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 181 | 243 | 101 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 263 | 306 | 121 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 14 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N; 13 unsafe keyword(s) remain

---

### global_condvar

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 0 | LLM |
| pthread | 40 | 28 | 0 | LLM |
| raw\_ptr | 29 | 27 | 0 | LLM |
| static\_mut | 4 | 2 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 7 | LLM |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 188 | 172 | 55 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 268 | 236 | 55 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 23 |

---

### global_custom

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 12 | 12 | 11 | LLM |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 4 | LLM |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 3 | 4 | LLM |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 2 | LLM |
| lines | 142 | 156 | 92 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 207 | 212 | 114 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 20 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1, NUM_MUTEX_ARC; 11 unsafe keyword(s) remain

---

### global_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 6 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 3 | LLM |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 0 | 0 | 2 | ConCrat |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 115 | 113 | 62 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 173 | 164 | 75 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 15 |

**Remaining Issues:**

- **LLM**: static mut variables remain: NUM_MUTEX, N1; 6 unsafe keyword(s) remain

---

### global_nested

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 7 | tie |
| pthread | 36 | 20 | 36 | ConCrat |
| raw\_ptr | 27 | 23 | 27 | ConCrat |
| static\_mut | 4 | 2 | 4 | ConCrat |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 5 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 148 | 137 | 149 | ConCrat |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 222 | 189 | 223 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (14 calls); static mut variables remain: n1, n2, n1_mutex, n2_mutex; 7 unsafe keyword(s) remain

---

### global_read

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 2 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 0 | LLM |
| static\_mut | 3 | 2 | 0 | LLM |
| libc | 0 | 0 | 2 | ConCrat |
| std\_mutex | 0 | 3 | 6 | LLM |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 2 | LLM |
| lines | 115 | 115 | 57 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 174 | 167 | 61 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 22 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

---

### global_rwlock

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 3 | LLM |
| pthread | 28 | 22 | 0 | LLM |
| raw\_ptr | 22 | 22 | 3 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 0 | 0 | 2 | ConCrat |
| std\_mutex | 0 | 1 | 3 | LLM |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 3 | 5 | LLM |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 125 | 116 | 72 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 169 | 80 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 19 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### global_simple

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 0 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 0 | LLM |
| static\_mut | 4 | 2 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 6 | LLM |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 2 | LLM |
| lines | 124 | 120 | 56 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 184 | 172 | 56 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 17 |

---

### global_trylock

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 0 | LLM |
| pthread | 38 | 38 | 0 | LLM |
| raw\_ptr | 26 | 26 | 0 | LLM |
| static\_mut | 2 | 2 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 0 | 5 | LLM |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 0 | 0 | tie |
| std\_condvar | 0 | 0 | 0 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 153 | 153 | 61 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 226 | 226 | 61 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 | 19 |

---

### global_while

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 8 | tie |
| pthread | 29 | 20 | 29 | ConCrat |
| raw\_ptr | 25 | 23 | 25 | ConCrat |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 133 | 133 | 134 | ConCrat |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 197 | 185 | 198 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (9 calls); static mut variables remain: n1, num_mutex; 8 unsafe keyword(s) remain

---

### struct_alias

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 10 | 10 | 10 | tie |
| pthread | 26 | 20 | 26 | ConCrat |
| raw\_ptr | 32 | 26 | 32 | ConCrat |
| static\_mut | 3 | 3 | 3 | tie |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 5 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 185 | 156 | 186 | ConCrat |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 256 | 215 | 257 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (4 calls); static mut variables remain: s1, s2, s3; 10 unsafe keyword(s) remain

---

### struct_assume

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 8 | tie |
| pthread | 29 | 24 | 29 | ConCrat |
| raw\_ptr | 33 | 33 | 33 | tie |
| static\_mut | 0 | 0 | 0 | tie |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 112 | 129 | 113 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 182 | 194 | 183 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (6 calls); 8 unsafe keyword(s) remain

---

### struct_condvar

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 34 | 26 | 0 | LLM |
| raw\_ptr | 28 | 26 | 3 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 3 | 5 | LLM |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 186 | 164 | 60 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 256 | 224 | 72 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 16 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_dup

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 28 | 20 | 0 | LLM |
| raw\_ptr | 28 | 24 | 2 | LLM |
| static\_mut | 2 | 2 | 0 | LLM |
| libc | 0 | 0 | 2 | ConCrat |
| std\_mutex | 0 | 5 | 9 | LLM |
| std\_arc | 0 | 0 | 9 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 4 | LLM |
| lines | 166 | 147 | 81 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 231 | 200 | 89 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 26 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_empty

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 5 | LLM |
| pthread | 30 | 24 | 0 | LLM |
| raw\_ptr | 34 | 32 | 7 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 4 | 9 | LLM |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 3 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 3 | LLM |
| lines | 142 | 133 | 62 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 214 | 197 | 79 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 6 | 27 |

**Remaining Issues:**

- **LLM**: 5 unsafe keyword(s) remain

---

### struct_init

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 7 | tie |
| pthread | 35 | 24 | 35 | ConCrat |
| raw\_ptr | 36 | 32 | 36 | ConCrat |
| static\_mut | 2 | 2 | 2 | tie |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 5 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 160 | 136 | 161 | ConCrat |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 240 | 201 | 241 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (8 calls); static mut variables remain: s1, s2; 7 unsafe keyword(s) remain

---

### struct_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 7 | tie |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 2 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 3 | 4 | LLM |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 3 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 2 | LLM |
| lines | 124 | 120 | 68 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 181 | 171 | 82 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 19 |

**Remaining Issues:**

- **LLM**: static mut variables remain: SHARED_DATA; 7 unsafe keyword(s) remain

---

### struct_malloc

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 7 | tie |
| pthread | 44 | 34 | 44 | ConCrat |
| raw\_ptr | 39 | 39 | 39 | tie |
| static\_mut | 0 | 0 | 0 | tie |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 163 | 179 | 164 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 253 | 259 | 254 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (6 calls); 7 unsafe keyword(s) remain

---

### struct_malloc2

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 8 | tie |
| pthread | 33 | 24 | 33 | ConCrat |
| raw\_ptr | 32 | 32 | 32 | tie |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 4 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 120 | 135 | 121 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 194 | 200 | 195 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 6 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (9 calls); static mut variables remain: x; 8 unsafe keyword(s) remain

---

### struct_multiple

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 10 | ConCrat |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 30 | 24 | 3 | LLM |
| static\_mut | 3 | 3 | 3 | tie |
| libc | 0 | 0 | 2 | ConCrat |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 11 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 3 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 175 | 138 | 90 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 242 | 193 | 108 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 25 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S1, S2, S3; 10 unsafe keyword(s) remain

---

### struct_nested

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 6 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 2 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 3 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 136 | 129 | 64 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 193 | 180 | 76 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 15 |

**Remaining Issues:**

- **LLM**: 6 unsafe keyword(s) remain

---

### struct_simple

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 6 | LLM |
| pthread | 28 | 20 | 0 | LLM |
| raw\_ptr | 27 | 23 | 3 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 157 | 138 | 78 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 220 | 189 | 93 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 16 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 6 unsafe keyword(s) remain

---

### struct_spin

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 7 | tie |
| pthread | 56 | 40 | 56 | ConCrat |
| raw\_ptr | 36 | 36 | 36 | tie |
| static\_mut | 0 | 0 | 0 | tie |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 7 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 4 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 205 | 197 | 206 | ConCrat |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 304 | 280 | 305 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 12 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (6 calls); 7 unsafe keyword(s) remain

---

### struct_timedwait

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 9 | 9 | 9 | tie |
| pthread | 42 | 26 | 42 | ConCrat |
| raw\_ptr | 30 | 28 | 30 | ConCrat |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 3 | 0 | LLM |
| std\_mutex | 0 | 3 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 251 | 278 | 252 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 333 | 345 | 334 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (8 calls); static mut variables remain: s; 9 unsafe keyword(s) remain

---

### unused_func

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 0 | LLM |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 0 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 5 | LLM |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 119 | 119 | 49 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 180 | 171 | 49 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 20 |

---
