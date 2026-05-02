# Concurrency Transformation Comparison Report

Three-way comparison: **Original** (c2rust output) vs **ConCrat** (automated transform) vs **LLM** (LLM-based rewrite)

## Summary Overview

| # | Example | Compiles (C / L) | Round | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_thread | lines |
|---|---------|:----------------:|:---:|------|------|------|------|------|------|------|------|
| 1 | [array_const](#array_const) | ✅ / ✅ | 2 | 7→7→0 | 42→20→0 | 41→23→0 | 2→1→0 | 0→0→0 | 0→5→9 | 0→0→1 | 183→132→60 |
| 2 | [array_main](#array_main) | ✅ / ✅ | c2rust | 7→7→7 | 32→20→32 | 37→23→37 | 2→1→2 | 0→0→0 | 0→7→0 | 0→0→0 | 211→151→212 |
| 3 | [array_simple](#array_simple) | ✅ / ✅ | c2rust | 7→7→7 | 32→20→32 | 39→25→39 | 4→2→4 | 0→0→0 | 0→7→0 | 0→0→0 | 235→197→236 |
| 4 | [global_assume](#global_assume) | ✅ / ✅ | 1 | 8→8→8 | 24→20→0 | 25→23→3 | 2→1→1 | 0→0→5 | 0→3→3 | 0→0→2 | 117→118→49 |
| 5 | [global_assume2](#global_assume2) | ✅ / ✅ | 1 | 8→8→0 | 24→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→1 | 122→129→53 |
| 6 | [global_check](#global_check) | ✅ / ✅ | c2rust | 10→10→10 | 44→22→44 | 26→24→26 | 2→1→2 | 0→6→0 | 0→3→0 | 0→0→0 | 181→243→182 |
| 7 | [global_condvar](#global_condvar) | ✅ / ✅ | 2 | 7→7→0 | 40→28→0 | 29→27→0 | 4→2→0 | 0→0→0 | 0→3→7 | 0→0→2 | 188→172→56 |
| 8 | [global_custom](#global_custom) | ✅ / ✅ | c2rust | 12→12→12 | 26→20→26 | 25→23→25 | 2→1→2 | 0→0→0 | 0→3→0 | 0→0→0 | 142→156→143 |
| 9 | [global_main](#global_main) | ✅ / ✅ | 1 | 7→7→3 | 24→20→0 | 25→23→5 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→2 | 115→113→51 |
| 10 | [global_nested](#global_nested) | ✅ / ✅ | c2rust | 7→7→7 | 36→20→36 | 27→23→27 | 4→2→4 | 0→0→0 | 0→5→0 | 0→0→0 | 148→137→149 |
| 11 | [global_read](#global_read) | ✅ / ✅ | 6 | 7→7→3 | 24→20→0 | 25→23→4 | 3→2→0 | 0→0→0 | 0→3→6 | 0→0→1 | 115→115→53 |
| 12 | [global_rwlock](#global_rwlock) | ✅ / ✅ | c2rust | 8→8→8 | 28→22→28 | 22→22→22 | 2→1→2 | 0→0→0 | 0→1→0 | 0→0→0 | 125→116→126 |
| 13 | [global_simple](#global_simple) | ✅ / ✅ | 7 | 7→7→0 | 24→20→0 | 25→23→0 | 4→2→0 | 0→0→0 | 0→3→6 | 0→0→1 | 124→120→56 |
| 14 | [global_trylock](#global_trylock) | ✅ / ✅ | 1 | 7→7→0 | 38→38→0 | 26→26→0 | 2→2→0 | 0→0→0 | 0→0→3 | 0→0→2 | 153→153→54 |
| 15 | [global_while](#global_while) | ✅ / ✅ | c2rust | 8→8→8 | 29→20→29 | 25→23→25 | 2→1→2 | 0→0→0 | 0→3→0 | 0→0→0 | 133→133→134 |
| 16 | [struct_alias](#struct_alias) | ✅ / ✅ | c2rust | 10→10→10 | 26→20→26 | 32→26→32 | 3→3→3 | 0→0→0 | 0→5→0 | 0→0→0 | 185→156→186 |
| 17 | [struct_assume](#struct_assume) | ✅ / ✅ | 1 | 8→8→5 | 29→24→0 | 33→33→6 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→2 | 112→129→62 |
| 18 | [struct_condvar](#struct_condvar) | ✅ / ✅ | 3 | 7→7→7 | 34→26→0 | 28→26→3 | 1→1→1 | 0→0→5 | 0→3→3 | 0→0→2 | 186→164→69 |
| 19 | [struct_dup](#struct_dup) | ✅ / ✅ | 3 | 7→7→9 | 28→20→0 | 28→24→2 | 2→2→2 | 0→0→5 | 0→5→5 | 0→0→2 | 166→147→88 |
| 20 | [struct_empty](#struct_empty) | ✅ / ✅ | c2rust | 7→7→7 | 30→24→30 | 34→32→34 | 1→1→1 | 0→0→0 | 0→4→0 | 0→0→0 | 142→133→143 |
| 21 | [struct_init](#struct_init) | ✅ / ✅ | c2rust | 7→7→7 | 35→24→35 | 36→32→36 | 2→2→2 | 0→0→0 | 0→5→0 | 0→0→0 | 160→136→161 |
| 22 | [struct_main](#struct_main) | ✅ / ✅ | 5 | 7→7→5 | 24→20→0 | 25→23→2 | 1→1→0 | 0→0→4 | 0→3→5 | 0→0→1 | 124→120→59 |
| 23 | [struct_malloc](#struct_malloc) | ✅ / ✅ | c2rust | 7→7→7 | 44→34→44 | 39→39→39 | 0→0→0 | 0→0→0 | 0→3→0 | 0→0→0 | 163→179→164 |
| 24 | [struct_malloc2](#struct_malloc2) | ✅ / ✅ | c2rust | 8→8→8 | 33→24→33 | 32→32→32 | 1→1→1 | 0→0→0 | 0→4→0 | 0→0→0 | 120→135→121 |
| 25 | [struct_multiple](#struct_multiple) | ✅ / ❌ | c2rust | 8→8→8 | 26→20→0 | 30→24→3 | 3→3→3 | 0→0→5 | 0→5→5 | 0→0→2 | 175→138→83 |
| 26 | [struct_nested](#struct_nested) | ✅ / ✅ | 6 | 7→7→2 | 24→20→0 | 25→23→1 | 1→1→0 | 0→0→3 | 0→3→7 | 0→0→2 | 136→129→61 |
| 27 | [struct_simple](#struct_simple) | ✅ / ✅ | c2rust | 7→7→7 | 28→20→28 | 27→23→27 | 1→1→1 | 0→0→0 | 0→5→0 | 0→0→0 | 157→138→158 |
| 28 | [struct_spin](#struct_spin) | ✅ / ✅ | c2rust | 7→7→7 | 56→40→56 | 36→36→36 | 0→0→0 | 0→0→0 | 0→7→0 | 0→0→0 | 205→197→206 |
| 29 | [struct_timedwait](#struct_timedwait) | ✅ / ✅ | c2rust | 9→9→9 | 42→26→42 | 30→28→30 | 1→1→1 | 0→3→0 | 0→3→0 | 0→0→0 | 251→278→252 |
| 30 | [unused_func](#unused_func) | ✅ / ✅ | 1 | 8→8→6 | 26→20→0 | 25→23→3 | 2→1→1 | 0→0→5 | 0→3→4 | 0→0→1 | 119→119→66 |
| 31 | [array_const____deadlock](#array_const____deadlock) | NEG | ✅ | 5 | ✅ | 2 | 10→0 | 64→0 | 58→0 | 2→0 | 0→0 | 0→12 | 0→1 | 233→99 |
| 32 | [array_const____lock_mismatch](#array_const____lock_mismatch) | NEG | ✅ | c2rust | ✅ | 2 | 7→7 | 42→42 | 41→41 | 2→2 | 0→0 | 0→0 | 0→0 | 183→184 |
| 33 | [array_main____lock_leak](#array_main____lock_leak) | NEG | ✅ | c2rust | ✅ | c2rust | 7→7 | 27→27 | 34→34 | 2→2 | 0→0 | 0→0 | 0→0 | 207→208 |
| 34 | [array_main____partial_critical_section](#array_main____partial_critical_section) | NEG | ✅ | c2rust | ✅ | c2rust | 7→7 | 32→32 | 37→37 | 2→2 | 0→0 | 0→0 | 0→0 | 212→213 |
| 35 | [array_simple____partial_critical_section](#array_simple____partial_critical_section) | NEG | ✅ | 2 | ✅ | c2rust | 7→6 | 22→0 | 33→2 | 4→4 | 0→4 | 0→7 | 0→1 | 227→70 |
| 36 | [global_assume2____self_lock](#global_assume2____self_lock) | NEG | ✅ | 1 | ✅ | 1 | 8→5 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→1 | 124→71 |
| 37 | [global_assume____lock_leak](#global_assume____lock_leak) | NEG | ✅ | 1 | ✅ | 1 | 8→0 | 21→0 | 24→0 | 2→0 | 0→0 | 0→3 | 0→1 | 115→49 |
| 38 | [global_check____lock_leak](#global_check____lock_leak) | NEG | ✅ | 3 | ✅ | c2rust | 10→8 | 39→0 | 26→3 | 2→2 | 0→5 | 0→10 | 0→2 | 176→124 |
| 39 | [global_check____lock_mismatch](#global_check____lock_mismatch) | NEG | ✅ | c2rust | ✅ | c2rust | 10→10 | 46→46 | 28→28 | 3→3 | 0→0 | 0→0 | 0→0 | 197→198 |
| 40 | [global_condvar____lost_wakeup](#global_condvar____lost_wakeup) | NEG | ✅ | 2 | ✅ | 2 | 7→0 | 38→0 | 28→0 | 4→0 | 0→0 | 0→7 | 0→2 | 187→60 |
| 41 | [global_condvar____partial_critical_section](#global_condvar____partial_critical_section) | NEG | ✅ | 2 | ✅ | 2 | 7→0 | 40→0 | 29→0 | 4→0 | 0→0 | 0→7 | 0→2 | 190→66 |
| 42 | [global_custom____self_lock](#global_custom____self_lock) | NEG | ✅ | c2rust | ✅ | c2rust | 12→12 | 26→26 | 25→25 | 2→2 | 0→0 | 0→0 | 0→0 | 144→145 |
| 43 | [global_main____self_lock](#global_main____self_lock) | NEG | ✅ | 1 | ✅ | 1 | 7→7 | 26→0 | 25→3 | 2→1 | 0→4 | 0→4 | 0→2 | 117→68 |
| 44 | [global_nested____deadlock](#global_nested____deadlock) | NEG | ✅ | 1 | ✅ | c2rust | 8→2 | 48→0 | 28→3 | 4→0 | 0→6 | 0→5 | 0→2 | 175→73 |
| 45 | [global_read____lock_mismatch](#global_read____lock_mismatch) | NEG | ✅ | c2rust | ✅ | 6 | 8→8 | 28→28 | 28→28 | 4→4 | 0→0 | 0→0 | 0→0 | 141→142 |
| 46 | [global_rwlock____lock_leak](#global_rwlock____lock_leak) | NEG | ✅ | c2rust | ✅ | c2rust | 8→8 | 27→27 | 22→22 | 2→2 | 0→0 | 0→0 | 0→0 | 124→125 |
| 47 | [global_simple____partial_critical_section](#global_simple____partial_critical_section) | NEG | ✅ | 2 | ✅ | 7 | 7→1 | 24→0 | 25→1 | 4→0 | 0→2 | 0→7 | 0→1 | 125→69 |
| 48 | [global_while____lock_leak](#global_while____lock_leak) | NEG | ✅ | 1 | ✅ | c2rust | 8→0 | 27→0 | 25→0 | 2→0 | 0→0 | 0→3 | 0→2 | 131→61 |
| 49 | [struct_alias____self_lock](#struct_alias____self_lock) | NEG | ✅ | c2rust | ✅ | c2rust | 10→10 | 28→28 | 32→32 | 3→3 | 0→0 | 0→0 | 0→0 | 187→188 |
| 50 | [struct_assume____deadlock](#struct_assume____deadlock) | NEG | ✅ | c2rust | ✅ | 1 | 10→10 | 37→37 | 45→45 | 0→0 | 0→0 | 0→0 | 0→0 | 146→147 |
| 51 | [struct_condvar____lost_wakeup](#struct_condvar____lost_wakeup) | NEG | ✅ | 3 | ✅ | 3 | 7→7 | 32→0 | 27→3 | 1→1 | 0→5 | 0→5 | 0→2 | 185→71 |
| 52 | [struct_dup____deadlock](#struct_dup____deadlock) | NEG | ✅ | 8 | ✅ | 3 | 8→10 | 32→0 | 29→3 | 2→2 | 0→2 | 0→5 | 0→2 | 180→112 |
| 53 | [struct_init____partial_critical_section](#struct_init____partial_critical_section) | NEG | ✅ | c2rust | ✅ | c2rust | 7→7 | 29→29 | 34→34 | 2→2 | 0→0 | 0→0 | 0→0 | 157→158 |
| 54 | [struct_malloc2____lock_mismatch](#struct_malloc2____lock_mismatch) | NEG | ✅ | c2rust | ✅ | c2rust | 8→8 | 35→35 | 34→34 | 2→2 | 0→0 | 0→0 | 0→0 | 145→146 |
| 55 | [struct_malloc____lost_wakeup](#struct_malloc____lost_wakeup) | NEG | ✅ | c2rust | ✅ | c2rust | 7→7 | 41→41 | 38→38 | 0→0 | 0→0 | 0→0 | 0→0 | 158→159 |
| 56 | [struct_multiple____deadlock](#struct_multiple____deadlock) | NEG | ✅ | 3 | ❌ | c2rust | 11→0 | 32→0 | 37→0 | 3→0 | 0→0 | 0→5 | 0→3 | 198→73 |
| 57 | [struct_nested____self_lock](#struct_nested____self_lock) | NEG | ✅ | 4 | ✅ | 6 | 7→5 | 26→0 | 25→2 | 1→0 | 0→1 | 0→5 | 0→1 | 138→69 |
| 58 | [struct_simple____partial_critical_section](#struct_simple____partial_critical_section) | NEG | ✅ | 3 | ✅ | c2rust | 7→3 | 28→0 | 27→2 | 1→1 | 0→4 | 0→7 | 0→2 | 158→85 |
| 59 | [struct_spin____lock_leak](#struct_spin____lock_leak) | NEG | ✅ | 1 | ✅ | c2rust | 7→0 | 47→0 | 33→0 | 0→0 | 0→0 | 0→7 | 0→2 | 199→58 |
| 60 | [struct_timedwait____deadlock](#struct_timedwait____deadlock) | NEG | ✅ | c2rust | ✅ | c2rust | 9→9 | 48→48 | 32→32 | 2→2 | 0→0 | 0→0 | 0→0 | 271→272 |
| 61 | [struct_timedwait____lost_wakeup](#struct_timedwait____lost_wakeup) | NEG | ✅ | c2rust | ✅ | c2rust | 9→9 | 37→37 | 29→29 | 1→1 | 0→0 | 0→0 | 0→0 | 238→239 |
| 62 | [unused_func____lock_mismatch](#unused_func____lock_mismatch) | NEG | ✅ | c2rust | ✅ | 1 | 8→8 | 28→28 | 27→27 | 3→3 | 0→0 | 0→0 | 0→0 | 135→136 |
| | **TOTAL** | 30/30 / 61/30 | — | 492→231→539 | 2035→692→1543 | 1872→778→1521 | 128→39→119 | 0→9→106 | 0→113→278 | 0→0→82 | 10196→4483→11369 |

> **Reading the table**: Each metric cell shows **Original → ConCrat → LLM**. Compiles column shows **ConCrat / LLM**.

## All Metrics Summary

This section displays all 15 metrics for each sample in a compact format.

| Example | unsafe | pthread | raw\_ptr | static\_mut | libc | std\_mutex | std\_arc | std\_rwlock | std\_condvar | std\_thread | move\_closure | arc\_clone | join\_handle | arc\_mutex\_combo | lines |
|---------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| array_const | 7→7→0 | 42→20→0 | 41→23→0 | 2→1→0 | 0→0→0 | 0→5→9 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 183→132→60 |
| array_main | 7→7→7 | 32→20→32 | 37→23→37 | 2→1→2 | 0→0→0 | 0→7→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 211→151→212 |
| array_simple | 7→7→7 | 32→20→32 | 39→25→39 | 4→2→4 | 0→0→0 | 0→7→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 235→197→236 |
| global_assume | 8→8→8 | 24→20→0 | 25→23→3 | 2→1→1 | 0→0→5 | 0→3→3 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→0 | 117→118→49 |
| global_assume2 | 8→8→0 | 24→20→0 | 25→23→0 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→0 | 122→129→53 |
| global_check | 10→10→10 | 44→22→44 | 26→24→26 | 2→1→2 | 0→6→0 | 0→3→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 181→243→182 |
| global_condvar | 7→7→0 | 40→28→0 | 29→27→0 | 4→2→0 | 0→0→0 | 0→3→7 | 0→0→5 | 0→1→0 | 0→3→3 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 188→172→56 |
| global_custom | 12→12→12 | 26→20→26 | 25→23→25 | 2→1→2 | 0→0→0 | 0→3→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 142→156→143 |
| global_main | 7→7→3 | 24→20→0 | 25→23→5 | 2→1→0 | 0→0→0 | 0→3→3 | 0→0→8 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 115→113→51 |
| global_nested | 7→7→7 | 36→20→36 | 27→23→27 | 4→2→4 | 0→0→0 | 0→5→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 148→137→149 |
| global_read | 7→7→3 | 24→20→0 | 25→23→4 | 3→2→0 | 0→0→0 | 0→3→6 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 115→115→53 |
| global_rwlock | 8→8→8 | 28→22→28 | 22→22→22 | 2→1→2 | 0→0→0 | 0→1→0 | 0→0→0 | 0→3→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 125→116→126 |
| global_simple | 7→7→0 | 24→20→0 | 25→23→0 | 4→2→0 | 0→0→0 | 0→3→6 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→2 | 124→120→56 |
| global_trylock | 7→7→0 | 38→38→0 | 26→26→0 | 2→2→0 | 0→0→0 | 0→0→3 | 0→0→6 | 0→0→0 | 0→0→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 153→153→54 |
| global_while | 8→8→8 | 29→20→29 | 25→23→25 | 2→1→2 | 0→0→0 | 0→3→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 133→133→134 |
| struct_alias | 10→10→10 | 26→20→26 | 32→26→32 | 3→3→3 | 0→0→0 | 0→5→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 185→156→186 |
| struct_assume | 8→8→5 | 29→24→0 | 33→33→6 | 0→0→0 | 0→0→0 | 0→3→3 | 0→0→6 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 112→129→62 |
| struct_condvar | 7→7→7 | 34→26→0 | 28→26→3 | 1→1→1 | 0→0→5 | 0→3→3 | 0→0→4 | 0→1→0 | 0→3→3 | 0→0→2 | 0→0→2 | 0→0→1 | 0→0→2 | 0→0→0 | 186→164→69 |
| struct_dup | 7→7→9 | 28→20→0 | 28→24→2 | 2→2→2 | 0→0→5 | 0→5→5 | 0→0→7 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→0 | 166→147→88 |
| struct_empty | 7→7→7 | 30→24→30 | 34→32→34 | 1→1→1 | 0→0→0 | 0→4→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 142→133→143 |
| struct_init | 7→7→7 | 35→24→35 | 36→32→36 | 2→2→2 | 0→0→0 | 0→5→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 160→136→161 |
| struct_main | 7→7→5 | 24→20→0 | 25→23→2 | 1→1→0 | 0→0→4 | 0→3→5 | 0→0→3 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 0→0→1 | 124→120→59 |
| struct_malloc | 7→7→7 | 44→34→44 | 39→39→39 | 0→0→0 | 0→0→0 | 0→3→0 | 0→0→0 | 0→1→0 | 0→3→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 163→179→164 |
| struct_malloc2 | 8→8→8 | 33→24→33 | 32→32→32 | 1→1→1 | 0→0→0 | 0→4→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 120→135→121 |
| struct_multiple | 8→8→8 | 26→20→0 | 30→24→3 | 3→3→3 | 0→0→5 | 0→5→5 | 0→0→8 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→3 | 0→0→2 | 0→0→0 | 175→138→83 |
| struct_nested | 7→7→2 | 24→20→0 | 25→23→1 | 1→1→0 | 0→0→3 | 0→3→7 | 0→0→5 | 0→1→0 | 0→1→0 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→2 | 0→0→3 | 136→129→61 |
| struct_simple | 7→7→7 | 28→20→28 | 27→23→27 | 1→1→1 | 0→0→0 | 0→5→0 | 0→0→0 | 0→1→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 157→138→158 |
| struct_spin | 7→7→7 | 56→40→56 | 36→36→36 | 0→0→0 | 0→0→0 | 0→7→0 | 0→0→0 | 0→4→0 | 0→1→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 205→197→206 |
| struct_timedwait | 9→9→9 | 42→26→42 | 30→28→30 | 1→1→1 | 0→3→0 | 0→3→0 | 0→0→0 | 0→1→0 | 0→3→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 0→0→0 | 251→278→252 |
| unused_func | 8→8→6 | 26→20→0 | 25→23→3 | 2→1→1 | 0→0→5 | 0→3→4 | 0→0→1 | 0→1→0 | 0→1→0 | 0→0→1 | 0→0→0 | 0→0→0 | 0→0→1 | 0→0→0 | 119→119→66 |
| **TOTAL** | 492→231→539 | 2035→692→1543 | 1872→778→1521 | 128→39→119 | 0→9→106 | 0→113→278 | 0→0→276 | 0→34→0 | 0→37→18 | 0→0→82 | 0→0→71 | 0→0→75 | 0→0→82 | 0→0→35 | 10196→4483→11369 |

> **All Metrics** table shows all 15 metrics (including std\_arc, std\_rwlock, std\_condvar, move\_closure, arc\_clone, join\_handle, arc\_mutex\_combo) for each sample. Format: **Original → ConCrat → LLM** (or **Original → LLM** for negative samples).

## Aggregate Statistics

| Metric | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|--------|----------|---------|-----|:----------------:|:----------------:|
| unsafe | 492 | 231 | 539 | 53.0% | -9.6% |
| pthread | 2035 | 692 | 1543 | 66.0% | 24.2% |
| raw\_ptr | 1872 | 778 | 1521 | 58.4% | 18.8% |
| static\_mut | 128 | 39 | 119 | 69.5% | 7.0% |
| libc | 0 | 9 | 106 | — | — |
| std\_mutex | 0 | 113 | 278 | — | — |
| std\_arc | 0 | 0 | 276 | — | — |
| std\_rwlock | 0 | 34 | 0 | — | — |
| std\_condvar | 0 | 37 | 18 | — | — |
| std\_thread | 0 | 0 | 82 | — | — |
| move\_closure | 0 | 0 | 71 | — | — |
| arc\_clone | 0 | 0 | 75 | — | — |
| join\_handle | 0 | 0 | 82 | — | — |
| arc\_mutex\_combo | 0 | 0 | 35 | — | — |
| lines | 10196 | 4483 | 11369 | 56.0% | -11.5% |

| **Compile success** | — | 30/62 (48%) | 61/62 (98%) | | |

## Metric Categories Summary

Aggregate of metrics grouped by category (Lower is Better vs Higher is Better):

| Category | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|----------|----------|---------|-----|:----------------:|:----------------:|
| **Lower is Better** (∑unsafe, pthread, raw_ptr, static_mut, libc, lines) | 14723 | 6232 | 15197 | 57.7% | -3.2% |
| **Higher is Better** (∑std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 184 | 917 | — | — |

## Safety Features Adoption

| Example | Round | std::sync::Mutex | Arc<Mutex> | RwLock | Condvar | std::thread | join() |
|---------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| array_const | 2 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| array_main | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| array_simple | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| global_assume | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_assume2 | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_check | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| global_condvar | 2 | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| global_custom | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| global_main | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_nested | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| global_read | 6 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_rwlock | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| global_simple | 7 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_trylock | 1 | ·,L | ·,· | ·,· | ·,· | ·,L | ·,L |
| global_while | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_alias | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_assume | 1 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_condvar | 3 | ·,L | ·,· | C,· | C,L | ·,L | ·,L |
| struct_dup | 3 | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_empty | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_init | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_main | 5 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_malloc | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_malloc2 | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_multiple | c2rust | ·,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_nested | 6 | ·,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_simple | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_spin | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_timedwait | c2rust | ·,· | ·,· | C,· | C,· | ·,· | ·,· |
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

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 7 | tie |
| pthread | 32 | 20 | 32 | ConCrat |
| raw\_ptr | 37 | 23 | 37 | ConCrat |
| static\_mut | 2 | 1 | 2 | ConCrat |
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
| lines | 211 | 151 | 212 | ConCrat |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 289 | 202 | 290 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 9 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (4 calls); static mut variables remain: n1, num_mutex; 7 unsafe keyword(s) remain

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

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 10 | 10 | 10 | tie |
| pthread | 44 | 22 | 44 | ConCrat |
| raw\_ptr | 26 | 24 | 26 | ConCrat |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 0 | 6 | 0 | LLM |
| std\_mutex | 0 | 3 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 181 | 243 | 182 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 263 | 306 | 264 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (19 calls); static mut variables remain: n, m; 10 unsafe keyword(s) remain

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

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 12 | 12 | 12 | tie |
| pthread | 26 | 20 | 26 | ConCrat |
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
| lines | 142 | 156 | 143 | LLM |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 207 | 212 | 208 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (6 calls); static mut variables remain: n1, num_mutex; 12 unsafe keyword(s) remain

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

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 8 | tie |
| pthread | 28 | 22 | 28 | ConCrat |
| raw\_ptr | 22 | 22 | 22 | tie |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 0 | 0 | 0 | tie |
| std\_mutex | 0 | 1 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 3 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| move\_closure | 0 | 0 | 0 | tie |
| arc\_clone | 0 | 0 | 0 | tie |
| join\_handle | 0 | 0 | 0 | tie |
| arc\_mutex\_combo | 0 | 0 | 0 | tie |
| lines | 125 | 116 | 126 | ConCrat |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 185 | 169 | 186 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 5 | 0 |

**Remaining Issues:**

- **LLM**: static mut variables remain: n, lock; 8 unsafe keyword(s) remain

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

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 7 | tie |
| pthread | 30 | 24 | 30 | ConCrat |
| raw\_ptr | 34 | 32 | 34 | ConCrat |
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
| lines | 142 | 133 | 143 | ConCrat |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 214 | 197 | 215 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 6 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (6 calls); static mut variables remain: s1; 7 unsafe keyword(s) remain

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

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 7 | tie |
| pthread | 28 | 20 | 28 | ConCrat |
| raw\_ptr | 27 | 23 | 27 | ConCrat |
| static\_mut | 1 | 1 | 1 | tie |
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
| lines | 157 | 138 | 158 | ConCrat |

**Category Totals:**

| Category | Original | ConCrat | LLM |
|----------|:--------:|:-------:|:---:|
| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc, lines) | 220 | 189 | 221 |
| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | 0 | 7 | 0 |

**Remaining Issues:**

- **LLM**: Still uses pthread_mutex (6 calls); static mut variables remain: s; 7 unsafe keyword(s) remain

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
