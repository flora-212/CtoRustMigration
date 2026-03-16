# Concurrency Transformation Comparison Report

Three-way comparison: **Original** (c2rust output) vs **ConCrat** (automated transform) vs **LLM** (LLM-based rewrite)

## Summary Overview

| # | Example | Compiles (C / L) | unsafe | pthread | raw_ptr | static_mut | libc | std_mutex | std_thread | Lines |
|---|---------|:----------------:|--------|---------|---------|------------|------|-----------|------------|-------|
| 1 | [array_const](#array_const) | ✅ / ❌ | 7→7→6 | 29→22→18 | 44→32→30 | 2→1→0 | 74→38→36 | 0→5→9 | 0→0→0 | 160→107→111 |
| 2 | [array_main](#array_main) | ✅ / ✅ | 7→7→4 | 25→22→0 | 52→32→2 | 2→1→0 | 92→47→2 | 0→7→7 | 0→0→2 | 217→122→54 |
| 3 | [array_simple](#array_simple) | ✅ / ✅ | 7→7→5 | 25→22→21 | 52→32→14 | 4→2→4 | 99→54→32 | 0→7→0 | 0→0→0 | 222→121→183 |
| 4 | [global_assume](#global_assume) | ✅ / ✅ | 8→8→3 | 25→22→0 | 36→32→2 | 2→1→1 | 48→39→2 | 0→3→3 | 0→0→2 | 109→95→40 |
| 5 | [global_assume2](#global_assume2) | ✅ / ✅ | 8→8→6 | 25→22→6 | 36→32→10 | 2→1→1 | 50→41→18 | 0→3→3 | 0→0→0 | 110→95→67 |
| 6 | [global_check](#global_check) | ✅ / ❌ | 10→10→0 | 45→24→0 | 37→33→0 | 2→1→1 | 53→50→0 | 0→3→3 | 0→0→2 | 171→184→61 |
| 7 | [global_condvar](#global_condvar) | ✅ / ✅ | 7→7→6 | 44→30→0 | 44→36→0 | 4→2→2 | 78→59→0 | 0→3→4 | 0→0→1 | 193→155→52 |
| 8 | [global_custom](#global_custom) | ✅ / ❌ | 12→12→2 | 27→22→0 | 36→32→5 | 2→1→1 | 59→50→5 | 0→3→3 | 0→0→2 | 136→128→57 |
| 9 | [global_main](#global_main) | ✅ / ✅ | 7→7→7 | 25→22→25 | 36→32→36 | 2→1→2 | 48→39→47 | 0→3→0 | 0→0→0 | 108→93→108 |
| 10 | [global_nested](#global_nested) | ✅ / ✅ | 7→7→0 | 36→22→0 | 40→32→0 | 4→2→2 | 59→41→0 | 0→5→7 | 0→0→2 | 147→115→38 |
| 11 | [global_read](#global_read) | ✅ / ✅ | 7→7→5 | 25→22→6 | 36→32→10 | 3→2→2 | 50→41→17 | 0→3→3 | 0→0→0 | 107→92→59 |
| 12 | [global_rwlock](#global_rwlock) | ✅ / ✅ | 8→8→0 | 29→24→0 | 31→31→0 | 2→1→1 | 73→47→0 | 0→1→0 | 0→0→2 | 116→99→28 |
| 13 | [global_simple](#global_simple) | ✅ / ✅ | 7→7→5 | 25→22→6 | 36→32→9 | 4→2→3 | 53→44→17 | 0→3→3 | 0→0→0 | 111→96→59 |
| 14 | [global_trylock](#global_trylock) | ✅ / ✅ | 7→7→2 | 38→24→0 | 37→33→0 | 2→1→1 | 53→48→0 | 0→3→4 | 0→0→1 | 137→135→50 |
| 15 | [global_while](#global_while) | ✅ / ✅ | 8→8→0 | 30→22→0 | 36→32→0 | 2→1→0 | 52→43→0 | 0→3→5 | 0→0→1 | 125→112→46 |
| 16 | [struct_alias](#struct_alias) | ✅ / ❌ | 10→10→4 | 25→22→0 | 47→35→0 | 3→3→3 | 68→41→0 | 0→5→5 | 0→0→1 | 189→129→64 |
| 17 | [struct_assume](#struct_assume) | ✅ / ❌ | 8→8→6 | 31→26→6 | 45→43→17 | 0→0→0 | 48→47→25 | 0→3→3 | 0→0→0 | 113→115→82 |
| 18 | [struct_condvar](#struct_condvar) | ✅ / ✅ | 7→7→4 | 36→28→0 | 41→35→2 | 1→1→1 | 74→55→5 | 0→3→3 | 0→0→2 | 192→154→52 |
| 19 | [struct_dup](#struct_dup) | ✅ / ✅ | 7→7→7 | 28→22→28 | 40→32→40 | 2→2→2 | 64→46→64 | 0→5→0 | 0→0→0 | 163→125→180 |
| 20 | [struct_empty](#struct_empty) | ✅ / ❌ | 7→7→4 | 31→26→0 | 48→42→6 | 1→1→1 | 58→48→7 | 0→4→4 | 0→0→2 | 139→118→54 |
| 21 | [struct_init](#struct_init) | ✅ / ❌ | 7→7→5 | 35→26→15 | 56→42→15 | 2→2→0 | 51→47→14 | 0→5→5 | 0→0→0 | 155→124→87 |
| 22 | [struct_main](#struct_main) | ✅ / ✅ | 7→7→4 | 25→22→0 | 36→32→2 | 1→1→1 | 48→39→6 | 0→3→3 | 0→0→2 | 119→101→48 |
| 23 | [struct_malloc](#struct_malloc) | ✅ / ✅ | 7→7→0 | 49→36→0 | 56→49→0 | 0→0→0 | 70→68→0 | 0→3→5 | 0→0→2 | 184→179→41 |
| 24 | [struct_malloc2](#struct_malloc2) | ✅ / ❌ | 8→8→6 | 35→26→15 | 52→48→12 | 1→1→1 | 54→52→13 | 0→4→4 | 0→0→0 | 127→127→87 |
| 25 | [struct_multiple](#struct_multiple) | ✅ / ❌ | 8→8→3 | 25→22→0 | 45→33→3 | 3→3→3 | 68→41→3 | 0→5→5 | 0→0→2 | 181→119→54 |
| 26 | [struct_nested](#struct_nested) | ✅ / ❌ | 7→7→0 | 25→22→0 | 36→32→0 | 1→1→0 | 50→41→0 | 0→3→5 | 0→0→2 | 132→114→48 |
| 27 | [struct_simple](#struct_simple) | ✅ / ❌ | 7→7→0 | 28→22→0 | 40→32→0 | 1→1→0 | 64→46→0 | 0→5→5 | 0→0→1 | 153→115→52 |
| 28 | [struct_spin](#struct_spin) | ✅ / ❌ | 7→7→5 | 57→42→6 | 53→45→15 | 0→0→0 | 74→70→22 | 0→7→7 | 0→0→0 | 200→176→78 |
| 29 | [struct_timedwait](#struct_timedwait) | ✅ / ❌ | 9→9→4 | 48→28→0 | 53→37→1 | 1→1→0 | 88→73→5 | 0→3→2 | 0→0→2 | 249→218→95 |
| 30 | [unused_func](#unused_func) | ✅ / ✅ | 8→8→0 | 27→22→0 | 36→32→0 | 2→1→1 | 48→39→0 | 0→3→3 | 0→0→2 | 111→98→32 |
| | **TOTAL** | 30/30 / 17/30 | 231→231→103 | 958→736→152 | 1273→1054→231 | 58→38→34 | 1868→1434→340 | 0→116→113 | 0→0→33 | 4576→3761→2067 |

> **Reading the table**: Each metric cell shows **Original → ConCrat → LLM**. Compiles column shows **ConCrat / LLM**.

## Aggregate Statistics

| Metric | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|--------|----------|---------|-----|:----------------:|:----------------:|
| unsafe | 231 | 231 | 103 | 0.0% | 55.4% |
| pthread | 958 | 736 | 152 | 23.2% | 84.1% |
| raw\_ptr | 1273 | 1054 | 231 | 17.2% | 81.9% |
| static\_mut | 58 | 38 | 34 | 34.5% | 41.4% |
| libc | 1868 | 1434 | 340 | 23.2% | 81.8% |
| std\_mutex | 0 | 116 | 113 | — | — |
| std\_arc | 0 | 0 | 83 | — | — |
| std\_rwlock | 0 | 35 | 6 | — | — |
| std\_condvar | 0 | 38 | 12 | — | — |
| std\_thread | 0 | 0 | 33 | — | — |
| lines | 4576 | 3761 | 2067 | 17.8% | 54.8% |

| **Compile success** | — | 30/30 (100%) | 17/30 (57%) | | |

## Safety Features Adoption

| Example | std::sync::Mutex | Arc<Mutex> | RwLock | Condvar | std::thread | join() |
|---------|:---:|:---:|:---:|:---:|:---:|:---:|
| array_const | C,L | ·,· | C,· | C,· | ·,· | ·,· |
| array_main | C,L | ·,· | C,· | C,· | ·,L | ·,L |
| array_simple | C,· | ·,· | C,· | C,· | ·,· | ·,· |
| global_assume | C,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_assume2 | C,L | ·,· | C,· | C,· | ·,· | ·,· |
| global_check | C,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_condvar | C,L | ·,L | C,· | C,L | ·,L | ·,L |
| global_custom | C,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_main | C,· | ·,· | C,· | C,· | ·,· | ·,· |
| global_nested | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_read | C,L | ·,· | C,· | C,· | ·,· | ·,· |
| global_rwlock | C,· | ·,· | C,L | C,· | ·,L | ·,L |
| global_simple | C,L | ·,· | C,· | C,· | ·,· | ·,· |
| global_trylock | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_while | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_alias | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_assume | C,L | ·,· | C,· | C,· | ·,· | ·,· |
| struct_condvar | C,L | ·,· | C,· | C,L | ·,L | ·,L |
| struct_dup | C,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_empty | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_init | C,L | ·,L | C,· | C,· | ·,· | ·,· |
| struct_main | C,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_malloc | C,L | ·,· | C,· | C,L | ·,L | ·,L |
| struct_malloc2 | C,L | ·,· | C,· | C,· | ·,· | ·,· |
| struct_multiple | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_nested | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_simple | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_spin | C,L | ·,· | C,L | C,· | ·,· | ·,· |
| struct_timedwait | C,L | ·,· | C,· | C,L | ·,L | ·,L |
| unused_func | C,L | ·,· | C,· | C,· | ·,L | ·,L |

> **C** = ConCrat uses it, **L** = LLM uses it, **·** = not used

## Per-Example Details

### array_const

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 6 | LLM |
| pthread | 29 | 22 | 18 | LLM |
| raw\_ptr | 44 | 32 | 30 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 74 | 38 | 36 | LLM |
| std\_mutex | 0 | 5 | 9 | LLM |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 160 | 107 | 111 | ConCrat |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex; 7 unsafe keyword(s) remain
- **LLM**: 6 unsafe keyword(s) remain

---

### array_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 25 | 22 | 0 | LLM |
| raw\_ptr | 52 | 32 | 2 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 92 | 47 | 2 | LLM |
| std\_mutex | 0 | 7 | 7 | tie |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 217 | 122 | 54 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex; 7 unsafe keyword(s) remain
- **LLM**: 4 unsafe keyword(s) remain

---

### array_simple

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 5 | LLM |
| pthread | 25 | 22 | 21 | LLM |
| raw\_ptr | 52 | 32 | 14 | LLM |
| static\_mut | 4 | 2 | 4 | ConCrat |
| libc | 99 | 54 | 32 | LLM |
| std\_mutex | 0 | 7 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 222 | 121 | 183 | ConCrat |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: n3, num_mutex; 7 unsafe keyword(s) remain
- **LLM**: Still uses pthread_mutex (4 calls); static mut variables remain: n1, n2, n3, num_mutex; 5 unsafe keyword(s) remain

---

### global_assume

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 3 | LLM |
| pthread | 25 | 22 | 0 | LLM |
| raw\_ptr | 36 | 32 | 2 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 48 | 39 | 2 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 109 | 95 | 40 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex; 8 unsafe keyword(s) remain
- **LLM**: static mut variables remain: N1; 3 unsafe keyword(s) remain

---

### global_assume2

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 6 | LLM |
| pthread | 25 | 22 | 6 | LLM |
| raw\_ptr | 36 | 32 | 10 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 50 | 41 | 18 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 110 | 95 | 67 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex; 8 unsafe keyword(s) remain
- **LLM**: static mut variables remain: n1; 6 unsafe keyword(s) remain

---

### global_check

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 10 | 10 | 0 | LLM |
| pthread | 45 | 24 | 0 | LLM |
| raw\_ptr | 37 | 33 | 0 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 53 | 50 | 0 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 171 | 184 | 61 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: m; 10 unsafe keyword(s) remain
- **LLM**: static mut variables remain: N

---

### global_condvar

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 6 | LLM |
| pthread | 44 | 30 | 0 | LLM |
| raw\_ptr | 44 | 36 | 0 | LLM |
| static\_mut | 4 | 2 | 2 | tie |
| libc | 78 | 59 | 0 | LLM |
| std\_mutex | 0 | 3 | 4 | LLM |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 4 | LLM |
| std\_thread | 0 | 0 | 1 | LLM |
| lines | 193 | 155 | 52 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex, cond; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: N1, N2; 6 unsafe keyword(s) remain

---

### global_custom

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 12 | 12 | 2 | LLM |
| pthread | 27 | 22 | 0 | LLM |
| raw\_ptr | 36 | 32 | 5 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 59 | 50 | 5 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 136 | 128 | 57 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex; 12 unsafe keyword(s) remain
- **LLM**: static mut variables remain: n1; 2 unsafe keyword(s) remain

---

### global_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 7 | tie |
| pthread | 25 | 22 | 25 | ConCrat |
| raw\_ptr | 36 | 32 | 36 | ConCrat |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 48 | 39 | 47 | ConCrat |
| std\_mutex | 0 | 3 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 108 | 93 | 108 | ConCrat |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex; 7 unsafe keyword(s) remain
- **LLM**: Still uses pthread_mutex (4 calls); static mut variables remain: n1, num_mutex; 7 unsafe keyword(s) remain

---

### global_nested

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 0 | LLM |
| pthread | 36 | 22 | 0 | LLM |
| raw\_ptr | 40 | 32 | 0 | LLM |
| static\_mut | 4 | 2 | 2 | tie |
| libc | 59 | 41 | 0 | LLM |
| std\_mutex | 0 | 5 | 7 | LLM |
| std\_arc | 0 | 0 | 9 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 147 | 115 | 38 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: n1_mutex, n2_mutex; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: N1, N2

---

### global_read

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 5 | LLM |
| pthread | 25 | 22 | 6 | LLM |
| raw\_ptr | 36 | 32 | 10 | LLM |
| static\_mut | 3 | 2 | 2 | tie |
| libc | 50 | 41 | 17 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 107 | 92 | 59 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: n2, num_mutex; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: n1, n2; 5 unsafe keyword(s) remain

---

### global_rwlock

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 0 | LLM |
| pthread | 29 | 24 | 0 | LLM |
| raw\_ptr | 31 | 31 | 0 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 73 | 47 | 0 | LLM |
| std\_mutex | 0 | 1 | 0 | ConCrat |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 3 | 3 | tie |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 116 | 99 | 28 | LLM |

**Remaining Issues:**

- **ConCrat**: static mut variables remain: lock; 8 unsafe keyword(s) remain
- **LLM**: static mut variables remain: N

---

### global_simple

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 5 | LLM |
| pthread | 25 | 22 | 6 | LLM |
| raw\_ptr | 36 | 32 | 9 | LLM |
| static\_mut | 4 | 2 | 3 | ConCrat |
| libc | 53 | 44 | 17 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 111 | 96 | 59 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: n3, num_mutex; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: n1, n2, n3; 5 unsafe keyword(s) remain

---

### global_trylock

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 2 | LLM |
| pthread | 38 | 24 | 0 | LLM |
| raw\_ptr | 37 | 33 | 0 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 53 | 48 | 0 | LLM |
| std\_mutex | 0 | 3 | 4 | LLM |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| lines | 137 | 135 | 50 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: m; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: N; 2 unsafe keyword(s) remain

---

### global_while

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 0 | LLM |
| pthread | 30 | 22 | 0 | LLM |
| raw\_ptr | 36 | 32 | 0 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 52 | 43 | 0 | LLM |
| std\_mutex | 0 | 3 | 5 | LLM |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| lines | 125 | 112 | 46 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex; 8 unsafe keyword(s) remain

---

### struct_alias

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 10 | 10 | 4 | LLM |
| pthread | 25 | 22 | 0 | LLM |
| raw\_ptr | 47 | 35 | 0 | LLM |
| static\_mut | 3 | 3 | 3 | tie |
| libc | 68 | 41 | 0 | LLM |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| lines | 189 | 129 | 64 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: s1, s2, s3; 10 unsafe keyword(s) remain
- **LLM**: static mut variables remain: S1, S2, S3; 4 unsafe keyword(s) remain

---

### struct_assume

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 6 | LLM |
| pthread | 31 | 26 | 6 | LLM |
| raw\_ptr | 45 | 43 | 17 | LLM |
| static\_mut | 0 | 0 | 0 | tie |
| libc | 48 | 47 | 25 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 113 | 115 | 82 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (3 calls); 8 unsafe keyword(s) remain
- **LLM**: 6 unsafe keyword(s) remain

---

### struct_condvar

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 36 | 28 | 0 | LLM |
| raw\_ptr | 41 | 35 | 2 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 74 | 55 | 5 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 192 | 154 | 52 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: s; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: S; 4 unsafe keyword(s) remain

---

### struct_dup

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 7 | tie |
| pthread | 28 | 22 | 28 | ConCrat |
| raw\_ptr | 40 | 32 | 40 | ConCrat |
| static\_mut | 2 | 2 | 2 | tie |
| libc | 64 | 46 | 64 | ConCrat |
| std\_mutex | 0 | 5 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 163 | 125 | 180 | ConCrat |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: s1, s2; 7 unsafe keyword(s) remain
- **LLM**: Still uses pthread_mutex (6 calls); static mut variables remain: s1, s2; 7 unsafe keyword(s) remain

---

### struct_empty

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 31 | 26 | 0 | LLM |
| raw\_ptr | 48 | 42 | 6 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 58 | 48 | 7 | LLM |
| std\_mutex | 0 | 4 | 4 | tie |
| std\_arc | 0 | 0 | 9 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 139 | 118 | 54 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (3 calls); static mut variables remain: s1; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: s1; 4 unsafe keyword(s) remain

---

### struct_init

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 5 | LLM |
| pthread | 35 | 26 | 15 | LLM |
| raw\_ptr | 56 | 42 | 15 | LLM |
| static\_mut | 2 | 2 | 0 | LLM |
| libc | 51 | 47 | 14 | LLM |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 12 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 155 | 124 | 87 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (3 calls); static mut variables remain: s1, s2; 7 unsafe keyword(s) remain
- **LLM**: 5 unsafe keyword(s) remain

---

### struct_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 25 | 22 | 0 | LLM |
| raw\_ptr | 36 | 32 | 2 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 48 | 39 | 6 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 119 | 101 | 48 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: s; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: S; 4 unsafe keyword(s) remain

---

### struct_malloc

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 0 | LLM |
| pthread | 49 | 36 | 0 | LLM |
| raw\_ptr | 56 | 49 | 0 | LLM |
| static\_mut | 0 | 0 | 0 | tie |
| libc | 70 | 68 | 0 | LLM |
| std\_mutex | 0 | 3 | 5 | LLM |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 184 | 179 | 41 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (3 calls); 7 unsafe keyword(s) remain

---

### struct_malloc2

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 6 | LLM |
| pthread | 35 | 26 | 15 | LLM |
| raw\_ptr | 52 | 48 | 12 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 54 | 52 | 13 | LLM |
| std\_mutex | 0 | 4 | 4 | tie |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 127 | 127 | 87 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (3 calls); static mut variables remain: x; 8 unsafe keyword(s) remain
- **LLM**: static mut variables remain: x; 6 unsafe keyword(s) remain

---

### struct_multiple

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 3 | LLM |
| pthread | 25 | 22 | 0 | LLM |
| raw\_ptr | 45 | 33 | 3 | LLM |
| static\_mut | 3 | 3 | 3 | tie |
| libc | 68 | 41 | 3 | LLM |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 5 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 181 | 119 | 54 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: s1, s2, s3; 8 unsafe keyword(s) remain
- **LLM**: static mut variables remain: s1, s2, s3; 3 unsafe keyword(s) remain

---

### struct_nested

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 0 | LLM |
| pthread | 25 | 22 | 0 | LLM |
| raw\_ptr | 36 | 32 | 0 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 50 | 41 | 0 | LLM |
| std\_mutex | 0 | 3 | 5 | LLM |
| std\_arc | 0 | 0 | 6 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 132 | 114 | 48 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: s; 7 unsafe keyword(s) remain

---

### struct_simple

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 0 | LLM |
| pthread | 28 | 22 | 0 | LLM |
| raw\_ptr | 40 | 32 | 0 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 64 | 46 | 0 | LLM |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| lines | 153 | 115 | 52 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: s; 7 unsafe keyword(s) remain

---

### struct_spin

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 5 | LLM |
| pthread | 57 | 42 | 6 | LLM |
| raw\_ptr | 53 | 45 | 15 | LLM |
| static\_mut | 0 | 0 | 0 | tie |
| libc | 74 | 70 | 22 | LLM |
| std\_mutex | 0 | 7 | 7 | tie |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 4 | 3 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 200 | 176 | 78 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (3 calls); 7 unsafe keyword(s) remain
- **LLM**: 5 unsafe keyword(s) remain

---

### struct_timedwait

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 9 | 9 | 4 | LLM |
| pthread | 48 | 28 | 0 | LLM |
| raw\_ptr | 53 | 37 | 1 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 88 | 73 | 5 | LLM |
| std\_mutex | 0 | 3 | 2 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 2 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 249 | 218 | 95 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: s; 9 unsafe keyword(s) remain
- **LLM**: 4 unsafe keyword(s) remain

---

### unused_func

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 0 | LLM |
| pthread | 27 | 22 | 0 | LLM |
| raw\_ptr | 36 | 32 | 0 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 48 | 39 | 0 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 111 | 98 | 32 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex; 8 unsafe keyword(s) remain
- **LLM**: static mut variables remain: N1

---
