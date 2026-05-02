# Concurrency Transformation Comparison Report (Positive Samples Only)

Comparing **Original** vs **ConCrat** vs **LLM** for positive examples

## Summary Overview

| # | Example | Compiles (C / L) | Round | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_thread | lines |
|---|---------|:----------------:|:---:|------|------|------|------|------|------|------|------|
| 1 | [array_const](#array_const) | ✅ / ❌ | c2rust | 7→7→6 | 42→20→31 | 41→23→21 | 2→1→2 | 0→0→0 | 0→5→1 | 0→0→0 | 183→132→167 |
| 2 | [array_main](#array_main) | ✅ / ✅ | 14 | 7→7→4 | 32→20→0 | 37→23→3 | 2→1→1 | 0→0→5 | 0→7→7 | 0→0→1 | 211→151→76 |
| 3 | [array_simple](#array_simple) | ✅ / ❌ | c2rust | 7→7→6 | 32→20→0 | 39→25→2 | 4→2→4 | 0→0→3 | 0→7→9 | 0→0→1 | 235→197→82 |
| 4 | [global_assume](#global_assume) | ✅ / ✅ | 4 | 8→8→9 | 24→20→0 | 25→23→3 | 2→1→2 | 0→0→2 | 0→3→3 | 0→0→2 | 117→118→63 |
| 5 | [global_assume2](#global_assume2) | ✅ / ❌ | c2rust | 8→8→5 | 24→20→0 | 25→23→2 | 2→1→1 | 0→0→3 | 0→3→4 | 0→0→1 | 122→129→70 |
| 6 | [global_check](#global_check) | ✅ / ❌ | c2rust | 10→10→0 | 44→22→0 | 26→24→0 | 2→1→0 | 0→6→0 | 0→3→3 | 0→0→2 | 181→243→99 |
| 7 | [global_condvar](#global_condvar) | ✅ / ✅ | 2 | 7→7→0 | 40→28→0 | 29→27→0 | 4→2→0 | 0→0→0 | 0→3→7 | 0→0→2 | 188→172→61 |
| 8 | [global_custom](#global_custom) | ✅ / ✅ | 3 | 12→12→11 | 26→20→0 | 25→23→4 | 2→1→2 | 0→0→2 | 0→3→4 | 0→0→2 | 142→156→93 |
| 9 | [global_main](#global_main) | ✅ / ✅ | 1 | 7→7→0 | 24→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→1 | 115→113→49 |
| 10 | [global_nested](#global_nested) | ✅ / ✅ | 1 | 7→7→0 | 36→20→0 | 27→23→0 | 4→2→0 | 0→0→0 | 0→5→5 | 0→0→2 | 148→137→52 |
| 11 | [global_read](#global_read) | ✅ / ✅ | 3 | 7→7→0 | 24→20→0 | 25→23→0 | 3→2→0 | 0→0→0 | 0→3→7 | 0→0→2 | 115→115→44 |
| 12 | [global_rwlock](#global_rwlock) | ✅ / ❌ | c2rust | 8→8→5 | 28→22→0 | 22→22→3 | 2→1→1 | 0→0→4 | 0→1→5 | 0→0→1 | 125→116→97 |
| 13 | [global_simple](#global_simple) | ✅ / ❌ | c2rust | 7→7→3 | 24→20→0 | 25→23→0 | 4→2→3 | 0→0→0 | 0→3→3 | 0→0→2 | 124→120→53 |
| 14 | [global_trylock](#global_trylock) | ✅ / ✅ | 1 | 7→7→0 | 38→38→0 | 26→26→0 | 2→2→0 | 0→0→0 | 0→0→3 | 0→0→2 | 153→153→57 |
| 15 | [global_while](#global_while) | ✅ / ✅ | 2 | 8→8→0 | 29→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→2 | 133→133→58 |
| 16 | [struct_alias](#struct_alias) | ✅ / ❌ | c2rust | 10→10→8 | 26→20→0 | 32→26→2 | 3→3→0 | 0→0→4 | 0→5→11 | 0→0→2 | 185→156→97 |
| 17 | [struct_assume](#struct_assume) | ✅ / ✅ | 2 | 8→8→4 | 29→24→0 | 33→33→6 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→1 | 112→129→59 |
| 18 | [struct_condvar](#struct_condvar) | ✅ / ✅ | 3 | 7→7→6 | 34→26→0 | 28→26→3 | 1→1→1 | 0→0→5 | 0→3→5 | 0→0→1 | 186→164→71 |
| 19 | [struct_dup](#struct_dup) | ✅ / ✅ | 3 | 7→7→9 | 28→20→0 | 28→24→4 | 2→2→2 | 0→0→7 | 0→5→5 | 0→0→2 | 166→147→87 |
| 20 | [struct_empty](#struct_empty) | ✅ / ❌ | c2rust | 7→7→4 | 30→24→0 | 34→32→7 | 1→1→0 | 0→0→7 | 0→4→7 | 0→0→2 | 142→133→54 |
| 21 | [struct_init](#struct_init) | ✅ / ❌ | c2rust | 7→7→4 | 35→24→0 | 36→32→5 | 2→2→0 | 0→0→0 | 0→5→10 | 0→0→2 | 160→136→58 |
| 22 | [struct_main](#struct_main) | ✅ / ✅ | 2 | 7→7→1 | 24→20→0 | 25→23→0 | 1→1→0 | 0→0→1 | 0→3→3 | 0→0→2 | 124→120→44 |
| 23 | [struct_malloc](#struct_malloc) | ✅ / ❌ | c2rust | 7→7→3 | 44→34→0 | 39→39→5 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→2 | 163→179→58 |
| 24 | [struct_malloc2](#struct_malloc2) | ✅ / ✅ | 1 | 8→8→7 | 33→24→0 | 32→32→3 | 1→1→1 | 0→0→3 | 0→4→4 | 0→0→2 | 120→135→66 |
| 25 | [struct_multiple](#struct_multiple) | ✅ / ✅ | 3 | 8→8→6 | 26→20→0 | 30→24→7 | 3→3→3 | 0→0→5 | 0→5→5 | 0→0→2 | 175→138→88 |
| 26 | [struct_nested](#struct_nested) | ✅ / ❌ | c2rust | 7→7→4 | 24→20→6 | 25→23→3 | 1→1→1 | 0→0→11 | 0→3→3 | 0→0→0 | 136→129→72 |
| 27 | [struct_simple](#struct_simple) | ✅ / ❌ | c2rust | 7→7→1 | 28→20→0 | 27→23→0 | 1→1→1 | 0→0→0 | 0→5→10 | 0→0→2 | 157→138→70 |
| 28 | [struct_spin](#struct_spin) | ✅ / ❌ | c2rust | 7→7→1 | 56→40→0 | 36→36→3 | 0→0→0 | 0→0→0 | 0→7→5 | 0→0→2 | 205→197→61 |
| 29 | [struct_timedwait](#struct_timedwait) | ✅ / ❌ | c2rust | 9→9→0 | 42→26→0 | 30→28→0 | 1→1→0 | 0→3→0 | 0→3→3 | 0→0→2 | 251→278→96 |
| 30 | [unused_func](#unused_func) | ✅ / ✅ | 2 | 8→8→0 | 26→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→5 | 0→0→2 | 119→119→49 |
| | **TOTAL** | 30/30 / 16/30 | — | 231→231→107 | 952→692→37 | 882→778→86 | 58→39→25 | 0→9→62 | 0→113→149 | 0→0→49 | 4693→4483→2151 |

> **Reading the table**: Each metric cell shows **Original → ConCrat → LLM**. Compiles column shows **ConCrat / LLM**.

## All Metrics Summary

This section displays all 15 metrics for each sample in a compact format.

| Example | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_arc | std\_rwlock | std\_condvar | std\_thread | move\_closure | arc\_clone | join\_handle | arc\_mutex\_combo | lines |
|---------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| array_const | 7→7→6 | 42→20→31 | 41→23→21 | 2→1→2 | 0→0→0 | 0→5→1 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 183→132→167 |
| array_main | 7→7→4 | 32→20→0 | 37→23→3 | 2→1→1 | 0→0→5 | 0→7→7 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→0 | 0→0→0 | 0→0→1 | 0→0→1 | 211→151→76 |
| array_simple | 7→7→6 | 32→20→0 | 39→25→2 | 4→2→4 | 0→0→3 | 0→7→9 | 0→0→9 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→0 | 0→0→0 | 0→0→1 | 0→0→2 | 235→197→82 |
| global_assume | 8→8→9 | 24→20→0 | 25→23→3 | 2→1→2 | 0→0→2 | 0→3→3 | 0→0→3 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→1 | 0→0→2 | 0→0→1 | 117→118→63 |
| global_assume2 | 8→8→5 | 24→20→0 | 25→23→2 | 2→1→1 | 0→0→3 | 0→3→4 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→0 | 0→0→0 | 0→0→1 | 0→0→0 | 122→129→70 |
| global_check | 10→10→0 | 44→22→0 | 26→24→0 | 2→1→0 | 0→6→0 | 0→3→3 | 0→0→9 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 181→243→99 |
| global_condvar | 7→7→0 | 40→28→0 | 29→27→0 | 4→2→0 | 0→0→0 | 0→3→7 | 0→0→6 | 0→1→0 | 0→3→3 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 188→172→61 |
| global_custom | 12→12→11 | 26→20→0 | 25→23→4 | 2→1→2 | 0→0→2 | 0→3→4 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 142→156→93 |
| global_main | 7→7→0 | 24→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 115→113→49 |
| global_nested | 7→7→0 | 36→20→0 | 27→23→0 | 4→2→0 | 0→0→0 | 0→5→5 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 148→137→52 |
| global_read | 7→7→0 | 24→20→0 | 25→23→0 | 3→2→0 | 0→0→0 | 0→3→7 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 115→115→44 |
| global_rwlock | 8→8→5 | 28→22→0 | 22→22→3 | 2→1→1 | 0→0→4 | 0→1→5 | 0→0→3 | 0→3→7 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 125→116→97 |
| global_simple | 7→7→3 | 24→20→0 | 25→23→0 | 4→2→3 | 0→0→0 | 0→3→3 | 0→0→4 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→1 | 0→0→2 | 0→0→1 | 124→120→53 |
| global_trylock | 7→7→0 | 38→38→0 | 26→26→0 | 2→2→0 | 0→0→0 | 0→0→3 | 0→0→5 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 153→153→57 |
| global_while | 8→8→0 | 29→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→0 | 133→133→58 |
| struct_alias | 10→10→8 | 26→20→0 | 32→26→2 | 3→3→0 | 0→0→4 | 0→5→11 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→3 | 185→156→97 |
| struct_assume | 8→8→4 | 29→24→0 | 33→33→6 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→4 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 112→129→59 |
| struct_condvar | 7→7→6 | 34→26→0 | 28→26→3 | 1→1→1 | 0→0→5 | 0→3→5 | 0→0→4 | 0→1→0 | 0→3→3 | 0→0→1 | 0→0→1 | 0→0→2 | 0→0→1 | 0→0→0 | 186→164→71 |
| struct_dup | 7→7→9 | 28→20→0 | 28→24→4 | 2→2→2 | 0→0→7 | 0→5→5 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 166→147→87 |
| struct_empty | 7→7→4 | 30→24→0 | 34→32→7 | 1→1→0 | 0→0→7 | 0→4→7 | 0→0→9 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→1 | 142→133→54 |
| struct_init | 7→7→4 | 35→24→0 | 36→32→5 | 2→2→0 | 0→0→0 | 0→5→10 | 0→0→10 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 160→136→58 |
| struct_main | 7→7→1 | 24→20→0 | 25→23→0 | 1→1→0 | 0→0→1 | 0→3→3 | 0→0→2 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 124→120→44 |
| struct_malloc | 7→7→3 | 44→34→0 | 39→39→5 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→9 | 0→1→0 | 0→3→3 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 163→179→58 |
| struct_malloc2 | 8→8→7 | 33→24→0 | 32→32→3 | 1→1→1 | 0→0→3 | 0→4→4 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→0 | 120→135→66 |
| struct_multiple | 8→8→6 | 26→20→0 | 30→24→7 | 3→3→3 | 0→0→5 | 0→5→5 | 0→0→10 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→6 | 0→0→2 | 0→0→0 | 175→138→88 |
| struct_nested | 7→7→4 | 24→20→6 | 25→23→3 | 1→1→1 | 0→0→11 | 0→3→3 | 0→0→3 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→1 | 136→129→72 |
| struct_simple | 7→7→1 | 28→20→0 | 27→23→0 | 1→1→1 | 0→0→0 | 0→5→10 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→4 | 157→138→70 |
| struct_spin | 7→7→1 | 56→40→0 | 36→36→3 | 0→0→0 | 0→0→0 | 0→7→5 | 0→0→9 | 0→4→3 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 205→197→61 |
| struct_timedwait | 9→9→0 | 42→26→0 | 30→28→0 | 1→1→0 | 0→3→0 | 0→3→3 | 0→0→3 | 0→1→0 | 0→3→3 | 0→0→2 | 0→0→0 | 0→0→3 | 0→0→2 | 0→0→0 | 251→278→96 |
| unused_func | 8→8→0 | 26→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→5 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 119→119→49 |
| **TOTAL** | 231→231→107 | 952→692→37 | 882→778→86 | 58→39→25 | 0→9→62 | 0→113→149 | 0→0→167 | 0→34→10 | 0→37→12 | 0→0→49 | 0→0→40 | 0→0→48 | 0→0→49 | 0→0→20 | 4693→4483→2151 |

> **All Metrics** table shows all 15 metrics (including std\_arc, std\_rwlock, std\_condvar, move\_closure, arc\_clone, join\_handle, arc\_mutex\_combo) for each sample. Format: **Original → ConCrat → LLM** (or **Original → LLM** for negative samples).

## Aggregate Statistics

| Metric | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|--------|----------|---------|-----|:----------------:|:----------------:|
| unsafe | 231 | 231 | 107 | 0.0% | 53.7% |
| pthread | 952 | 692 | 37 | 27.3% | 96.1% |
| raw\_ptr | 882 | 778 | 86 | 11.8% | 90.2% |
| static\_mut | 58 | 39 | 25 | 32.8% | 56.9% |
| libc | 0 | 9 | 62 | — | — |
| std\_mutex | 0 | 113 | 149 | — | — |
| std\_arc | 0 | 0 | 167 | — | — |
| std\_rwlock | 0 | 34 | 10 | — | — |
| std\_condvar | 0 | 37 | 12 | — | — |
| std\_thread | 0 | 0 | 49 | — | — |
| move\_closure | 0 | 0 | 40 | — | — |
| arc\_clone | 0 | 0 | 48 | — | — |
| join\_handle | 0 | 0 | 49 | — | — |
| arc\_mutex\_combo | 0 | 0 | 20 | — | — |
| lines | 4693 | 4483 | 2151 | 4.5% | 54.2% |

| **Compile success** | — | 30/30 (100%) | 16/30 (53%) | | |

## Metric Categories Summary

Aggregate of metrics grouped by category (Lower is Better vs Higher is Better):

| Category | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|----------|----------|---------|-----|:----------------:|:----------------:|
| **Lower is Better** (∑unsafe, pthread, raw_ptr, static_mut, libc, lines) | 6816 | 6232 | 2468 | 8.6% | 63.8% |
| **Higher is Better** (∑std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 184 | 544 | — | — |

## Safety Features Adoption

| Example | Round | std::sync::Mutex | Arc<Mutex> | RwLock | Condvar | std::thread | join() |
|---------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| array_const | c2rust | ·,L | ·,· | C,· | C,· | ·,· | ·,· |
| array_main | 14 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| array_simple | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_assume | 4 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_assume2 | c2rust | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_check | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_condvar | 2 | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| global_custom | 3 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_main | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_nested | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_read | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_rwlock | c2rust | ·,L | ·,· | C,L | C,· | ·,L | ·,L |
| global_simple | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_trylock | 1 | ·,L | ·,L | ·,· | ·,· | ·,L | ·,L |
| global_while | 2 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_alias | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_assume | 2 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_condvar | 3 | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| struct_dup | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_empty | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_init | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_main | 2 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_malloc | c2rust | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| struct_malloc2 | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_multiple | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_nested | c2rust | ·,L | ·,L | C,· | C,· | ·,· | ·,· |
| struct_simple | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_spin | c2rust | ·,L | ·,L | C,L | C,· | ·,L | ·,L |
| struct_timedwait | c2rust | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| unused_func | 2 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |

> **C** = ConCrat uses it, **L** = LLM uses it, **·** = not used

## Per-Example Details

### array_const

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 6 | LLM |
| pthread | 42 | 20 | 31 | ConCrat |
| raw\_ptr | 41 | 23 | 21 | LLM |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 5 | 1 | ConCrat |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 183 | 132 | 167 | ConCrat |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 275 | 183 | 227 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 2 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (6 calls); static mut variables remain: n1, num_mutex; 6 unsafe keyword(s) remain

---

### array_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 32 | 20 | 0 | LLM |
| raw\_ptr | 37 | 23 | 3 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 7 | 7 | tie |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 211 | 151 | 76 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 289 | 202 | 89 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 9 | 17 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1; 4 unsafe keyword(s) remain

---

### array_simple

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 6 | LLM |
| pthread | 32 | 20 | 0 | LLM |
| raw\_ptr | 39 | 25 | 2 | LLM |
| static\_mut | 4 | 2 | 4 | ConCrat |
| libc | 0 | 0 | 3 | ConCrat |
| std\_mutex | 0 | 7 | 9 | LLM |
| std\_arc | 0 | 0 | 9 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 2 | LLM |
| lines | 235 | 197 | 82 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 317 | 251 | 97 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 9 | 22 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, n2, n3, NUM_MUTEX_INIT; 6 unsafe keyword(s) remain

---

### global_assume

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 9 | ConCrat |
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
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 117 | 118 | 63 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 176 | 170 | 79 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 14 |

**Remaining Issues:**

- **LLM**: static mut variables remain: NUM_MUTEX, n1; 9 unsafe keyword(s) remain

---

### global_assume2

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 5 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 2 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 0 | 0 | 3 | ConCrat |
| std\_mutex | 0 | 3 | 4 | LLM |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 122 | 129 | 70 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 181 | 181 | 81 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 7 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 5 unsafe keyword(s) remain

---

### global_check

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 10 | 10 | 0 | LLM |
| pthread | 44 | 22 | 0 | LLM |
| raw\_ptr | 26 | 24 | 0 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 0 | 6 | 0 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 9 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 181 | 243 | 99 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 263 | 306 | 99 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 20 |

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
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 188 | 172 | 61 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 268 | 236 | 61 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 24 |

---

### global_custom

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 12 | 12 | 11 | LLM |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 4 | LLM |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 0 | 0 | 2 | ConCrat |
| std\_mutex | 0 | 3 | 4 | LLM |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 2 | LLM |
| lines | 142 | 156 | 93 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 207 | 212 | 112 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 20 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1, NUM_MUTEX_ARC; 11 unsafe keyword(s) remain

---

### global_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 0 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 0 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 115 | 113 | 49 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 173 | 164 | 49 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 12 |

---

### global_nested

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 0 | LLM |
| pthread | 36 | 20 | 0 | LLM |
| raw\_ptr | 27 | 23 | 0 | LLM |
| static\_mut | 4 | 2 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 148 | 137 | 52 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 222 | 189 | 52 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 19 |

---

### global_read

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 0 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 0 | LLM |
| static\_mut | 3 | 2 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 7 | LLM |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 115 | 115 | 44 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 174 | 167 | 44 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 21 |

---

### global_rwlock

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 5 | LLM |
| pthread | 28 | 22 | 0 | LLM |
| raw\_ptr | 22 | 22 | 3 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 1 | 5 | LLM |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 3 | 7 | LLM |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 125 | 116 | 97 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 169 | 110 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 19 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N; 5 unsafe keyword(s) remain

---

### global_simple

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 3 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 0 | LLM |
| static\_mut | 4 | 2 | 3 | ConCrat |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 124 | 120 | 53 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 184 | 172 | 59 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 15 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, n2, n3; 3 unsafe keyword(s) remain

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
| std\_mutex | 0 | 0 | 3 | LLM |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 0 | 0 | tie |
| std\_condvar | 0 | 0 | 0 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 153 | 153 | 57 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 226 | 226 | 57 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 | 16 |

---

### global_while

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 0 | LLM |
| pthread | 29 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 0 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 3 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 133 | 133 | 58 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 197 | 185 | 58 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 17 |

---

### struct_alias

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 10 | 10 | 8 | LLM |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 32 | 26 | 2 | LLM |
| static\_mut | 3 | 3 | 0 | LLM |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 5 | 11 | LLM |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 3 | LLM |
| lines | 185 | 156 | 97 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 256 | 215 | 111 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 25 |

**Remaining Issues:**

- **LLM**: 8 unsafe keyword(s) remain

---

### struct_assume

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 4 | LLM |
| pthread | 29 | 24 | 0 | LLM |
| raw\_ptr | 33 | 33 | 6 | LLM |
| static\_mut | 0 | 0 | 0 | tie |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 112 | 129 | 59 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 182 | 194 | 69 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 11 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_condvar

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 6 | LLM |
| pthread | 34 | 26 | 0 | LLM |
| raw\_ptr | 28 | 26 | 3 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 3 | 5 | LLM |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 186 | 164 | 71 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 256 | 224 | 86 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 17 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 6 unsafe keyword(s) remain

---

### struct_dup

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 9 | ConCrat |
| pthread | 28 | 20 | 0 | LLM |
| raw\_ptr | 28 | 24 | 4 | LLM |
| static\_mut | 2 | 2 | 2 | tie |
| libc | 0 | 0 | 7 | ConCrat |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 166 | 147 | 87 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 231 | 200 | 109 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 20 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S1, S2; 9 unsafe keyword(s) remain

---

### struct_empty

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 30 | 24 | 0 | LLM |
| raw\_ptr | 34 | 32 | 7 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 0 | 0 | 7 | ConCrat |
| std\_mutex | 0 | 4 | 7 | LLM |
| std\_arc | 0 | 0 | 9 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 142 | 133 | 54 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 214 | 197 | 72 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 6 | 25 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_init

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 35 | 24 | 0 | LLM |
| raw\_ptr | 36 | 32 | 5 | LLM |
| static\_mut | 2 | 2 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 5 | 10 | LLM |
| std\_arc | 0 | 0 | 10 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 2 | LLM |
| lines | 160 | 136 | 58 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 240 | 201 | 67 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 30 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 1 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 0 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 0 | 0 | 1 | ConCrat |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 2 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 124 | 120 | 44 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 181 | 171 | 46 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 13 |

**Remaining Issues:**

- **LLM**: 1 unsafe keyword(s) remain

---

### struct_malloc

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 3 | LLM |
| pthread | 44 | 34 | 0 | LLM |
| raw\_ptr | 39 | 39 | 5 | LLM |
| static\_mut | 0 | 0 | 0 | tie |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 9 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 163 | 179 | 58 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 253 | 259 | 66 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 23 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### struct_malloc2

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 7 | LLM |
| pthread | 33 | 24 | 0 | LLM |
| raw\_ptr | 32 | 32 | 3 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 3 | ConCrat |
| std\_mutex | 0 | 4 | 4 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 120 | 135 | 66 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 194 | 200 | 80 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 6 | 9 |

**Remaining Issues:**

- **LLM**: static mut variables remain: x; 7 unsafe keyword(s) remain

---

### struct_multiple

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 6 | LLM |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 30 | 24 | 7 | LLM |
| static\_mut | 3 | 3 | 3 | tie |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 10 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 6 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 175 | 138 | 88 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 242 | 193 | 109 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 27 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S1, S2, S3; 6 unsafe keyword(s) remain

---

### struct_nested

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 24 | 20 | 6 | LLM |
| raw\_ptr | 25 | 23 | 3 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 11 | ConCrat |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 136 | 129 | 72 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 193 | 180 | 97 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 7 |

**Remaining Issues:**

- **LLM**: static mut variables remain: s; 4 unsafe keyword(s) remain

---

### struct_simple

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 1 | LLM |
| pthread | 28 | 20 | 0 | LLM |
| raw\_ptr | 27 | 23 | 0 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 5 | 10 | LLM |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 3 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 4 | LLM |
| lines | 157 | 138 | 70 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 220 | 189 | 72 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 29 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 1 unsafe keyword(s) remain

---

### struct_spin

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 1 | LLM |
| pthread | 56 | 40 | 0 | LLM |
| raw\_ptr | 36 | 36 | 3 | LLM |
| static\_mut | 0 | 0 | 0 | tie |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 7 | 5 | ConCrat |
| std\_arc | 0 | 0 | 9 | LLM |
| std\_rwlock | 0 | 4 | 3 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 2 | LLM |
| lines | 205 | 197 | 61 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 304 | 280 | 65 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 12 | 27 |

**Remaining Issues:**

- **LLM**: 1 unsafe keyword(s) remain

---

### struct_timedwait

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 9 | 9 | 0 | LLM |
| pthread | 42 | 26 | 0 | LLM |
| raw\_ptr | 30 | 28 | 0 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 0 | 3 | 0 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 3 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 251 | 278 | 96 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 333 | 345 | 96 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 16 |

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
