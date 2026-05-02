# Concurrency Transformation Comparison Report

Three-way comparison: **Original** (c2rust output) vs **ConCrat** (automated transform) vs **LLM** (LLM-based rewrite)

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
| 31 | [array_const____deadlock](#array_const____deadlock) | NEG | ✅ | 2 | ❌ | c2rust | 10→0 | 64→0 | 58→0 | 2→0 | 0→0 | 0→7 | 0→1 | 233→79 |
| 32 | [array_const____lock_mismatch](#array_const____lock_mismatch) | NEG | ❌ | c2rust | ❌ | c2rust | 7→5 | 42→0 | 41→4 | 2→0 | 0→4 | 0→8 | 0→1 | 183→70 |
| 33 | [array_main____lock_leak](#array_main____lock_leak) | NEG | ❌ | c2rust | ✅ | 1 | 7→4 | 27→0 | 34→2 | 2→2 | 0→4 | 0→7 | 0→1 | 207→67 |
| 34 | [array_main____partial_critical_section](#array_main____partial_critical_section) | NEG | ❌ | c2rust | ✅ | 1 | 7→4 | 32→0 | 37→2 | 2→2 | 0→4 | 0→7 | 0→1 | 212→72 |
| 35 | [array_simple____partial_critical_section](#array_simple____partial_critical_section) | NEG | ❌ | c2rust | ✅ | 1 | 7→6 | 22→0 | 33→2 | 4→4 | 0→6 | 0→7 | 0→1 | 227→85 |
| 36 | [global_assume2____self_lock](#global_assume2____self_lock) | NEG | ❌ | c2rust | ❌ | c2rust | 8→7 | 26→15 | 25→10 | 2→2 | 0→0 | 0→1 | 0→0 | 124→132 |
| 37 | [global_assume____lock_leak](#global_assume____lock_leak) | NEG | ❌ | c2rust | ✅ | 1 | 8→6 | 21→0 | 24→3 | 2→1 | 0→5 | 0→3 | 0→1 | 115→64 |
| 38 | [global_check____lock_leak](#global_check____lock_leak) | NEG | ✅ | 3 | ✅ | 11 | 10→0 | 39→0 | 26→0 | 2→0 | 0→0 | 0→5 | 0→2 | 176→82 |
| 39 | [global_check____lock_mismatch](#global_check____lock_mismatch) | NEG | ❌ | c2rust | ✅ | 11 | 10→7 | 46→0 | 28→3 | 3→0 | 0→5 | 0→6 | 0→2 | 197→133 |
| 40 | [global_condvar____lost_wakeup](#global_condvar____lost_wakeup) | NEG | ✅ | 2 | ✅ | 2 | 7→0 | 38→0 | 28→0 | 4→0 | 0→0 | 0→7 | 0→2 | 187→62 |
| 41 | [global_condvar____partial_critical_section](#global_condvar____partial_critical_section) | NEG | ✅ | 2 | ✅ | 2 | 7→0 | 40→0 | 29→0 | 4→0 | 0→0 | 0→7 | 0→2 | 190→61 |
| 42 | [global_custom____self_lock](#global_custom____self_lock) | NEG | ✅ | 3 | ✅ | 2 | 12→4 | 26→0 | 25→4 | 2→2 | 0→4 | 0→4 | 0→2 | 144→92 |
| 43 | [global_main____self_lock](#global_main____self_lock) | NEG | ✅ | 2 | ✅ | 1 | 7→4 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→2 | 117→61 |
| 44 | [global_nested____deadlock](#global_nested____deadlock) | NEG | ✅ | 1 | ✅ | 3 | 8→2 | 48→0 | 28→3 | 4→0 | 0→6 | 0→5 | 0→2 | 175→72 |
| 45 | [global_read____lock_mismatch](#global_read____lock_mismatch) | NEG | ✅ | 1 | ✅ | 6 | 8→2 | 28→0 | 28→3 | 4→0 | 0→6 | 0→5 | 0→2 | 141→59 |
| 46 | [global_rwlock____lock_leak](#global_rwlock____lock_leak) | NEG | ❌ | c2rust | ❌ | c2rust | 8→6 | 27→0 | 22→2 | 2→1 | 0→1 | 0→5 | 0→2 | 124→89 |
| 47 | [global_simple____partial_critical_section](#global_simple____partial_critical_section) | NEG | ❌ | c2rust | ❌ | c2rust | 7→5 | 24→0 | 25→5 | 4→0 | 0→6 | 0→6 | 0→2 | 125→72 |
| 48 | [global_while____lock_leak](#global_while____lock_leak) | NEG | ❌ | c2rust | ✅ | 2 | 8→0 | 27→0 | 25→0 | 2→0 | 0→0 | 0→5 | 0→2 | 131→57 |
| 49 | [struct_alias____self_lock](#struct_alias____self_lock) | NEG | ❌ | c2rust | ✅ | 5 | 10→3 | 28→0 | 32→2 | 3→0 | 0→4 | 0→5 | 0→2 | 187→100 |
| 50 | [struct_assume____deadlock](#struct_assume____deadlock) | NEG | ❌ | c2rust | ❌ | c2rust | 10→0 | 37→0 | 45→0 | 0→0 | 0→0 | 0→4 | 0→2 | 146→67 |
| 51 | [struct_condvar____lost_wakeup](#struct_condvar____lost_wakeup) | NEG | ❌ | c2rust | ❌ | c2rust | 7→4 | 32→0 | 27→3 | 1→0 | 0→2 | 0→3 | 0→1 | 185→58 |
| 52 | [struct_dup____deadlock](#struct_dup____deadlock) | NEG | ✅ | 9 | ✅ | 6 | 8→6 | 32→0 | 29→4 | 2→0 | 0→2 | 0→9 | 0→2 | 180→107 |
| 53 | [struct_init____partial_critical_section](#struct_init____partial_critical_section) | NEG | ✅ | 5 | ❌ | c2rust | 7→5 | 29→0 | 34→6 | 2→2 | 0→5 | 0→10 | 0→2 | 157→77 |
| 54 | [struct_malloc2____lock_mismatch](#struct_malloc2____lock_mismatch) | NEG | ✅ | 4 | ✅ | 3 | 8→5 | 35→0 | 34→2 | 2→0 | 0→3 | 0→8 | 0→2 | 145→68 |
| 55 | [struct_malloc____lost_wakeup](#struct_malloc____lost_wakeup) | NEG | ❌ | c2rust | ❌ | c2rust | 7→4 | 41→0 | 38→5 | 0→0 | 0→0 | 0→3 | 0→2 | 158→55 |
| 56 | [struct_multiple____deadlock](#struct_multiple____deadlock) | NEG | ✅ | 3 | ❌ | c2rust | 11→0 | 32→0 | 37→0 | 3→0 | 0→0 | 0→5 | 0→3 | 198→73 |
| 57 | [struct_nested____self_lock](#struct_nested____self_lock) | NEG | ✅ | 3 | ✅ | 6 | 7→4 | 26→0 | 25→3 | 1→0 | 0→4 | 0→5 | 0→2 | 138→65 |
| 58 | [struct_simple____partial_critical_section](#struct_simple____partial_critical_section) | NEG | ✅ | 5 | ✅ | 3 | 7→3 | 28→0 | 27→1 | 1→1 | 0→3 | 0→10 | 0→2 | 158→90 |
| 59 | [struct_spin____lock_leak](#struct_spin____lock_leak) | NEG | ❌ | c2rust | ✅ | 1 | 7→2 | 47→0 | 33→6 | 0→0 | 0→0 | 0→7 | 0→2 | 199→64 |
| 60 | [struct_timedwait____deadlock](#struct_timedwait____deadlock) | NEG | ✅ | 3 | ❌ | c2rust | 9→6 | 48→0 | 32→0 | 2→1 | 0→2 | 0→9 | 0→2 | 271→117 |
| 61 | [struct_timedwait____lost_wakeup](#struct_timedwait____lost_wakeup) | NEG | ❌ | c2rust | ❌ | c2rust | 9→1 | 37→0 | 29→0 | 1→1 | 0→0 | 0→3 | 0→2 | 238→81 |
| 62 | [unused_func____lock_mismatch](#unused_func____lock_mismatch) | NEG | ✅ | 2 | ✅ | 1 | 8→6 | 28→0 | 27→3 | 3→1 | 0→5 | 0→5 | 0→1 | 135→70 |
| | **TOTAL** | 30/30 / 36/30 | — | 492→231→332 | 2035→692→30 | 1872→778→231 | 128→39→57 | 0→9→243 | 0→113→526 | 0→0→163 | 10196→4483→7059 |

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
| **TOTAL** | 492→231→332 | 2035→692→30 | 1872→778→231 | 128→39→57 | 0→9→243 | 0→113→526 | 0→0→513 | 0→34→18 | 0→37→40 | 0→0→163 | 0→0→126 | 0→0→161 | 0→0→161 | 0→0→65 | 10196→4483→7059 |

> **All Metrics** table shows all 15 metrics (including std\_arc, std\_rwlock, std\_condvar, move\_closure, arc\_clone, join\_handle, arc\_mutex\_combo) for each sample. Format: **Original → ConCrat → LLM** (or **Original → LLM** for negative samples).

## Aggregate Statistics

| Metric | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|--------|----------|---------|-----|:----------------:|:----------------:|
| unsafe | 492 | 231 | 332 | 53.0% | 32.5% |
| pthread | 2035 | 692 | 30 | 66.0% | 98.5% |
| raw\_ptr | 1872 | 778 | 231 | 58.4% | 87.7% |
| static\_mut | 128 | 39 | 57 | 69.5% | 55.5% |
| libc | 0 | 9 | 243 | — | — |
| std\_mutex | 0 | 113 | 526 | — | — |
| std\_arc | 0 | 0 | 513 | — | — |
| std\_rwlock | 0 | 34 | 18 | — | — |
| std\_condvar | 0 | 37 | 40 | — | — |
| std\_thread | 0 | 0 | 163 | — | — |
| move\_closure | 0 | 0 | 126 | — | — |
| arc\_clone | 0 | 0 | 161 | — | — |
| join\_handle | 0 | 0 | 161 | — | — |
| arc\_mutex\_combo | 0 | 0 | 65 | — | — |
| lines | 10196 | 4483 | 7059 | 56.0% | 30.8% |

| **Compile success** | — | 30/62 (48%) | 36/62 (58%) | | |

## Metric Categories Summary

Aggregate of metrics grouped by category (Lower is Better vs Higher is Better):

| Category | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|----------|----------|---------|-----|:----------------:|:----------------:|
| **Lower is Better** (∑unsafe, pthread, raw_ptr, static_mut, libc, lines) | 14723 | 6232 | 7952 | 57.7% | 46.0% |
| **Higher is Better** (∑std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 184 | 1773 | — | — |

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

### array_const____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 0 | -10 |
| pthread | 64 | 0 | -64 |
| raw_ptr | 58 | 0 | -58 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 233 | 79 | -154 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 367 | 79 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 16 |

---

### array_const____lock_mismatch

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 5 | -2 |
| pthread | 42 | 0 | -42 |
| raw_ptr | 41 | 4 | -37 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 4 | +4 |
| lines | 183 | 70 | -113 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 275 | 83 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 24 |

**Remaining Issues:**

- **LLM**: 5 unsafe keyword(s) remain

---

### array_main____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 27 | 0 | -27 |
| raw_ptr | 34 | 2 | -32 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 4 | +4 |
| lines | 207 | 67 | -140 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 277 | 79 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 17 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, num_mutex; 4 unsafe keyword(s) remain

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
| lines | 212 | 72 | -140 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 290 | 84 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 10 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, num_mutex; 4 unsafe keyword(s) remain

---

### array_simple____partial_critical_section

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 6 | -1 |
| pthread | 22 | 0 | -22 |
| raw_ptr | 33 | 2 | -31 |
| static_mut | 4 | 4 | +0 |
| libc | 0 | 6 | +6 |
| lines | 227 | 85 | -142 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 293 | 103 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 10 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n1, n2, n3, num_mutex; 6 unsafe keyword(s) remain

---

### global_assume2____self_lock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 7 | -1 |
| pthread | 26 | 15 | -11 |
| raw_ptr | 25 | 10 | -15 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 0 | +0 |
| lines | 124 | 132 | +8 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 166 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 2 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (4 calls); static mut variables remain: n1, num_mutex; 7 unsafe keyword(s) remain

---

### global_assume____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 6 | -2 |
| pthread | 21 | 0 | -21 |
| raw_ptr | 24 | 3 | -21 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 5 | +5 |
| lines | 115 | 64 | -51 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 170 | 79 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 10 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 6 unsafe keyword(s) remain

---

### global_check____lock_leak

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 0 | -10 |
| pthread | 39 | 0 | -39 |
| raw_ptr | 26 | 0 | -26 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 0 | +0 |
| lines | 176 | 82 | -94 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 253 | 82 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 18 |

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
| lines | 197 | 133 | -64 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 284 | 148 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 23 |

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
| lines | 187 | 62 | -125 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 264 | 62 |
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
| lines | 190 | 61 | -129 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 270 | 61 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 23 |

---

### global_custom____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 12 | 4 | -8 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 4 | -21 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 4 | +4 |
| lines | 144 | 92 | -52 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 209 | 106 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1, NUM_MUTEX_ARC; 4 unsafe keyword(s) remain

---

### global_main____self_lock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 3 | -22 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 4 | +4 |
| lines | 117 | 61 | -56 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 177 | 73 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 9 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 4 unsafe keyword(s) remain

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
| lines | 175 | 72 | -103 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 263 | 83 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

---

### global_read____lock_mismatch

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 2 | -6 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 28 | 3 | -25 |
| static_mut | 4 | 0 | -4 |
| libc | 0 | 6 | +6 |
| lines | 141 | 59 | -82 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 209 | 70 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

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
| lines | 124 | 89 | -35 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 183 | 99 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 15 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N; 6 unsafe keyword(s) remain

---

### global_simple____partial_critical_section

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 5 | -2 |
| pthread | 24 | 0 | -24 |
| raw_ptr | 25 | 5 | -20 |
| static_mut | 4 | 0 | -4 |
| libc | 0 | 6 | +6 |
| lines | 125 | 72 | -53 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 88 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 23 |

**Remaining Issues:**

- **LLM**: 5 unsafe keyword(s) remain

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
| lines | 131 | 57 | -74 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 193 | 57 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 19 |

---

### struct_alias____self_lock

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 10 | 3 | -7 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 32 | 2 | -30 |
| static_mut | 3 | 0 | -3 |
| libc | 0 | 4 | +4 |
| lines | 187 | 100 | -87 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 260 | 109 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 24 |

**Remaining Issues:**

- **LLM**: 3 unsafe keyword(s) remain

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
| lines | 146 | 67 | -79 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 238 | 67 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 22 |

---

### struct_condvar____lost_wakeup

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 27 | 3 | -24 |
| static_mut | 1 | 0 | -1 |
| libc | 0 | 2 | +2 |
| lines | 185 | 58 | -127 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 252 | 67 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 13 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_dup____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 6 | -2 |
| pthread | 32 | 0 | -32 |
| raw_ptr | 29 | 4 | -25 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 2 | +2 |
| lines | 180 | 107 | -73 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 251 | 119 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 20 |

**Remaining Issues:**

- **LLM**: 6 unsafe keyword(s) remain

---

### struct_init____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 5 | -2 |
| pthread | 29 | 0 | -29 |
| raw_ptr | 34 | 6 | -28 |
| static_mut | 2 | 2 | +0 |
| libc | 0 | 5 | +5 |
| lines | 157 | 77 | -80 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 229 | 95 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 32 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S1, S2; 5 unsafe keyword(s) remain

---

### struct_malloc2____lock_mismatch

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 5 | -3 |
| pthread | 35 | 0 | -35 |
| raw_ptr | 34 | 2 | -32 |
| static_mut | 2 | 0 | -2 |
| libc | 0 | 3 | +3 |
| lines | 145 | 68 | -77 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 224 | 78 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 21 |

**Remaining Issues:**

- **LLM**: 5 unsafe keyword(s) remain

---

### struct_malloc____lost_wakeup

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 4 | -3 |
| pthread | 41 | 0 | -41 |
| raw_ptr | 38 | 5 | -33 |
| static_mut | 0 | 0 | +0 |
| libc | 0 | 0 | +0 |
| lines | 158 | 55 | -103 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 244 | 64 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 21 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

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
| unsafe | 7 | 4 | -3 |
| pthread | 26 | 0 | -26 |
| raw_ptr | 25 | 3 | -22 |
| static_mut | 1 | 0 | -1 |
| libc | 0 | 4 | +4 |
| lines | 138 | 65 | -73 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 197 | 76 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 21 |

**Remaining Issues:**

- **LLM**: 4 unsafe keyword(s) remain

---

### struct_simple____partial_critical_section

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 3 | -4 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 27 | 1 | -26 |
| static_mut | 1 | 1 | +0 |
| libc | 0 | 3 | +3 |
| lines | 158 | 90 | -68 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 221 | 98 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 29 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 3 unsafe keyword(s) remain

---

### struct_spin____lock_leak

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 7 | 2 | -5 |
| pthread | 47 | 0 | -47 |
| raw_ptr | 33 | 6 | -27 |
| static_mut | 0 | 0 | +0 |
| libc | 0 | 0 | +0 |
| lines | 199 | 64 | -135 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 286 | 72 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 25 |

**Remaining Issues:**

- **LLM**: 2 unsafe keyword(s) remain

---

### struct_timedwait____deadlock

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 6 | -3 |
| pthread | 48 | 0 | -48 |
| raw_ptr | 32 | 0 | -32 |
| static_mut | 2 | 1 | -1 |
| libc | 0 | 2 | +2 |
| lines | 271 | 117 | -154 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 362 | 126 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 27 |

**Remaining Issues:**

- **LLM**: static mut variables remain: SHARED_DATA_INSTANCE; 6 unsafe keyword(s) remain

---

### struct_timedwait____lost_wakeup

**Compiles**: LLM ❌ No

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 9 | 1 | -8 |
| pthread | 37 | 0 | -37 |
| raw_ptr | 29 | 0 | -29 |
| static_mut | 1 | 1 | +0 |
| libc | 0 | 0 | +0 |
| lines | 238 | 81 | -157 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 314 | 83 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 24 |

**Remaining Issues:**

- **LLM**: static mut variables remain: S; 1 unsafe keyword(s) remain

---

### unused_func____lock_mismatch

**Compiles**: LLM ✅ Yes

| Metric | Original | LLM | Difference |
|--------|:--------:|:---:|:----------:|
| unsafe | 8 | 6 | -2 |
| pthread | 28 | 0 | -28 |
| raw_ptr | 27 | 3 | -24 |
| static_mut | 3 | 1 | -2 |
| libc | 0 | 5 | +5 |
| lines | 135 | 70 | -65 |

**Category Totals:**

| Category | Original | LLM |
|----------|:--------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 201 | 85 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 8 |

**Remaining Issues:**

- **LLM**: static mut variables remain: N1; 6 unsafe keyword(s) remain

---
