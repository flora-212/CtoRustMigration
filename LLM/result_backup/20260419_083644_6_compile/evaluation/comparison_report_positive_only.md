# Concurrency Transformation Comparison Report (Positive Samples Only)

Comparing **Original** vs **ConCrat** vs **LLM** for positive examples

## Summary Overview

| # | Example | Compiles (C / L) | Round | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_thread | lines |
|---|---------|:----------------:|:---:|------|------|------|------|------|------|------|------|
| 1 | [array_const](#array_const) | ✅ / ❌ | c2rust | 7→7→3 | 42→20→0 | 41→23→5 | 2→1→0 | 0→0→0 | 0→5→3 | 0→0→1 | 183→132→66 |
| 2 | [array_main](#array_main) | ✅ / ✅ | 1 | 7→7→4 | 32→20→0 | 37→23→3 | 2→1→2 | 0→0→5 | 0→7→7 | 0→0→2 | 211→151→70 |
| 3 | [array_simple](#array_simple) | ✅ / ✅ | 1 | 7→7→4 | 32→20→0 | 39→25→2 | 4→2→4 | 0→0→3 | 0→7→7 | 0→0→1 | 235→197→79 |
| 4 | [global_assume](#global_assume) | ✅ / ✅ | 1 | 8→8→8 | 24→20→0 | 25→23→3 | 2→1→1 | 0→0→5 | 0→3→3 | 0→0→2 | 117→118→49 |
| 5 | [global_assume2](#global_assume2) | ✅ / ❌ | c2rust | 8→8→3 | 24→20→0 | 25→23→3 | 2→1→0 | 0→0→4 | 0→3→3 | 0→0→2 | 122→129→59 |
| 6 | [global_check](#global_check) | ✅ / ✅ | 11 | 10→10→7 | 44→22→0 | 26→24→3 | 2→1→0 | 0→6→5 | 0→3→10 | 0→0→2 | 181→243→121 |
| 7 | [global_condvar](#global_condvar) | ✅ / ✅ | 2 | 7→7→0 | 40→28→0 | 29→27→0 | 4→2→0 | 0→0→0 | 0→3→7 | 0→0→2 | 188→172→61 |
| 8 | [global_custom](#global_custom) | ✅ / ✅ | 2 | 12→12→0 | 26→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→6 | 0→0→1 | 142→156→75 |
| 9 | [global_main](#global_main) | ✅ / ✅ | 1 | 7→7→0 | 24→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→2 | 115→113→46 |
| 10 | [global_nested](#global_nested) | ✅ / ✅ | 3 | 7→7→6 | 36→20→0 | 27→23→4 | 4→2→1 | 0→0→5 | 0→5→5 | 0→0→2 | 148→137→69 |
| 11 | [global_read](#global_read) | ✅ / ✅ | 6 | 7→7→0 | 24→20→0 | 25→23→0 | 3→2→0 | 0→0→0 | 0→3→6 | 0→0→2 | 115→115→51 |
| 12 | [global_rwlock](#global_rwlock) | ✅ / ❌ | c2rust | 8→8→6 | 28→22→0 | 22→22→3 | 2→1→1 | 0→0→4 | 0→1→5 | 0→0→2 | 125→116→79 |
| 13 | [global_simple](#global_simple) | ✅ / ❌ | c2rust | 7→7→0 | 24→20→0 | 25→23→0 | 4→2→0 | 0→0→0 | 0→3→3 | 0→0→2 | 124→120→51 |
| 14 | [global_trylock](#global_trylock) | ✅ / ✅ | 1 | 7→7→0 | 38→38→0 | 26→26→0 | 2→2→0 | 0→0→0 | 0→0→3 | 0→0→2 | 153→153→52 |
| 15 | [global_while](#global_while) | ✅ / ✅ | 2 | 8→8→0 | 29→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→2 | 133→133→58 |
| 16 | [struct_alias](#struct_alias) | ✅ / ✅ | 5 | 10→10→9 | 26→20→0 | 32→26→2 | 3→3→0 | 0→0→3 | 0→5→15 | 0→0→2 | 185→156→98 |
| 17 | [struct_assume](#struct_assume) | ✅ / ❌ | c2rust | 8→8→3 | 29→24→0 | 33→33→4 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→1 | 112→129→57 |
| 18 | [struct_condvar](#struct_condvar) | ✅ / ❌ | c2rust | 7→7→8 | 34→26→0 | 28→26→3 | 1→1→1 | 0→0→5 | 0→3→3 | 0→0→2 | 186→164→72 |
| 19 | [struct_dup](#struct_dup) | ✅ / ✅ | 6 | 7→7→6 | 28→20→0 | 28→24→2 | 2→2→0 | 0→0→4 | 0→5→9 | 0→0→2 | 166→147→88 |
| 20 | [struct_empty](#struct_empty) | ✅ / ✅ | 3 | 7→7→5 | 30→24→0 | 34→32→7 | 1→1→1 | 0→0→6 | 0→4→4 | 0→0→1 | 142→133→70 |
| 21 | [struct_init](#struct_init) | ✅ / ❌ | c2rust | 7→7→6 | 35→24→0 | 36→32→6 | 2→2→0 | 0→0→6 | 0→5→10 | 0→0→2 | 160→136→70 |
| 22 | [struct_main](#struct_main) | ✅ / ✅ | 3 | 7→7→7 | 24→20→0 | 25→23→3 | 1→1→1 | 0→0→5 | 0→3→3 | 0→0→1 | 124→120→64 |
| 23 | [struct_malloc](#struct_malloc) | ✅ / ❌ | c2rust | 7→7→3 | 44→34→0 | 39→39→5 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→2 | 163→179→60 |
| 24 | [struct_malloc2](#struct_malloc2) | ✅ / ✅ | 3 | 8→8→5 | 33→24→0 | 32→32→3 | 1→1→1 | 0→0→5 | 0→4→4 | 0→0→2 | 120→135→67 |
| 25 | [struct_multiple](#struct_multiple) | ✅ / ❌ | c2rust | 8→8→0 | 26→20→0 | 30→24→0 | 3→3→0 | 0→0→0 | 0→5→5 | 0→0→2 | 175→138→60 |
| 26 | [struct_nested](#struct_nested) | ✅ / ✅ | 6 | 7→7→6 | 24→20→0 | 25→23→2 | 1→1→1 | 0→0→4 | 0→3→5 | 0→0→1 | 136→129→73 |
| 27 | [struct_simple](#struct_simple) | ✅ / ✅ | 3 | 7→7→8 | 28→20→0 | 27→23→2 | 1→1→1 | 0→0→4 | 0→5→5 | 0→0→2 | 157→138→82 |
| 28 | [struct_spin](#struct_spin) | ✅ / ✅ | 1 | 7→7→3 | 56→40→0 | 36→36→4 | 0→0→0 | 0→0→0 | 0→7→5 | 0→0→2 | 205→197→67 |
| 29 | [struct_timedwait](#struct_timedwait) | ✅ / ❌ | c2rust | 9→9→0 | 42→26→0 | 30→28→0 | 1→1→0 | 0→3→0 | 0→3→5 | 0→0→2 | 251→278→96 |
| 30 | [unused_func](#unused_func) | ✅ / ✅ | 1 | 8→8→0 | 26→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→2 | 119→119→47 |
| | **TOTAL** | 30/30 / 20/30 | — | 231→231→110 | 952→692→0 | 882→778→69 | 58→39→15 | 0→9→73 | 0→113→156 | 0→0→53 | 4693→4483→2057 |

> **Reading the table**: Each metric cell shows **Original → ConCrat → LLM**. Compiles column shows **ConCrat / LLM**.

## All Metrics Summary

This section displays all 15 metrics for each sample in a compact format.

| Example | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_arc | std\_rwlock | std\_condvar | std\_thread | move\_closure | arc\_clone | join\_handle | arc\_mutex\_combo | lines |
|---------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| array_const | 7→7→3 | 42→20→0 | 41→23→5 | 2→1→0 | 0→0→0 | 0→5→3 | 0→0→3 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 183→132→66 |
| array_main | 7→7→4 | 32→20→0 | 37→23→3 | 2→1→2 | 0→0→5 | 0→7→7 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→0 | 211→151→70 |
| array_simple | 7→7→4 | 32→20→0 | 39→25→2 | 4→2→4 | 0→0→3 | 0→7→7 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→0 | 0→0→0 | 0→0→1 | 0→0→0 | 235→197→79 |
| global_assume | 8→8→8 | 24→20→0 | 25→23→3 | 2→1→1 | 0→0→5 | 0→3→3 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→0 | 117→118→49 |
| global_assume2 | 8→8→3 | 24→20→0 | 25→23→3 | 2→1→0 | 0→0→4 | 0→3→3 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→0 | 122→129→59 |
| global_check | 10→10→7 | 44→22→0 | 26→24→3 | 2→1→0 | 0→6→5 | 0→3→10 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→5 | 181→243→121 |
| global_condvar | 7→7→0 | 40→28→0 | 29→27→0 | 4→2→0 | 0→0→0 | 0→3→7 | 0→0→6 | 0→1→0 | 0→3→3 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 188→172→61 |
| global_custom | 12→12→0 | 26→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→6 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→4 | 0→0→1 | 0→0→0 | 142→156→75 |
| global_main | 7→7→0 | 24→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 0→0→0 | 115→113→46 |
| global_nested | 7→7→6 | 36→20→0 | 27→23→4 | 4→2→1 | 0→0→5 | 0→5→5 | 0→0→4 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→1 | 0→0→2 | 0→0→0 | 148→137→69 |
| global_read | 7→7→0 | 24→20→0 | 25→23→0 | 3→2→0 | 0→0→0 | 0→3→6 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 115→115→51 |
| global_rwlock | 8→8→6 | 28→22→0 | 22→22→3 | 2→1→1 | 0→0→4 | 0→1→5 | 0→0→2 | 0→3→5 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→1 | 0→0→2 | 0→0→0 | 125→116→79 |
| global_simple | 7→7→0 | 24→20→0 | 25→23→0 | 4→2→0 | 0→0→0 | 0→3→3 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 124→120→51 |
| global_trylock | 7→7→0 | 38→38→0 | 26→26→0 | 2→2→0 | 0→0→0 | 0→0→3 | 0→0→6 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 153→153→52 |
| global_while | 8→8→0 | 29→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→0 | 133→133→58 |
| struct_alias | 10→10→9 | 26→20→0 | 32→26→2 | 3→3→0 | 0→0→3 | 0→5→15 | 0→0→11 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→6 | 0→0→2 | 0→0→7 | 185→156→98 |
| struct_assume | 8→8→3 | 29→24→0 | 33→33→4 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→4 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 112→129→57 |
| struct_condvar | 7→7→8 | 34→26→0 | 28→26→3 | 1→1→1 | 0→0→5 | 0→3→3 | 0→0→3 | 0→1→0 | 0→3→3 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→0 | 186→164→72 |
| struct_dup | 7→7→6 | 28→20→0 | 28→24→2 | 2→2→0 | 0→0→4 | 0→5→9 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→2 | 166→147→88 |
| struct_empty | 7→7→5 | 30→24→0 | 34→32→7 | 1→1→1 | 0→0→6 | 0→4→4 | 0→0→4 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 142→133→70 |
| struct_init | 7→7→6 | 35→24→0 | 36→32→6 | 2→2→0 | 0→0→6 | 0→5→10 | 0→0→11 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 160→136→70 |
| struct_main | 7→7→7 | 24→20→0 | 25→23→3 | 1→1→1 | 0→0→5 | 0→3→3 | 0→0→4 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 124→120→64 |
| struct_malloc | 7→7→3 | 44→34→0 | 39→39→5 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→7 | 0→1→0 | 0→3→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 163→179→60 |
| struct_malloc2 | 8→8→5 | 33→24→0 | 32→32→3 | 1→1→1 | 0→0→5 | 0→4→4 | 0→0→4 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→2 | 0→0→2 | 0→0→0 | 120→135→67 |
| struct_multiple | 8→8→0 | 26→20→0 | 30→24→0 | 3→3→0 | 0→0→0 | 0→5→5 | 0→0→8 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→0 | 175→138→60 |
| struct_nested | 7→7→6 | 24→20→0 | 25→23→2 | 1→1→1 | 0→0→4 | 0→3→5 | 0→0→4 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 136→129→73 |
| struct_simple | 7→7→8 | 28→20→0 | 27→23→2 | 1→1→1 | 0→0→4 | 0→5→5 | 0→0→4 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→1 | 0→0→2 | 0→0→0 | 157→138→82 |
| struct_spin | 7→7→3 | 56→40→0 | 36→36→4 | 0→0→0 | 0→0→0 | 0→7→5 | 0→0→6 | 0→4→3 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 205→197→67 |
| struct_timedwait | 9→9→0 | 42→26→0 | 30→28→0 | 1→1→0 | 0→3→0 | 0→3→5 | 0→0→5 | 0→1→0 | 0→3→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→1 | 251→278→96 |
| unused_func | 8→8→0 | 26→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 119→119→47 |
| **TOTAL** | 231→231→110 | 952→692→0 | 882→778→69 | 58→39→15 | 0→9→73 | 0→113→156 | 0→0→149 | 0→34→8 | 0→37→10 | 0→0→53 | 0→0→40 | 0→0→49 | 0→0→51 | 0→0→19 | 4693→4483→2057 |

> **All Metrics** table shows all 15 metrics (including std\_arc, std\_rwlock, std\_condvar, move\_closure, arc\_clone, join\_handle, arc\_mutex\_combo) for each sample. Format: **Original → ConCrat → LLM** (or **Original → LLM** for negative samples).

## Aggregate Statistics

| Metric | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|--------|----------|---------|-----|:----------------:|:----------------:|
| unsafe | 231 | 231 | 110 | 0.0% | 52.4% |
| pthread | 952 | 692 | 0 | 27.3% | 100.0% |
| raw\_ptr | 882 | 778 | 69 | 11.8% | 92.2% |
| static\_mut | 58 | 39 | 15 | 32.8% | 74.1% |
| libc | 0 | 9 | 73 | — | — |
| std\_mutex | 0 | 113 | 156 | — | — |
| std\_arc | 0 | 0 | 149 | — | — |
| std\_rwlock | 0 | 34 | 8 | — | — |
| std\_condvar | 0 | 37 | 10 | — | — |
| std\_thread | 0 | 0 | 53 | — | — |
| move\_closure | 0 | 0 | 40 | — | — |
| arc\_clone | 0 | 0 | 49 | — | — |
| join\_handle | 0 | 0 | 51 | — | — |
| arc\_mutex\_combo | 0 | 0 | 19 | — | — |
| lines | 4693 | 4483 | 2057 | 4.5% | 56.2% |

| **Compile success** | — | 30/30 (100%) | 20/30 (67%) | | |

## Metric Categories Summary

Aggregate of metrics grouped by category (Lower is Better vs Higher is Better):

| Category | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|----------|----------|---------|-----|:----------------:|:----------------:|
| **Lower is Better** (∑unsafe, pthread, raw_ptr, static_mut, libc, lines) | 6816 | 6232 | 2324 | 8.6% | 65.9% |
| **Higher is Better** (∑std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 184 | 535 | — | — |

## Safety Features Adoption

| Example | Round | std::sync::Mutex | Arc<Mutex> | RwLock | Condvar | std::thread | join() |
|---------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| array_const | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| array_main | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| array_simple | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_assume | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_assume2 | c2rust | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_check | 11 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_condvar | 2 | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| global_custom | 2 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_main | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,· |
| global_nested | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_read | 6 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_rwlock | c2rust | ·,L | ·,· | C,L | C,· | ·,L | ·,L |
| global_simple | c2rust | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_trylock | 1 | ·,L | ·,· | ·,· | ·,· | ·,L | ·,L |
| global_while | 2 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_alias | 5 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_assume | c2rust | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_condvar | c2rust | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| struct_dup | 6 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_empty | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_init | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_main | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_malloc | c2rust | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| struct_malloc2 | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_multiple | c2rust | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_nested | 6 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_simple | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_spin | 1 | ·,L | ·,· | C,L | C,· | ·,L | ·,L |
| struct_timedwait | c2rust | ·,L | ·,L | C,· | C,L | ·,L | ·,L |
| unused_func | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |

> **C** = ConCrat uses it, **L** = LLM uses it, **·** = not used

## Per-Example Details

### array_const

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 3 | LLM |
| pthread | 42 | 20 | 0 | LLM |
| raw\_ptr | 41 | 23 | 5 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 5 | 3 | ConCrat |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 183 | 132 | 66 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 275 | 183 | 74 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 10 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### array_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 32 | 20 | 0 | LLM |
| raw\_ptr | 37 | 23 | 3 | LLM |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 7 | 7 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 211 | 151 | 70 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 289 | 202 | 84 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 9 | 12 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, num_mutex; 4 unsafe keyword(s) remain

---

### array_simple

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 32 | 20 | 0 | LLM |
| raw\_ptr | 39 | 25 | 2 | LLM |
| static\_mut | 4 | 2 | 4 | ConCrat |
| libc | 0 | 0 | 3 | ConCrat |
| std\_mutex | 0 | 7 | 7 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 235 | 197 | 79 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 317 | 251 | 92 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 9 | 10 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, n2, n3, num_mutex; 4 unsafe keyword(s) remain

---

### global_assume

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 8 | tie |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 3 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 117 | 118 | 49 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 176 | 170 | 66 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 8 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 8 unsafe keyword(s) remain

---

### global_assume2

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 3 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 3 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 122 | 129 | 59 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 181 | 181 | 69 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 8 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### global_check

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 10 | 10 | 7 | LLM |
| pthread | 44 | 22 | 0 | LLM |
| raw\_ptr | 26 | 24 | 3 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 0 | 6 | 5 | LLM |
| std\_mutex | 0 | 3 | 10 | LLM |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 3 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 5 | LLM |
| lines | 181 | 243 | 121 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 263 | 306 | 136 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 31 |

**Remaining Issues:**

- **LLM**: 7 unsafe keyword(s) remain

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
| unsafe | 12 | 12 | 0 | LLM |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 0 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 6 | LLM |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 4 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 142 | 156 | 75 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 207 | 212 | 75 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 20 |

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
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 115 | 113 | 46 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 173 | 164 | 46 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 15 |

**Remaining Issues:**

- **LLM**: thread::spawn without join (detached thread)

---

### global_nested

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 6 | LLM |
| pthread | 36 | 20 | 0 | LLM |
| raw\_ptr | 27 | 23 | 4 | LLM |
| static\_mut | 4 | 2 | 1 | LLM |
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
| lines | 148 | 137 | 69 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 222 | 189 | 85 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 16 |

**Remaining Issues:**

- **LLM**: static mut variables remain: SHARED_DATA; 6 unsafe keyword(s) remain

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
| std\_mutex | 0 | 3 | 6 | LLM |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 2 | LLM |
| lines | 115 | 115 | 51 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 174 | 167 | 51 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 22 |

---

### global_rwlock

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 6 | LLM |
| pthread | 28 | 22 | 0 | LLM |
| raw\_ptr | 22 | 22 | 3 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 1 | 5 | LLM |
| std\_arc | 0 | 0 | 2 | LLM |
| std\_rwlock | 0 | 3 | 5 | LLM |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 125 | 116 | 79 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 169 | 93 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 19 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N; 6 unsafe keyword(s) remain

---

### global_simple

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 0 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 0 | LLM |
| static\_mut | 4 | 2 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 124 | 120 | 51 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 184 | 172 | 51 |
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
| std\_mutex | 0 | 0 | 3 | LLM |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 0 | 0 | tie |
| std\_condvar | 0 | 0 | 0 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 153 | 153 | 52 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 226 | 226 | 52 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 | 17 |

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

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 10 | 10 | 9 | LLM |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 32 | 26 | 2 | LLM |
| static\_mut | 3 | 3 | 0 | LLM |
| libc | 0 | 0 | 3 | ConCrat |
| std\_mutex | 0 | 5 | 15 | LLM |
| std\_arc | 0 | 0 | 11 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 6 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 7 | LLM |
| lines | 185 | 156 | 98 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 256 | 215 | 112 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 45 |

**Remaining Issues:**

- **LLM**: 9 unsafe keyword(s) remain

---

### struct_assume

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 3 | LLM |
| pthread | 29 | 24 | 0 | LLM |
| raw\_ptr | 33 | 33 | 4 | LLM |
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
| lines | 112 | 129 | 57 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 182 | 194 | 64 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 11 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### struct_condvar

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 8 | ConCrat |
| pthread | 34 | 26 | 0 | LLM |
| raw\_ptr | 28 | 26 | 3 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 3 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 186 | 164 | 72 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 256 | 224 | 89 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 18 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 8 unsafe keyword(s) remain

---

### struct_dup

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 6 | LLM |
| pthread | 28 | 20 | 0 | LLM |
| raw\_ptr | 28 | 24 | 2 | LLM |
| static\_mut | 2 | 2 | 0 | LLM |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 5 | 9 | LLM |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 2 | LLM |
| lines | 166 | 147 | 88 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 231 | 200 | 100 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 20 |

**Remaining Issues:**

- **LLM**: 6 unsafe keyword(s) remain

---

### struct_empty

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 5 | LLM |
| pthread | 30 | 24 | 0 | LLM |
| raw\_ptr | 34 | 32 | 7 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 6 | ConCrat |
| std\_mutex | 0 | 4 | 4 | tie |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 142 | 133 | 70 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 214 | 197 | 89 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 6 | 12 |

**Remaining Issues:**

- **LLM**: static mut variables remain: s1; 5 unsafe keyword(s) remain

---

### struct_init

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 6 | LLM |
| pthread | 35 | 24 | 0 | LLM |
| raw\_ptr | 36 | 32 | 6 | LLM |
| static\_mut | 2 | 2 | 0 | LLM |
| libc | 0 | 0 | 6 | ConCrat |
| std\_mutex | 0 | 5 | 10 | LLM |
| std\_arc | 0 | 0 | 11 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 2 | LLM |
| lines | 160 | 136 | 70 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 240 | 201 | 88 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 31 |

**Remaining Issues:**

- **LLM**: 6 unsafe keyword(s) remain

---

### struct_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 7 | tie |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 3 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 124 | 120 | 64 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 181 | 171 | 80 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 11 |

**Remaining Issues:**

- **LLM**: static mut variables remain: SHARED_DATA; 7 unsafe keyword(s) remain

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
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 2 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 163 | 179 | 60 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 253 | 259 | 68 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 20 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### struct_malloc2

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 5 | LLM |
| pthread | 33 | 24 | 0 | LLM |
| raw\_ptr | 32 | 32 | 3 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 4 | 4 | tie |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 120 | 135 | 67 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 194 | 200 | 81 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 6 | 14 |

**Remaining Issues:**

- **LLM**: static mut variables remain: X; 5 unsafe keyword(s) remain

---

### struct_multiple

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 0 | LLM |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 30 | 24 | 0 | LLM |
| static\_mut | 3 | 3 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 8 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 175 | 138 | 60 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 242 | 193 | 60 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 17 |

---

### struct_nested

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 6 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 2 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 3 | 5 | LLM |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 136 | 129 | 73 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 193 | 180 | 86 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 13 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 6 unsafe keyword(s) remain

---

### struct_simple

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 8 | ConCrat |
| pthread | 28 | 20 | 0 | LLM |
| raw\_ptr | 27 | 23 | 2 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 157 | 138 | 82 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 220 | 189 | 97 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 16 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 8 unsafe keyword(s) remain

---

### struct_spin

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 3 | LLM |
| pthread | 56 | 40 | 0 | LLM |
| raw\_ptr | 36 | 36 | 4 | LLM |
| static\_mut | 0 | 0 | 0 | tie |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 7 | 5 | ConCrat |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 4 | 3 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 205 | 197 | 67 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 304 | 280 | 74 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 12 | 22 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

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
| std\_mutex | 0 | 3 | 5 | LLM |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 2 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 251 | 278 | 96 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 333 | 345 | 96 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 21 |

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
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 119 | 119 | 47 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 180 | 171 | 47 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 18 |

---
