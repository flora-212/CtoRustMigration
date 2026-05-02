# Concurrency Transformation Comparison Report

Three-way comparison: **Original** (c2rust output) vs **ConCrat** (automated transform) vs **LLM** (LLM-based rewrite)

## Summary Overview

| # | Example | Compiles (C / L) | Round | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_thread |
|---|---------|:----------------:|:---:|------|------|------|------|------|------|------|
| 1 | [array_const](#array_const) | ✅ / ✅ | 2 | 7→7→0 | 42→20→0 | 41→23→0 | 2→1→0 | 0→0→0 | 0→5→5 | 0→0→1 |
| 2 | [array_main](#array_main) | ✅ / ✅ | 2 | 7→7→5 | 32→20→0 | 37→23→3 | 2→1→2 | 0→0→3 | 0→7→7 | 0→0→1 |
| 3 | [array_simple](#array_simple) | ✅ / ❌ | c2rust | 7→7→5 | 32→20→0 | 39→25→2 | 4→2→3 | 0→0→6 | 0→7→7 | 0→0→1 |
| 4 | [global_assume](#global_assume) | ✅ / ✅ | 2 | 8→8→0 | 24→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→5 | 0→0→1 |
| 5 | [global_assume2](#global_assume2) | ✅ / ❌ | c2rust | 8→8→7 | 24→20→15 | 25→23→12 | 2→1→2 | 0→0→0 | 0→3→1 | 0→0→0 |
| 6 | [global_check](#global_check) | ✅ / ❌ | c2rust | 10→10→9 | 44→22→28 | 26→24→8 | 2→1→2 | 0→6→0 | 0→3→1 | 0→0→0 |
| 7 | [global_condvar](#global_condvar) | ✅ / ✅ | 2 | 7→7→0 | 40→28→0 | 29→27→0 | 4→2→0 | 0→0→0 | 0→3→7 | 0→0→2 |
| 8 | [global_custom](#global_custom) | ✅ / ✅ | 12 | 12→12→4 | 26→20→0 | 25→23→3 | 2→1→0 | 0→0→2 | 0→3→5 | 0→0→1 |
| 9 | [global_main](#global_main) | ✅ / ✅ | 1 | 7→7→3 | 24→20→0 | 25→23→3 | 2→1→1 | 0→0→5 | 0→3→3 | 0→0→2 |
| 10 | [global_nested](#global_nested) | ✅ / ✅ | 1 | 7→7→0 | 36→20→0 | 27→23→0 | 4→2→0 | 0→0→0 | 0→5→5 | 0→0→2 |
| 11 | [global_read](#global_read) | ✅ / ✅ | 6 | 7→7→3 | 24→20→0 | 25→23→5 | 3→2→0 | 0→0→0 | 0→3→6 | 0→0→2 |
| 12 | [global_rwlock](#global_rwlock) | ✅ / ❌ | c2rust | 8→8→5 | 28→22→0 | 22→22→3 | 2→1→1 | 0→0→4 | 0→1→5 | 0→0→2 |
| 13 | [global_simple](#global_simple) | ✅ / ✅ | 18 | 7→7→0 | 24→20→0 | 25→23→0 | 4→2→0 | 0→0→0 | 0→3→6 | 0→0→2 |
| 14 | [global_trylock](#global_trylock) | ✅ / ✅ | 1 | 7→7→0 | 38→38→0 | 26→26→0 | 2→2→0 | 0→0→0 | 0→0→3 | 0→0→2 |
| 15 | [global_while](#global_while) | ✅ / ✅ | 3 | 8→8→0 | 29→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→5 | 0→0→2 |
| 16 | [struct_alias](#struct_alias) | ✅ / ❌ | c2rust | 10→10→2 | 26→20→0 | 32→26→3 | 3→3→0 | 0→0→1 | 0→5→12 | 0→0→2 |
| 17 | [struct_assume](#struct_assume) | ✅ / ✅ | 2 | 8→8→7 | 29→24→0 | 33→33→6 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→1 |
| 18 | [struct_condvar](#struct_condvar) | ✅ / ❌ | c2rust | 7→7→7 | 34→26→0 | 28→26→3 | 1→1→1 | 0→0→5 | 0→3→3 | 0→0→2 |
| 19 | [struct_dup](#struct_dup) | ✅ / ✅ | 3 | 7→7→9 | 28→20→0 | 28→24→4 | 2→2→2 | 0→0→7 | 0→5→5 | 0→0→2 |
| 20 | [struct_empty](#struct_empty) | ✅ / ❌ | c2rust | 7→7→6 | 30→24→0 | 34→32→8 | 1→1→0 | 0→0→6 | 0→4→7 | 0→0→1 |
| 21 | [struct_init](#struct_init) | ✅ / ❌ | c2rust | 7→7→4 | 35→24→0 | 36→32→8 | 2→2→0 | 0→0→0 | 0→5→10 | 0→0→2 |
| 22 | [struct_main](#struct_main) | ✅ / ✅ | 6 | 7→7→8 | 24→20→0 | 25→23→3 | 1→1→1 | 0→0→5 | 0→3→3 | 0→0→2 |
| 23 | [struct_malloc](#struct_malloc) | ✅ / ❌ | c2rust | 7→7→3 | 44→34→0 | 39→39→5 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→2 |
| 24 | [struct_malloc2](#struct_malloc2) | ✅ / ❌ | c2rust | 8→8→7 | 33→24→0 | 32→32→2 | 1→1→0 | 0→0→3 | 0→4→6 | 0→0→2 |
| 25 | [struct_multiple](#struct_multiple) | ✅ / ✅ | 3 | 8→8→3 | 26→20→0 | 30→24→3 | 3→3→0 | 0→0→5 | 0→5→5 | 0→0→2 |
| 26 | [struct_nested](#struct_nested) | ✅ / ❌ | c2rust | 7→7→6 | 24→20→0 | 25→23→2 | 1→1→0 | 0→0→4 | 0→3→3 | 0→0→1 |
| 27 | [struct_simple](#struct_simple) | ✅ / ✅ | 13 | 7→7→5 | 28→20→0 | 27→23→3 | 1→1→0 | 0→0→5 | 0→5→9 | 0→0→2 |
| 28 | [struct_spin](#struct_spin) | ✅ / ❌ | c2rust | 7→7→1 | 56→40→0 | 36→36→3 | 0→0→0 | 0→0→0 | 0→7→5 | 0→0→2 |
| 29 | [struct_timedwait](#struct_timedwait) | ✅ / ✅ | 6 | 9→9→0 | 42→26→0 | 30→28→0 | 1→1→0 | 0→3→0 | 0→3→9 | 0→0→2 |
| 30 | [unused_func](#unused_func) | ✅ / ✅ | 2 | 8→8→0 | 26→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→2 |
| 31 | [array_const____deadlock](#array_const____deadlock) | NEG | ✅ | 3 | ✅ | 2 | 10→0 | 64→0 | 58→0 | 2→0 | 0→0 | 0→9 | 0→1 |
| 32 | [array_const____lock_mismatch](#array_const____lock_mismatch) | NEG | ❌ | c2rust | ✅ | 2 | 7→4 | 42→6 | 41→3 | 2→1 | 0→11 | 0→3 | 0→0 |
| 33 | [array_main____lock_leak](#array_main____lock_leak) | NEG | ❌ | c2rust | ✅ | 2 | 7→5 | 27→0 | 34→2 | 2→2 | 0→4 | 0→7 | 0→1 |
| 34 | [array_main____partial_critical_section](#array_main____partial_critical_section) | NEG | ✅ | 3 | ✅ | 2 | 7→6 | 32→0 | 37→3 | 2→2 | 0→4 | 0→8 | 0→1 |
| 35 | [array_simple____partial_critical_section](#array_simple____partial_critical_section) | NEG | ✅ | 1 | ❌ | c2rust | 7→6 | 22→0 | 33→2 | 4→4 | 0→1 | 0→7 | 0→1 |
| 36 | [global_assume2____self_lock](#global_assume2____self_lock) | NEG | ✅ | 1 | ❌ | c2rust | 8→6 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→1 |
| 37 | [global_assume____lock_leak](#global_assume____lock_leak) | NEG | ✅ | 1 | ✅ | 2 | 8→0 | 21→0 | 24→0 | 2→0 | 0→0 | 0→3 | 0→1 |
| 38 | [global_check____lock_leak](#global_check____lock_leak) | NEG | ❌ | c2rust | ❌ | c2rust | 10→10 | 39→32 | 26→18 | 2→2 | 0→1 | 0→5 | 0→2 |
| 39 | [global_check____lock_mismatch](#global_check____lock_mismatch) | NEG | ❌ | c2rust | ❌ | c2rust | 10→9 | 46→33 | 28→9 | 3→3 | 0→0 | 0→1 | 0→0 |
| 40 | [global_condvar____lost_wakeup](#global_condvar____lost_wakeup) | NEG | ❌ | c2rust | ✅ | 2 | 7→4 | 38→0 | 28→3 | 4→2 | 0→5 | 0→3 | 0→2 |
| 41 | [global_condvar____partial_critical_section](#global_condvar____partial_critical_section) | NEG | ✅ | 2 | ✅ | 2 | 7→0 | 40→0 | 29→0 | 4→0 | 0→0 | 0→7 | 0→2 |
| 42 | [global_custom____self_lock](#global_custom____self_lock) | NEG | ✅ | 1 | ✅ | 12 | 12→0 | 26→0 | 25→0 | 2→0 | 0→0 | 0→3 | 0→2 |
| 43 | [global_main____self_lock](#global_main____self_lock) | NEG | ✅ | 1 | ✅ | 1 | 7→7 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→2 |
| 44 | [global_nested____deadlock](#global_nested____deadlock) | NEG | ✅ | 1 | ✅ | 1 | 8→2 | 48→0 | 28→3 | 4→0 | 0→6 | 0→5 | 0→2 |
| 45 | [global_read____lock_mismatch](#global_read____lock_mismatch) | NEG | ✅ | 3 | ✅ | 6 | 8→8 | 28→0 | 28→6 | 4→4 | 0→6 | 0→7 | 0→2 |
| 46 | [global_rwlock____lock_leak](#global_rwlock____lock_leak) | NEG | ✅ | 9 | ❌ | c2rust | 8→1 | 27→0 | 22→2 | 2→0 | 0→1 | 0→7 | 0→1 |
| 47 | [global_simple____partial_critical_section](#global_simple____partial_critical_section) | NEG | ✅ | 3 | ✅ | 18 | 7→7 | 24→0 | 25→4 | 4→4 | 0→2 | 0→4 | 0→2 |
| 48 | [global_while____lock_leak](#global_while____lock_leak) | NEG | ✅ | 2 | ✅ | 3 | 8→0 | 27→0 | 25→0 | 2→0 | 0→0 | 0→5 | 0→2 |
| 49 | [struct_alias____self_lock](#struct_alias____self_lock) | NEG | ❌ | c2rust | ❌ | c2rust | 10→6 | 28→0 | 32→5 | 3→0 | 0→4 | 0→5 | 0→2 |
| 50 | [struct_assume____deadlock](#struct_assume____deadlock) | NEG | ✅ | 20 | ✅ | 2 | 10→7 | 37→0 | 45→13 | 0→0 | 0→9 | 0→12 | 0→2 |
| 51 | [struct_condvar____lost_wakeup](#struct_condvar____lost_wakeup) | NEG | ✅ | 3 | ❌ | c2rust | 7→7 | 32→0 | 27→3 | 1→1 | 0→5 | 0→5 | 0→2 |
| 52 | [struct_dup____deadlock](#struct_dup____deadlock) | NEG | ❌ | c2rust | ✅ | 3 | 8→10 | 32→0 | 29→3 | 2→2 | 0→6 | 0→7 | 0→2 |
| 53 | [struct_init____partial_critical_section](#struct_init____partial_critical_section) | NEG | ❌ | c2rust | ❌ | c2rust | 7→7 | 29→0 | 34→7 | 2→2 | 0→6 | 0→5 | 0→2 |
| 54 | [struct_malloc2____lock_mismatch](#struct_malloc2____lock_mismatch) | NEG | ✅ | 4 | ❌ | c2rust | 8→8 | 35→0 | 34→2 | 2→1 | 0→3 | 0→6 | 0→2 |
| 55 | [struct_malloc____lost_wakeup](#struct_malloc____lost_wakeup) | NEG | ✅ | 1 | ❌ | c2rust | 7→3 | 41→0 | 38→5 | 0→0 | 0→0 | 0→3 | 0→2 |
| 56 | [struct_multiple____deadlock](#struct_multiple____deadlock) | NEG | ✅ | 4 | ✅ | 3 | 11→2 | 32→0 | 37→5 | 3→0 | 0→0 | 0→12 | 0→3 |
| 57 | [struct_nested____self_lock](#struct_nested____self_lock) | NEG | ✅ | 8 | ❌ | c2rust | 7→6 | 26→0 | 25→2 | 1→1 | 0→3 | 0→3 | 0→2 |
| 58 | [struct_simple____partial_critical_section](#struct_simple____partial_critical_section) | NEG | ❌ | c2rust | ✅ | 13 | 7→3 | 28→0 | 27→4 | 1→0 | 0→4 | 0→6 | 0→1 |
| 59 | [struct_spin____lock_leak](#struct_spin____lock_leak) | NEG | ✅ | 1 | ❌ | c2rust | 7→0 | 47→0 | 33→0 | 0→0 | 0→0 | 0→7 | 0→2 |
| 60 | [struct_timedwait____deadlock](#struct_timedwait____deadlock) | NEG | ❌ | c2rust | ✅ | 6 | 9→2 | 48→0 | 32→0 | 2→1 | 0→0 | 0→3 | 0→2 |
| 61 | [struct_timedwait____lost_wakeup](#struct_timedwait____lost_wakeup) | NEG | ❌ | c2rust | ✅ | 6 | 9→3 | 37→0 | 29→0 | 1→1 | 0→1 | 0→9 | 0→2 |
| 62 | [unused_func____lock_mismatch](#unused_func____lock_mismatch) | NEG | ✅ | 3 | ✅ | 2 | 8→6 | 28→0 | 27→2 | 3→0 | 0→3 | 0→3 | 0→1 |
| | **TOTAL** | 30/30 / 39/30 | — | 492→231→399 | 2035→692→185 | 1872→778→316 | 128→39→85 | 0→9→247 | 0→113→513 | 0→0→152 |

> **Reading the table**: Each metric cell shows **Original → ConCrat → LLM**. Compiles column shows **ConCrat / LLM**.

## All Metrics Summary

This section displays all 15 metrics for each sample in a compact format.

| Example | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_arc | std\_rwlock | std\_condvar | std\_thread | move\_closure | arc\_clone | join\_handle | arc\_mutex\_combo | lines |
|---------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| array_const | 7→7→0 | 42→20→0 | 41→23→0 | 2→1→0 | 0→0→0 | 0→5→5 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 183→132→57 |
| array_main | 7→7→5 | 32→20→0 | 37→23→3 | 2→1→2 | 0→0→3 | 0→7→7 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→0 | 0→0→0 | 0→0→1 | 0→0→0 | 211→151→71 |
| array_simple | 7→7→5 | 32→20→0 | 39→25→2 | 4→2→3 | 0→0→6 | 0→7→7 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→0 | 0→0→0 | 0→0→1 | 0→0→1 | 235→197→89 |
| global_assume | 8→8→0 | 24→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→5 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 117→118→51 |
| global_assume2 | 8→8→7 | 24→20→15 | 25→23→12 | 2→1→2 | 0→0→0 | 0→3→1 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 122→129→128 |
| global_check | 10→10→9 | 44→22→28 | 26→24→8 | 2→1→2 | 0→6→0 | 0→3→1 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 181→243→172 |
| global_condvar | 7→7→0 | 40→28→0 | 29→27→0 | 4→2→0 | 0→0→0 | 0→3→7 | 0→0→5 | 0→1→0 | 0→3→3 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 188→172→56 |
| global_custom | 12→12→4 | 26→20→0 | 25→23→3 | 2→1→0 | 0→0→2 | 0→3→5 | 0→0→3 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→0 | 0→0→0 | 0→0→1 | 0→0→1 | 142→156→104 |
| global_main | 7→7→3 | 24→20→0 | 25→23→3 | 2→1→1 | 0→0→5 | 0→3→3 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→0 | 115→113→48 |
| global_nested | 7→7→0 | 36→20→0 | 27→23→0 | 4→2→0 | 0→0→0 | 0→5→5 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 148→137→52 |
| global_read | 7→7→3 | 24→20→0 | 25→23→5 | 3→2→0 | 0→0→0 | 0→3→6 | 0→0→8 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→1 | 115→115→53 |
| global_rwlock | 8→8→5 | 28→22→0 | 22→22→3 | 2→1→1 | 0→0→4 | 0→1→5 | 0→0→1 | 0→3→5 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→0 | 125→116→82 |
| global_simple | 7→7→0 | 24→20→0 | 25→23→0 | 4→2→0 | 0→0→0 | 0→3→6 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 124→120→51 |
| global_trylock | 7→7→0 | 38→38→0 | 26→26→0 | 2→2→0 | 0→0→0 | 0→0→3 | 0→0→6 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 153→153→53 |
| global_while | 8→8→0 | 29→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→5 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→0 | 133→133→59 |
| struct_alias | 10→10→2 | 26→20→0 | 32→26→3 | 3→3→0 | 0→0→1 | 0→5→12 | 0→0→11 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→4 | 185→156→101 |
| struct_assume | 8→8→7 | 29→24→0 | 33→33→6 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 112→129→68 |
| struct_condvar | 7→7→7 | 34→26→0 | 28→26→3 | 1→1→1 | 0→0→5 | 0→3→3 | 0→0→4 | 0→1→0 | 0→3→3 | 0→0→2 | 0→0→2 | 0→0→1 | 0→0→2 | 0→0→0 | 186→164→69 |
| struct_dup | 7→7→9 | 28→20→0 | 28→24→4 | 2→2→2 | 0→0→7 | 0→5→5 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 166→147→88 |
| struct_empty | 7→7→6 | 30→24→0 | 34→32→8 | 1→1→0 | 0→0→6 | 0→4→7 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 142→133→65 |
| struct_init | 7→7→4 | 35→24→0 | 36→32→8 | 2→2→0 | 0→0→0 | 0→5→10 | 0→0→10 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 160→136→66 |
| struct_main | 7→7→8 | 24→20→0 | 25→23→3 | 1→1→1 | 0→0→5 | 0→3→3 | 0→0→4 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→0 | 124→120→75 |
| struct_malloc | 7→7→3 | 44→34→0 | 39→39→5 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→7 | 0→1→0 | 0→3→3 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 163→179→60 |
| struct_malloc2 | 8→8→7 | 33→24→0 | 32→32→2 | 1→1→0 | 0→0→3 | 0→4→6 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→2 | 0→0→2 | 0→0→1 | 120→135→75 |
| struct_multiple | 8→8→3 | 26→20→0 | 30→24→3 | 3→3→0 | 0→0→5 | 0→5→5 | 0→0→14 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→0 | 175→138→94 |
| struct_nested | 7→7→6 | 24→20→0 | 25→23→2 | 1→1→0 | 0→0→4 | 0→3→3 | 0→0→2 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 136→129→65 |
| struct_simple | 7→7→5 | 28→20→0 | 27→23→3 | 1→1→0 | 0→0→5 | 0→5→9 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→1 | 0→0→2 | 0→0→3 | 157→138→82 |
| struct_spin | 7→7→1 | 56→40→0 | 36→36→3 | 0→0→0 | 0→0→0 | 0→7→5 | 0→0→9 | 0→4→3 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 205→197→61 |
| struct_timedwait | 9→9→0 | 42→26→0 | 30→28→0 | 1→1→0 | 0→3→0 | 0→3→9 | 0→0→3 | 0→1→0 | 0→3→3 | 0→0→2 | 0→0→2 | 0→0→4 | 0→0→2 | 0→0→0 | 251→278→93 |
| unused_func | 8→8→0 | 26→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 119→119→44 |
| **TOTAL** | 492→231→399 | 2035→692→185 | 1872→778→316 | 128→39→85 | 0→9→247 | 0→113→513 | 0→0→509 | 0→34→18 | 0→37→48 | 0→0→152 | 0→0→113 | 0→0→147 | 0→0→152 | 0→0→66 | 10196→4483→7682 |

> **All Metrics** table shows all 15 metrics (including std\_arc, std\_rwlock, std\_condvar, move\_closure, arc\_clone, join\_handle, arc\_mutex\_combo) for each sample. Format: **Original → ConCrat → LLM** (or **Original → LLM** for negative samples).

## Aggregate Statistics

| Metric | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|--------|----------|---------|-----|:----------------:|:----------------:|
| unsafe | 492 | 231 | 399 | 53.0% | 18.9% |
| pthread | 2035 | 692 | 185 | 66.0% | 90.9% |
| raw\_ptr | 1872 | 778 | 316 | 58.4% | 83.1% |
| static\_mut | 128 | 39 | 85 | 69.5% | 33.6% |
| libc | 0 | 9 | 247 | — | — |
| std\_mutex | 0 | 113 | 513 | — | — |
| std\_arc | 0 | 0 | 509 | — | — |
| std\_rwlock | 0 | 34 | 18 | — | — |
| std\_condvar | 0 | 37 | 48 | — | — |
| std\_thread | 0 | 0 | 152 | — | — |
| move\_closure | 0 | 0 | 113 | — | — |
| arc\_clone | 0 | 0 | 147 | — | — |
| join\_handle | 0 | 0 | 152 | — | — |
| arc\_mutex\_combo | 0 | 0 | 66 | — | — |
| lines | 10196 | 4483 | 7682 | 56.0% | 24.7% |

| **Compile success** | — | 30/62 (48%) | 39/62 (63%) | | |

## Metric Categories Summary

Aggregate of metrics grouped by category (Lower is Better vs Higher is Better):

| Category | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|----------|----------|---------|-----|:----------------:|:----------------:|
| **Lower is Better** (∑unsafe, pthread, raw_ptr, static_mut, libc) | 4527 (905.4) | 1749 (349.8) | 1232 (246.4) | 61.4% | 72.8% |
| **Higher is Better** (∑std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 184 (20.44) | 1718 (190.89) | — | — |

## Safety Features Adoption

| Example | Round | std::sync::Mutex | Arc<Mutex> | RwLock | Condvar | std::thread | join() |
|---------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| array_const | 2 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| array_main | 2 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| array_simple | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_assume | 2 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_assume2 | c2rust | ·,L | ·,· | C,· | C,· | ·,· | ·,· |
| global_check | c2rust | ·,L | ·,· | C,· | C,· | ·,· | ·,· |
| global_condvar | 2 | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| global_custom | 12 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_main | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_nested | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_read | 6 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_rwlock | c2rust | ·,L | ·,· | C,L | C,· | ·,L | ·,L |
| global_simple | 18 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_trylock | 1 | ·,L | ·,· | ·,· | ·,· | ·,L | ·,L |
| global_while | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_alias | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_assume | 2 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_condvar | c2rust | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| struct_dup | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_empty | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_init | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_main | 6 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_malloc | c2rust | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| struct_malloc2 | c2rust | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_multiple | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_nested | c2rust | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_simple | 13 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_spin | c2rust | ·,L | ·,L | C,L | C,· | ·,L | ·,L |
| struct_timedwait | 6 | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| unused_func | 2 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |

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
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 183 | 132 | 57 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 92 (18.4) | 51 (10.2) | 0 (0.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 7 (0.78) | 14 (1.56) |

---

### array_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 5 | LLM |
| pthread | 32 | 20 | 0 | LLM |
| raw\_ptr | 37 | 23 | 3 | LLM |
| static\_mut | 2 | 1 | 2 | ConCrat |
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
| lines | 211 | 151 | 71 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 78 (15.6) | 51 (10.2) | 13 (2.6) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 9 (1.0) | 10 (1.11) |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, num_mutex; 5 unsafe keyword(s) remain

---

### array_simple

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 5 | LLM |
| pthread | 32 | 20 | 0 | LLM |
| raw\_ptr | 39 | 25 | 2 | LLM |
| static\_mut | 4 | 2 | 3 | ConCrat |
| libc | 0 | 0 | 6 | ConCrat |
| std\_mutex | 0 | 7 | 7 | tie |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 235 | 197 | 89 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 82 (16.4) | 54 (10.8) | 16 (3.2) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 9 (1.0) | 17 (1.89) |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, n2, n3; 5 unsafe keyword(s) remain

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
| std\_mutex | 0 | 3 | 5 | LLM |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 117 | 118 | 51 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 59 (11.8) | 52 (10.4) | 0 (0.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 5 (0.56) | 15 (1.67) |

---

### global_assume2

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 7 | LLM |
| pthread | 24 | 20 | 15 | LLM |
| raw\_ptr | 25 | 23 | 12 | LLM |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 1 | ConCrat |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 122 | 129 | 128 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 59 (11.8) | 52 (10.4) | 36 (7.2) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 5 (0.56) | 2 (0.22) |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (2 calls); static mut variables remain: n1, num_mutex; 7 unsafe keyword(s) remain

---

### global_check

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 10 | 10 | 9 | LLM |
| pthread | 44 | 22 | 28 | ConCrat |
| raw\_ptr | 26 | 24 | 8 | LLM |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 0 | 6 | 0 | LLM |
| std\_mutex | 0 | 3 | 1 | ConCrat |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 181 | 243 | 172 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 82 (16.4) | 63 (12.6) | 47 (9.4) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 5 (0.56) | 2 (0.22) |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (17 calls); static mut variables remain: n, m; 9 unsafe keyword(s) remain

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 80 (16.0) | 64 (12.8) | 0 (0.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 7 (0.78) | 23 (2.56) |

---

### global_custom

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 12 | 12 | 4 | LLM |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 3 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 0 | 0 | 2 | ConCrat |
| std\_mutex | 0 | 3 | 5 | LLM |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 142 | 156 | 104 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 65 (13.0) | 56 (11.2) | 9 (1.8) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 5 (0.56) | 11 (1.22) |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### global_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 3 | LLM |
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
| lines | 115 | 113 | 48 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 58 (11.6) | 51 (10.2) | 12 (2.4) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 5 (0.56) | 8 (0.89) |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 3 unsafe keyword(s) remain

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 74 (14.8) | 52 (10.4) | 0 (0.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 7 (0.78) | 19 (2.11) |

---

### global_read

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 3 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 5 | LLM |
| static\_mut | 3 | 2 | 0 | LLM |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 3 | 6 | LLM |
| std\_arc | 0 | 0 | 8 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 115 | 115 | 53 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 59 (11.8) | 52 (10.4) | 8 (1.6) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 5 (0.56) | 23 (2.56) |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

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
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 3 | 5 | LLM |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 125 | 116 | 82 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 60 (12.0) | 53 (10.6) | 13 (2.6) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 5 (0.56) | 15 (1.67) |

**Remaining Issues:**

- **LLM**: static mut variables remain: N; 5 unsafe keyword(s) remain

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
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 2 | LLM |
| lines | 124 | 120 | 51 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 60 (12.0) | 52 (10.4) | 0 (0.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 5 (0.56) | 22 (2.44) |

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
| lines | 153 | 153 | 53 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 73 (14.6) | 73 (14.6) | 0 (0.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 0 (0.0) | 17 (1.89) |

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
| std\_mutex | 0 | 3 | 5 | LLM |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 3 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 133 | 133 | 59 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 64 (12.8) | 52 (10.4) | 0 (0.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 5 (0.56) | 19 (2.11) |

---

### struct_alias

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 10 | 10 | 2 | LLM |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 32 | 26 | 3 | LLM |
| static\_mut | 3 | 3 | 0 | LLM |
| libc | 0 | 0 | 1 | ConCrat |
| std\_mutex | 0 | 5 | 12 | LLM |
| std\_arc | 0 | 0 | 11 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 3 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 4 | LLM |
| lines | 185 | 156 | 101 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 71 (14.2) | 59 (11.8) | 6 (1.2) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 7 (0.78) | 36 (4.0) |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

---

### struct_assume

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 7 | LLM |
| pthread | 29 | 24 | 0 | LLM |
| raw\_ptr | 33 | 33 | 6 | LLM |
| static\_mut | 0 | 0 | 0 | tie |
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
| lines | 112 | 129 | 68 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 70 (14.0) | 65 (13.0) | 13 (2.6) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 5 (0.56) | 12 (1.33) |

**Remaining Issues:**

- **LLM**: 7 unsafe keyword(s) remain

---

### struct_condvar

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 70 (14.0) | 60 (12.0) | 16 (3.2) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 7 (0.78) | 17 (1.89) |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 7 unsafe keyword(s) remain

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
| lines | 166 | 147 | 88 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 65 (13.0) | 53 (10.6) | 22 (4.4) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 7 (0.78) | 20 (2.22) |

**Remaining Issues:**

- **LLM**: static mut variables remain: S1, S2; 9 unsafe keyword(s) remain

---

### struct_empty

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 6 | LLM |
| pthread | 30 | 24 | 0 | LLM |
| raw\_ptr | 34 | 32 | 8 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 0 | 0 | 6 | ConCrat |
| std\_mutex | 0 | 4 | 7 | LLM |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 142 | 133 | 65 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 72 (14.4) | 64 (12.8) | 20 (4.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 6 (0.67) | 17 (1.89) |

**Remaining Issues:**

- **LLM**: 6 unsafe keyword(s) remain

---

### struct_init

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 35 | 24 | 0 | LLM |
| raw\_ptr | 36 | 32 | 8 | LLM |
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
| lines | 160 | 136 | 66 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 80 (16.0) | 65 (13.0) | 12 (2.4) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 7 (0.78) | 30 (3.33) |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 8 | ConCrat |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 3 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 3 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 124 | 120 | 75 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 57 (11.4) | 51 (10.2) | 17 (3.4) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 5 (0.56) | 16 (1.78) |

**Remaining Issues:**

- **LLM**: static mut variables remain: SHARED_DATA; 8 unsafe keyword(s) remain

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
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 163 | 179 | 60 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 90 (18.0) | 80 (16.0) | 8 (1.6) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 7 (0.78) | 21 (2.33) |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### struct_malloc2

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 7 | LLM |
| pthread | 33 | 24 | 0 | LLM |
| raw\_ptr | 32 | 32 | 2 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 0 | 0 | 3 | ConCrat |
| std\_mutex | 0 | 4 | 6 | LLM |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 2 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 1 | LLM |
| lines | 120 | 135 | 75 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 74 (14.8) | 65 (13.0) | 12 (2.4) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 6 (0.67) | 19 (2.11) |

**Remaining Issues:**

- **LLM**: 7 unsafe keyword(s) remain

---

### struct_multiple

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 3 | LLM |
| pthread | 26 | 20 | 0 | LLM |
| raw\_ptr | 30 | 24 | 3 | LLM |
| static\_mut | 3 | 3 | 0 | LLM |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 14 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 3 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 175 | 138 | 94 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 67 (13.4) | 55 (11.0) | 11 (2.2) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 7 (0.78) | 28 (3.11) |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

---

### struct_nested

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 6 | LLM |
| pthread | 24 | 20 | 0 | LLM |
| raw\_ptr | 25 | 23 | 2 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 0 | 0 | 4 | ConCrat |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 2 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| move\_closure | 0 | 0 | 1 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 1 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 136 | 129 | 65 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 57 (11.4) | 51 (10.2) | 12 (2.4) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 5 (0.56) | 9 (1.0) |

**Remaining Issues:**

- **LLM**: 6 unsafe keyword(s) remain

---

### struct_simple

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 5 | LLM |
| pthread | 28 | 20 | 0 | LLM |
| raw\_ptr | 27 | 23 | 3 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 0 | 0 | 5 | ConCrat |
| std\_mutex | 0 | 5 | 9 | LLM |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 1 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 3 | LLM |
| lines | 157 | 138 | 82 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 63 (12.6) | 51 (10.2) | 13 (2.6) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 7 (0.78) | 24 (2.67) |

**Remaining Issues:**

- **LLM**: 5 unsafe keyword(s) remain

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 99 (19.8) | 83 (16.6) | 4 (0.8) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 12 (1.33) | 27 (3.0) |

**Remaining Issues:**

- **LLM**: 1 unsafe keyword(s) remain

---

### struct_timedwait

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 9 | 9 | 0 | LLM |
| pthread | 42 | 26 | 0 | LLM |
| raw\_ptr | 30 | 28 | 0 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 0 | 3 | 0 | LLM |
| std\_mutex | 0 | 3 | 9 | LLM |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| move\_closure | 0 | 0 | 2 | LLM |
| arc\_clone | 0 | 0 | 4 | LLM |
| join\_handle | 0 | 0 | 2 | LLM |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 251 | 278 | 93 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 82 (16.4) | 67 (13.4) | 0 (0.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 7 (0.78) | 25 (2.78) |

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
| lines | 119 | 119 | 44 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 61 (12.2) | 52 (10.4) | 0 (0.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 5 (0.56) | 18 (2.0) |

---

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 134 (26.8) | 0 (0.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 16 (1.78) |

---

### array_const____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 42 | 6 | -36 |
| raw_ptr | 41 | 3 | -38 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 11 | +11 |
| lines | 183 | 87 | -96 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 92 (18.4) | 25 (5.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 4 (0.44) |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 4 unsafe keyword(s) remain

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
| lines | 207 | 68 | -139 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 70 (14.0) | 13 (2.6) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 17 (1.89) |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, num_mutex; 5 unsafe keyword(s) remain

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 78 (15.6) | 15 (3.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 20 (2.22) |

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 66 (13.2) | 13 (2.6) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 10 (1.11) |

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 61 (12.2) | 14 (2.8) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 7 (0.78) |

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 55 (11.0) | 0 (0.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 13 (1.44) |

---

### global_check____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 10 | +0 |
| pthread | 39 | 32 | -7 |
| raw_ptr | 26 | 18 | -8 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 1 | +1 |
| lines | 176 | 205 | +29 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 77 (15.4) | 63 (12.6) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 17 (1.89) |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (14 calls); static mut variables remain: n, m; 10 unsafe keyword(s) remain

---

### global_check____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 9 | -1 |
| pthread | 46 | 33 | -13 |
| raw_ptr | 28 | 9 | -19 |
| static_mut | 3 | 3 | +0 |
| libc | 0 | 0 | +0 |
| lines | 197 | 199 | +2 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 87 (17.4) | 54 (10.8) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 2 (0.22) |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (17 calls); static mut variables remain: n, m, r; 9 unsafe keyword(s) remain

---

### global_condvar____lost_wakeup

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 38 | 0 | -38 |
| raw_ptr | 28 | 3 | -25 |
| static_mut | 4 | 2 | -2 |
| libc | 0 | 5 | +5 |
| lines | 187 | 104 | -83 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 77 (15.4) | 14 (2.8) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 19 (2.11) |

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
| lines | 190 | 71 | -119 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 80 (16.0) | 0 (0.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 24 (2.67) |

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 65 (13.0) | 0 (0.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 21 (2.33) |

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 60 (12.0) | 15 (3.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 9 (1.0) |

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 88 (17.6) | 11 (2.2) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 20 (2.22) |

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 68 (13.6) | 24 (4.8) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 22 (2.44) |

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 59 (11.8) | 4 (0.8) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 22 (2.44) |

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 60 (12.0) | 17 (3.4) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 18 (2.0) |

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 62 (12.4) | 0 (0.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 19 (2.11) |

---

### struct_alias____self_lock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 6 | -4 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 32 | 5 | -27 |
| static_mut | 3 | 0 | -3 |
| libc | 0 | 4 | +4 |
| lines | 187 | 100 | -87 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 73 (14.6) | 15 (3.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 16 (1.78) |

**Remaining Issues:**

- **LLM**: 6 unsafe keyword(s) remain

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 92 (18.4) | 29 (5.8) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 37 (4.11) |

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 67 (13.4) | 16 (3.2) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 19 (2.11) |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 7 unsafe keyword(s) remain

---

### struct_dup____deadlock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 10 | +2 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 29 | 3 | -26 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 6 | +6 |
| lines | 180 | 128 | -52 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 71 (14.2) | 21 (4.2) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 26 (2.89) |

**Remaining Issues:**

- **LLM**: static mut variables remain: S1, S2; 10 unsafe keyword(s) remain

---

### struct_init____partial_critical_section

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 7 | +0 |
| pthread | 29 | 0 | -29 |
| raw_ptr | 34 | 7 | -27 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 6 | +6 |
| lines | 157 | 85 | -72 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 72 (14.4) | 22 (4.4) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 23 (2.56) |

**Remaining Issues:**

- **LLM**: static mut variables remain: S1, S2; 7 unsafe keyword(s) remain

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 79 (15.8) | 14 (2.8) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 16 (1.78) |

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 86 (17.2) | 8 (1.6) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 20 (2.22) |

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 83 (16.6) | 7 (1.4) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 47 (5.22) |

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 59 (11.8) | 12 (2.4) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 14 (1.56) |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 6 unsafe keyword(s) remain

---

### struct_simple____partial_critical_section

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 3 | -4 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 27 | 4 | -23 |
| static_mut | 1 | 0 | -1 |
| libc | 0 | 4 | +4 |
| lines | 158 | 80 | -78 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 63 (12.6) | 11 (2.2) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 13 (1.44) |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 87 (17.4) | 0 (0.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 19 (2.11) |

---

### struct_timedwait____deadlock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 2 | -7 |
| pthread | 48 | 0 | -48 |
| raw_ptr | 32 | 0 | -32 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 0 | +0 |
| lines | 271 | 98 | -173 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 91 (18.2) | 3 (0.6) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 25 (2.78) |

**Remaining Issues:**

- **LLM**: static mut variables remain: SHARED_DATA; 2 unsafe keyword(s) remain

---

### struct_timedwait____lost_wakeup

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 3 | -6 |
| pthread | 37 | 0 | -37 |
| raw_ptr | 29 | 0 | -29 |
| static_mut | 1 | 1 | +0 |
| libc | 0 | 1 | +1 |
| lines | 238 | 86 | -152 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 76 (15.2) | 5 (1.0) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 30 (3.33) |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 3 unsafe keyword(s) remain

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
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | 66 (13.2) | 11 (2.2) |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 (0.0) | 6 (0.67) |

**Remaining Issues:**

- **LLM**: 6 unsafe keyword(s) remain

---
