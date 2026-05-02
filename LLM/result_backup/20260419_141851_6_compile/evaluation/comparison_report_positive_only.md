# Concurrency Transformation Comparison Report (Positive Samples Only)

Comparing **Original** vs **ConCrat** vs **LLM** for positive examples

## Summary Overview

| # | Example | Compiles (C / L) | Round | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_thread | lines |
|---|---------|:----------------:|:---:|------|------|------|------|------|------|------|------|
| 1 | [array_const](#array_const) | ✅ / ✅ | 2 | 7→7→0 | 42→20→0 | 41→23→0 | 2→1→0 | 0→0→0 | 0→5→9 | 0→0→1 | 183→132→60 |
| 2 | [array_main](#array_main) | ✅ / ❌ | c2rust | 7→7→4 | 32→20→0 | 37→23→2 | 2→1→2 | 0→0→4 | 0→7→7 | 0→0→1 | 211→151→67 |
| 3 | [array_simple](#array_simple) | ✅ / ❌ | c2rust | 7→7→4 | 32→20→0 | 39→25→2 | 4→2→4 | 0→0→4 | 0→7→7 | 0→0→1 | 235→197→72 |
| 4 | [global_assume](#global_assume) | ✅ / ✅ | 1 | 8→8→8 | 24→20→0 | 25→23→3 | 2→1→1 | 0→0→5 | 0→3→3 | 0→0→2 | 117→118→49 |
| 5 | [global_assume2](#global_assume2) | ✅ / ✅ | 1 | 8→8→0 | 24→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→1 | 122→129→53 |
| 6 | [global_check](#global_check) | ✅ / ❌ | c2rust | 10→10→8 | 44→22→0 | 26→24→2 | 2→1→1 | 0→6→4 | 0→3→5 | 0→0→2 | 181→243→131 |
| 7 | [global_condvar](#global_condvar) | ✅ / ✅ | 2 | 7→7→0 | 40→28→0 | 29→27→0 | 4→2→0 | 0→0→0 | 0→3→7 | 0→0→2 | 188→172→56 |
| 8 | [global_custom](#global_custom) | ✅ / ❌ | c2rust | 12→12→3 | 26→20→0 | 25→23→3 | 2→1→0 | 0→0→4 | 0→3→6 | 0→0→2 | 142→156→105 |
| 9 | [global_main](#global_main) | ✅ / ✅ | 1 | 7→7→3 | 24→20→0 | 25→23→5 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→2 | 115→113→51 |
| 10 | [global_nested](#global_nested) | ✅ / ❌ | c2rust | 7→7→4 | 36→20→0 | 27→23→2 | 4→2→0 | 0→0→4 | 0→5→5 | 0→0→2 | 148→137→65 |
| 11 | [global_read](#global_read) | ✅ / ✅ | 6 | 7→7→3 | 24→20→0 | 25→23→4 | 3→2→0 | 0→0→0 | 0→3→6 | 0→0→1 | 115→115→53 |
| 12 | [global_rwlock](#global_rwlock) | ✅ / ❌ | c2rust | 8→8→7 | 28→22→0 | 22→22→3 | 2→1→2 | 0→0→0 | 0→1→3 | 0→0→2 | 125→116→65 |
| 13 | [global_simple](#global_simple) | ✅ / ✅ | 7 | 7→7→0 | 24→20→0 | 25→23→0 | 4→2→0 | 0→0→0 | 0→3→6 | 0→0→1 | 124→120→56 |
| 14 | [global_trylock](#global_trylock) | ✅ / ✅ | 1 | 7→7→0 | 38→38→0 | 26→26→0 | 2→2→0 | 0→0→0 | 0→0→3 | 0→0→2 | 153→153→54 |
| 15 | [global_while](#global_while) | ✅ / ❌ | c2rust | 8→8→5 | 29→20→6 | 25→23→2 | 2→1→1 | 0→0→10 | 0→3→4 | 0→0→0 | 133→133→88 |
| 16 | [struct_alias](#struct_alias) | ✅ / ❌ | c2rust | 10→10→8 | 26→20→0 | 32→26→3 | 3→3→0 | 0→0→2 | 0→5→5 | 0→0→2 | 185→156→85 |
| 17 | [struct_assume](#struct_assume) | ✅ / ✅ | 1 | 8→8→5 | 29→24→0 | 33→33→6 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→2 | 112→129→62 |
| 18 | [struct_condvar](#struct_condvar) | ✅ / ✅ | 3 | 7→7→7 | 34→26→0 | 28→26→3 | 1→1→1 | 0→0→5 | 0→3→3 | 0→0→2 | 186→164→69 |
| 19 | [struct_dup](#struct_dup) | ✅ / ✅ | 3 | 7→7→9 | 28→20→0 | 28→24→2 | 2→2→2 | 0→0→5 | 0→5→5 | 0→0→2 | 166→147→88 |
| 20 | [struct_empty](#struct_empty) | ✅ / ❌ | c2rust | 7→7→4 | 30→24→0 | 34→32→5 | 1→1→0 | 0→0→6 | 0→4→7 | 0→0→2 | 142→133→63 |
| 21 | [struct_init](#struct_init) | ✅ / ❌ | c2rust | 7→7→4 | 35→24→0 | 36→32→3 | 2→2→0 | 0→0→0 | 0→5→5 | 0→0→2 | 160→136→60 |
| 22 | [struct_main](#struct_main) | ✅ / ✅ | 5 | 7→7→5 | 24→20→0 | 25→23→2 | 1→1→0 | 0→0→4 | 0→3→5 | 0→0→1 | 124→120→59 |
| 23 | [struct_malloc](#struct_malloc) | ✅ / ❌ | c2rust | 7→7→3 | 44→34→0 | 39→39→5 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→2 | 163→179→59 |
| 24 | [struct_malloc2](#struct_malloc2) | ✅ / ❌ | c2rust | 8→8→5 | 33→24→0 | 32→32→3 | 1→1→1 | 0→0→4 | 0→4→4 | 0→0→2 | 120→135→69 |
| 25 | [struct_multiple](#struct_multiple) | ✅ / ❌ | c2rust | 8→8→8 | 26→20→0 | 30→24→3 | 3→3→3 | 0→0→5 | 0→5→5 | 0→0→2 | 175→138→83 |
| 26 | [struct_nested](#struct_nested) | ✅ / ✅ | 6 | 7→7→2 | 24→20→0 | 25→23→1 | 1→1→0 | 0→0→3 | 0→3→7 | 0→0→2 | 136→129→61 |
| 27 | [struct_simple](#struct_simple) | ✅ / ❌ | c2rust | 7→7→4 | 28→20→0 | 27→23→5 | 1→1→0 | 0→0→5 | 0→5→11 | 0→0→2 | 157→138→68 |
| 28 | [struct_spin](#struct_spin) | ✅ / ❌ | c2rust | 7→7→3 | 56→40→0 | 36→36→5 | 0→0→0 | 0→0→0 | 0→7→5 | 0→0→2 | 205→197→62 |
| 29 | [struct_timedwait](#struct_timedwait) | ✅ / ❌ | c2rust | 9→9→2 | 42→26→0 | 30→28→0 | 1→1→1 | 0→3→0 | 0→3→10 | 0→0→2 | 251→278→95 |
| 30 | [unused_func](#unused_func) | ✅ / ✅ | 1 | 8→8→6 | 26→20→0 | 25→23→3 | 2→1→1 | 0→0→5 | 0→3→4 | 0→0→1 | 119→119→66 |
| | **TOTAL** | 30/30 / 14/30 | — | 231→231→124 | 952→692→6 | 882→778→77 | 58→39→20 | 0→9→79 | 0→113→159 | 0→0→50 | 4693→4483→2074 |

> **Reading the table**: Each metric cell shows **Original → ConCrat → LLM**. Compiles column shows **ConCrat / LLM**.

## All Metrics Summary

This section displays all 15 metrics for each sample in a compact format.

| Example | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_arc | std\_rwlock | std\_condvar | std\_thread | move\_closure | arc\_clone | join\_handle | arc\_mutex\_combo | lines |
|---------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| array_const | 7→7→0 | 42→20→0 | 41→23→0 | 2→1→0 | 0→0→0 | 0→5→9 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 183→132→60 |
| array_main | 7→7→4 | 32→20→0 | 37→23→2 | 2→1→2 | 0→0→4 | 0→7→7 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→0 | 0→0→0 | 0→0→1 | 0→0→1 | 211→151→67 |
| array_simple | 7→7→4 | 32→20→0 | 39→25→2 | 4→2→4 | 0→0→4 | 0→7→7 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→0 | 0→0→0 | 0→0→1 | 0→0→1 | 235→197→72 |
| global_assume | 8→8→8 | 24→20→0 | 25→23→3 | 2→1→1 | 0→0→5 | 0→3→3 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→0 | 117→118→49 |
| global_assume2 | 8→8→0 | 24→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 122→129→53 |
| global_check | 10→10→8 | 44→22→0 | 26→24→2 | 2→1→1 | 0→6→4 | 0→3→5 | 0→0→3 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→4 | 0→0→2 | 0→0→0 | 181→243→131 |
| global_condvar | 7→7→0 | 40→28→0 | 29→27→0 | 4→2→0 | 0→0→0 | 0→3→7 | 0→0→5 | 0→1→0 | 0→3→3 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 188→172→56 |
| global_custom | 12→12→3 | 26→20→0 | 25→23→3 | 2→1→0 | 0→0→4 | 0→3→6 | 0→0→3 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→1 | 142→156→105 |
| global_main | 7→7→3 | 24→20→0 | 25→23→5 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→8 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 115→113→51 |
| global_nested | 7→7→4 | 36→20→0 | 27→23→2 | 4→2→0 | 0→0→4 | 0→5→5 | 0→0→3 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 148→137→65 |
| global_read | 7→7→3 | 24→20→0 | 25→23→4 | 3→2→0 | 0→0→0 | 0→3→6 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 115→115→53 |
| global_rwlock | 8→8→7 | 28→22→0 | 22→22→3 | 2→1→2 | 0→0→0 | 0→1→3 | 0→0→5 | 0→3→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→1 | 125→116→65 |
| global_simple | 7→7→0 | 24→20→0 | 25→23→0 | 4→2→0 | 0→0→0 | 0→3→6 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→2 | 124→120→56 |
| global_trylock | 7→7→0 | 38→38→0 | 26→26→0 | 2→2→0 | 0→0→0 | 0→0→3 | 0→0→6 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 153→153→54 |
| global_while | 8→8→5 | 29→20→6 | 25→23→2 | 2→1→1 | 0→0→10 | 0→3→4 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 133→133→88 |
| struct_alias | 10→10→8 | 26→20→0 | 32→26→3 | 3→3→0 | 0→0→2 | 0→5→5 | 0→0→9 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→0 | 185→156→85 |
| struct_assume | 8→8→5 | 29→24→0 | 33→33→6 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 112→129→62 |
| struct_condvar | 7→7→7 | 34→26→0 | 28→26→3 | 1→1→1 | 0→0→5 | 0→3→3 | 0→0→4 | 0→1→0 | 0→3→3 | 0→0→2 | 0→0→2 | 0→0→1 | 0→0→2 | 0→0→0 | 186→164→69 |
| struct_dup | 7→7→9 | 28→20→0 | 28→24→2 | 2→2→2 | 0→0→5 | 0→5→5 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 166→147→88 |
| struct_empty | 7→7→4 | 30→24→0 | 34→32→5 | 1→1→0 | 0→0→6 | 0→4→7 | 0→0→9 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→1 | 142→133→63 |
| struct_init | 7→7→4 | 35→24→0 | 36→32→3 | 2→2→0 | 0→0→0 | 0→5→5 | 0→0→12 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 160→136→60 |
| struct_main | 7→7→5 | 24→20→0 | 25→23→2 | 1→1→0 | 0→0→4 | 0→3→5 | 0→0→3 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 124→120→59 |
| struct_malloc | 7→7→3 | 44→34→0 | 39→39→5 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→7 | 0→1→0 | 0→3→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 163→179→59 |
| struct_malloc2 | 8→8→5 | 33→24→0 | 32→32→3 | 1→1→1 | 0→0→4 | 0→4→4 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→0 | 120→135→69 |
| struct_multiple | 8→8→8 | 26→20→0 | 30→24→3 | 3→3→3 | 0→0→5 | 0→5→5 | 0→0→8 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→0 | 175→138→83 |
| struct_nested | 7→7→2 | 24→20→0 | 25→23→1 | 1→1→0 | 0→0→3 | 0→3→7 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→3 | 136→129→61 |
| struct_simple | 7→7→4 | 28→20→0 | 27→23→5 | 1→1→0 | 0→0→5 | 0→5→11 | 0→0→9 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 157→138→68 |
| struct_spin | 7→7→3 | 56→40→0 | 36→36→5 | 0→0→0 | 0→0→0 | 0→7→5 | 0→0→9 | 0→4→3 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 205→197→62 |
| struct_timedwait | 9→9→2 | 42→26→0 | 30→28→0 | 1→1→1 | 0→3→0 | 0→3→10 | 0→0→8 | 0→1→0 | 0→3→3 | 0→0→2 | 0→0→2 | 0→0→5 | 0→0→2 | 0→0→6 | 251→278→95 |
| unused_func | 8→8→6 | 26→20→0 | 25→23→3 | 2→1→1 | 0→0→5 | 0→3→4 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→0 | 0→0→0 | 0→0→1 | 0→0→0 | 119→119→66 |
| **TOTAL** | 231→231→124 | 952→692→6 | 882→778→77 | 58→39→20 | 0→9→79 | 0→113→159 | 0→0→169 | 0→34→3 | 0→37→11 | 0→0→50 | 0→0→41 | 0→0→47 | 0→0→50 | 0→0→20 | 4693→4483→2074 |

> **All Metrics** table shows all 15 metrics (including std\_arc, std\_rwlock, std\_condvar, move\_closure, arc\_clone, join\_handle, arc\_mutex\_combo) for each sample. Format: **Original → ConCrat → LLM** (or **Original → LLM** for negative samples).

## Aggregate Statistics

| Metric | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|--------|----------|---------|-----|:----------------:|:----------------:|
| unsafe | 231 | 231 | 124 | 0.0% | 46.3% |
| pthread | 952 | 692 | 6 | 27.3% | 99.4% |
| raw\_ptr | 882 | 778 | 77 | 11.8% | 91.3% |
| static\_mut | 58 | 39 | 20 | 32.8% | 65.5% |
| libc | 0 | 9 | 79 | — | — |
| std\_mutex | 0 | 113 | 159 | — | — |
| std\_arc | 0 | 0 | 169 | — | — |
| std\_rwlock | 0 | 34 | 3 | — | — |
| std\_condvar | 0 | 37 | 11 | — | — |
| std\_thread | 0 | 0 | 50 | — | — |
| move\_closure | 0 | 0 | 41 | — | — |
| arc\_clone | 0 | 0 | 47 | — | — |
| join\_handle | 0 | 0 | 50 | — | — |
| arc\_mutex\_combo | 0 | 0 | 20 | — | — |
| lines | 4693 | 4483 | 2074 | 4.5% | 55.8% |

| **Compile success** | — | 30/30 (100%) | 14/30 (47%) | | |

## Metric Categories Summary

Aggregate of metrics grouped by category (Lower is Better vs Higher is Better):

| Category | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|----------|----------|---------|-----|:----------------:|:----------------:|
| **Lower is Better** (∑unsafe, pthread, raw_ptr, static_mut, libc, lines) | 6816 | 6232 | 2380 | 8.6% | 65.1% |
| **Higher is Better** (∑std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 184 | 550 | — | — |

## Safety Features Adoption

| Example | Round | std::sync::Mutex | Arc<Mutex> | RwLock | Condvar | std::thread | join() |
|---------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| array_const | 2 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| array_main | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| array_simple | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_assume | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_assume2 | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_check | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_condvar | 2 | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| global_custom | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_main | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_nested | c2rust | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_read | 6 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_rwlock | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_simple | 7 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_trylock | 1 | ·,L | ·,· | ·,· | ·,· | ·,L | ·,L |
| global_while | c2rust | ·,L | ·,· | C,· | C,· | ·,· | ·,· |
| struct_alias | c2rust | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_assume | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_condvar | 3 | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| struct_dup | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_empty | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_init | c2rust | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_main | 5 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_malloc | c2rust | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| struct_malloc2 | c2rust | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_multiple | c2rust | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_nested | 6 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_simple | c2rust | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_spin | c2rust | ·,L | ·,L | C,L | C,· | ·,L | ·,L |
| struct_timedwait | c2rust | ·,L | ·,L | C,· | C,L | ·,L | ·,L |
| unused_func | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |

> **C** = ConCrat uses it, **L** = LLM uses it, **·** = not used

## Per-Example Details

### array_const

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 0 | LLM |
| pthread | 42 | 20 | 0 | LLM |
| raw\_ptr | 41 | 23 | 0 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 5 | 9 | LLM |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 183 | 132 | 60 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 275 | 183 | 60 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 18 |

---

### array_main

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 32 | 20 | 0 | LLM |
| raw\_ptr | 37 | 23 | 2 | LLM |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 7 | 7 | tie |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 211 | 151 | 67 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 289 | 202 | 79 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 9 | 17 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, num_mutex; 4 unsafe keyword(s) remain

---

### array_simple

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 32 | 20 | 0 | LLM |
| raw\_ptr | 39 | 25 | 2 | LLM |
| static\_mut | 4 | 2 | 4 | ConCrat |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 7 | 7 | tie |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 235 | 197 | 72 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 317 | 251 | 86 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 9 | 17 |

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
| lines | 122 | 129 | 53 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 181 | 181 | 53 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 13 |

---

### global_check

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 10 | 10 | 8 | LLM |
| pthread | 44 | 22 | 0 | LLM |
| raw\_ptr | 26 | 24 | 2 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 0 | 6 | 4 | LLM |
| std\_mutex | 0 | 3 | 5 | LLM |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 4 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 181 | 243 | 131 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 263 | 306 | 146 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 18 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N; 8 unsafe keyword(s) remain

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
| lines | 188 | 172 | 56 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 268 | 236 | 56 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 23 |

---

### global_custom

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 12 | 12 | 3 | LLM |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 3 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 3 | 6 | LLM |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 142 | 156 | 105 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 207 | 212 | 115 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 14 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### global_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 3 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 5 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 8 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 115 | 113 | 51 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 173 | 164 | 59 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 19 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### global_nested

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 36 | 20 | 0 | LLM |
| raw\_ptr | 27 | 23 | 2 | LLM |
| static\_mut | 4 | 2 | 0 | LLM |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 148 | 137 | 65 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 222 | 189 | 75 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 16 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### global_read

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 3 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 4 | LLM |
| static\_mut | 3 | 2 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 6 | LLM |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 115 | 115 | 53 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 174 | 167 | 60 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 17 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### global_rwlock

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 7 | LLM |
| pthread | 28 | 22 | 0 | LLM |
| raw\_ptr | 22 | 22 | 3 | LLM |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 1 | 3 | LLM |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 3 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 125 | 116 | 65 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 169 | 77 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 17 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N, LOCK; 7 unsafe keyword(s) remain

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
| std\_mutex | 0 | 0 | 3 | LLM |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 0 | 0 | tie |
| std\_condvar | 0 | 0 | 0 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 153 | 153 | 54 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 226 | 226 | 54 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 0 | 17 |

---

### global_while

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 5 | LLM |
| pthread | 29 | 20 | 6 | LLM |
| raw\_ptr | 25 | 23 | 2 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 0 | 0 | 10 | ConCrat |
| std\_mutex | 0 | 3 | 4 | LLM |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 133 | 133 | 88 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 197 | 185 | 112 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 5 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 5 unsafe keyword(s) remain

---

### struct_alias

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 10 | 10 | 8 | LLM |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 32 | 26 | 3 | LLM |
| static\_mut | 3 | 3 | 0 | LLM |
| libc | 0 | 0 | 2 | ConCrat |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 9 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 3 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 185 | 156 | 85 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 256 | 215 | 98 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 23 |

**Remaining Issues:**

- **LLM**: 8 unsafe keyword(s) remain

---

### struct_assume

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 5 | LLM |
| pthread | 29 | 24 | 0 | LLM |
| raw\_ptr | 33 | 33 | 6 | LLM |
| static\_mut | 0 | 0 | 0 | tie |
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
| lines | 112 | 129 | 62 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 182 | 194 | 73 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 17 |

**Remaining Issues:**

- **LLM**: 5 unsafe keyword(s) remain

---

### struct_condvar

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 7 | tie |
| pthread | 34 | 26 | 0 | LLM |
| raw\_ptr | 28 | 26 | 3 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 186 | 164 | 69 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 256 | 224 | 85 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 17 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 7 unsafe keyword(s) remain

---

### struct_dup

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 9 | ConCrat |
| pthread | 28 | 20 | 0 | LLM |
| raw\_ptr | 28 | 24 | 2 | LLM |
| static\_mut | 2 | 2 | 2 | tie |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 166 | 147 | 88 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 231 | 200 | 106 |
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
| raw\_ptr | 34 | 32 | 5 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 0 | 0 | 6 | ConCrat |
| std\_mutex | 0 | 4 | 7 | LLM |
| std\_arc | 0 | 0 | 9 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 142 | 133 | 63 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 214 | 197 | 78 |
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
| raw\_ptr | 36 | 32 | 3 | LLM |
| static\_mut | 2 | 2 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 12 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 160 | 136 | 60 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 240 | 201 | 67 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 25 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 5 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 2 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 3 | 5 | LLM |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 124 | 120 | 59 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 181 | 171 | 70 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 13 |

**Remaining Issues:**

- **LLM**: 5 unsafe keyword(s) remain

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
| lines | 163 | 179 | 59 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 253 | 259 | 67 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 20 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### struct_malloc2

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 5 | LLM |
| pthread | 33 | 24 | 0 | LLM |
| raw\_ptr | 32 | 32 | 3 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 4 | 4 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 120 | 135 | 69 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 194 | 200 | 82 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 6 | 9 |

**Remaining Issues:**

- **LLM**: static mut variables remain: x; 5 unsafe keyword(s) remain

---

### struct_multiple

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 8 | tie |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 30 | 24 | 3 | LLM |
| static\_mut | 3 | 3 | 3 | tie |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 8 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 3 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 175 | 138 | 83 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 242 | 193 | 102 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 22 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S1, S2, S3; 8 unsafe keyword(s) remain

---

### struct_nested

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 2 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 1 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 0 | 0 | 3 | ConCrat |
| std\_mutex | 0 | 3 | 7 | LLM |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 3 | LLM |
| lines | 136 | 129 | 61 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 193 | 180 | 67 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 23 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

---

### struct_simple

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 28 | 20 | 0 | LLM |
| raw\_ptr | 27 | 23 | 5 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 5 | 11 | LLM |
| std\_arc | 0 | 0 | 9 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 157 | 138 | 68 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 220 | 189 | 82 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 28 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_spin

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 3 | LLM |
| pthread | 56 | 40 | 0 | LLM |
| raw\_ptr | 36 | 36 | 5 | LLM |
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
| lines | 205 | 197 | 62 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 304 | 280 | 70 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 12 | 27 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### struct_timedwait

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 9 | 9 | 2 | LLM |
| pthread | 42 | 26 | 0 | LLM |
| raw\_ptr | 30 | 28 | 0 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 3 | 0 | LLM |
| std\_mutex | 0 | 3 | 10 | LLM |
| std\_arc | 0 | 0 | 8 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 5 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 6 | LLM |
| lines | 251 | 278 | 95 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 333 | 345 | 98 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 38 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 2 unsafe keyword(s) remain

---

### unused_func

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 6 | LLM |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 3 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 3 | 4 | LLM |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 119 | 119 | 66 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 180 | 171 | 81 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 7 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 6 unsafe keyword(s) remain

---
