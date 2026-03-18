# Concurrency Transformation Comparison Report

Three-way comparison: **Original** (c2rust output) vs **ConCrat** (automated transform) vs **LLM** (LLM-based rewrite)

## Summary Overview

| # | Example | Compiles (C / L) | unsafe | pthread | raw_ptr | static_mut | libc | std_mutex | std_thread | Lines |
|---|---------|:----------------:|--------|---------|---------|------------|------|-----------|------------|-------|
| 1 | [array_const](#array_const) | ✅ / ❌ | 7→7→4 | 29→22→12 | 44→32→4 | 2→1→1 | 74→38→4 | 0→5→5 | 0→0→0 | 160→107→56 |
| 2 | [array_main](#array_main) | ✅ / ✅ | 7→7→5 | 25→22→21 | 52→32→13 | 2→1→2 | 92→47→24 | 0→7→0 | 0→0→0 | 217→122→176 |
| 3 | [array_simple](#array_simple) | ✅ / ✅ | 7→7→2 | 25→22→0 | 52→32→0 | 4→2→3 | 99→54→0 | 0→7→7 | 0→0→2 | 222→121→44 |
| 4 | [global_assume](#global_assume) | ✅ / ✅ | 8→8→0 | 25→22→0 | 36→32→0 | 2→1→0 | 48→39→0 | 0→3→5 | 0→0→2 | 109→95→30 |
| 5 | [global_assume2](#global_assume2) | ✅ / ✅ | 8→8→8 | 25→22→25 | 36→32→36 | 2→1→2 | 50→41→49 | 0→3→0 | 0→0→0 | 110→95→110 |
| 6 | [global_check](#global_check) | ✅ / ❌ | 10→10→0 | 45→24→0 | 37→33→0 | 2→1→1 | 53→50→0 | 0→3→3 | 0→0→2 | 171→184→61 |
| 7 | [global_condvar](#global_condvar) | ✅ / ✅ | 7→7→7 | 44→30→17 | 44→36→28 | 4→2→2 | 78→59→28 | 0→3→3 | 0→0→0 | 193→155→80 |
| 8 | [global_custom](#global_custom) | ✅ / ✅ | 12→12→0 | 27→22→0 | 36→32→0 | 2→1→1 | 59→50→0 | 0→3→3 | 0→0→2 | 136→128→50 |
| 9 | [global_main](#global_main) | ✅ / ✅ | 7→7→4 | 25→22→6 | 36→32→13 | 2→1→1 | 48→39→20 | 0→3→4 | 0→0→0 | 108→93→67 |
| 10 | [global_nested](#global_nested) | ✅ / ✅ | 7→7→0 | 36→22→0 | 40→32→0 | 4→2→2 | 59→41→0 | 0→5→7 | 0→0→2 | 147→115→61 |
| 11 | [global_read](#global_read) | ✅ / ✅ | 7→7→1 | 25→22→0 | 36→32→0 | 3→2→2 | 50→41→0 | 0→3→3 | 0→0→2 | 107→92→31 |
| 12 | [global_rwlock](#global_rwlock) | ✅ / ✅ | 8→8→0 | 29→24→0 | 31→31→0 | 2→1→0 | 73→47→0 | 0→1→0 | 0→0→2 | 116→99→35 |
| 13 | [global_simple](#global_simple) | ✅ / ✅ | 7→7→3 | 25→22→0 | 36→32→0 | 4→2→3 | 53→44→0 | 0→3→4 | 0→0→1 | 111→96→41 |
| 14 | [global_trylock](#global_trylock) | ✅ / ✅ | 7→7→2 | 38→24→0 | 37→33→0 | 2→1→1 | 53→48→0 | 0→3→4 | 0→0→1 | 137→135→48 |
| 15 | [global_while](#global_while) | ✅ / ✅ | 8→8→6 | 30→22→0 | 36→32→0 | 2→1→1 | 52→43→0 | 0→3→3 | 0→0→2 | 125→112→41 |
| 16 | [struct_alias](#struct_alias) | ✅ / ✅ | 10→10→8 | 25→22→23 | 47→35→31 | 3→3→3 | 68→41→30 | 0→5→0 | 0→0→0 | 189→129→173 |
| 17 | [struct_assume](#struct_assume) | ✅ / ❌ | 8→8→5 | 31→26→6 | 45→43→8 | 0→0→0 | 48→47→12 | 0→3→3 | 0→0→0 | 113→115→65 |
| 18 | [struct_condvar](#struct_condvar) | ✅ / ❌ | 7→7→4 | 36→28→0 | 41→35→2 | 1→1→0 | 74→55→5 | 0→3→3 | 0→0→2 | 192→154→48 |
| 19 | [struct_dup](#struct_dup) | ✅ / ✅ | 7→7→5 | 28→22→24 | 40→32→13 | 2→2→2 | 64→46→27 | 0→5→0 | 0→0→0 | 163→125→151 |
| 20 | [struct_empty](#struct_empty) | ✅ / ❌ | 7→7→4 | 31→26→0 | 48→42→5 | 1→1→1 | 58→48→5 | 0→4→4 | 0→0→2 | 139→118→48 |
| 21 | [struct_init](#struct_init) | ✅ / ❌ | 7→7→0 | 35→26→0 | 56→42→0 | 2→2→0 | 51→47→0 | 0→5→5 | 0→0→2 | 155→124→50 |
| 22 | [struct_main](#struct_main) | ✅ / ✅ | 7→7→4 | 25→22→0 | 36→32→2 | 1→1→1 | 48→39→5 | 0→3→3 | 0→0→2 | 119→101→48 |
| 23 | [struct_malloc](#struct_malloc) | ✅ / ✅ | 7→7→5 | 49→36→6 | 56→49→13 | 0→0→0 | 70→68→18 | 0→3→5 | 0→0→0 | 184→179→75 |
| 24 | [struct_malloc2](#struct_malloc2) | ✅ / ❌ | 8→8→4 | 35→26→0 | 52→48→2 | 1→1→1 | 54→52→6 | 0→4→4 | 0→0→2 | 127→127→57 |
| 25 | [struct_multiple](#struct_multiple) | ✅ / ❌ | 8→8→3 | 25→22→0 | 45→33→2 | 3→3→3 | 68→41→2 | 0→5→5 | 0→0→2 | 181→119→54 |
| 26 | [struct_nested](#struct_nested) | ✅ / ❌ | 7→7→5 | 25→22→15 | 36→32→28 | 1→1→1 | 50→41→36 | 0→3→3 | 0→0→0 | 132→114→82 |
| 27 | [struct_simple](#struct_simple) | ✅ / ✅ | 7→7→0 | 28→22→0 | 40→32→0 | 1→1→0 | 64→46→0 | 0→5→7 | 0→0→2 | 153→115→53 |
| 28 | [struct_spin](#struct_spin) | ✅ / ✅ | 7→7→0 | 57→42→0 | 53→45→0 | 0→0→0 | 74→70→0 | 0→7→7 | 0→0→2 | 200→176→56 |
| 29 | [struct_timedwait](#struct_timedwait) | ✅ / ❌ | 9→9→9 | 48→28→12 | 53→37→21 | 1→1→1 | 88→73→51 | 0→3→3 | 0→0→0 | 249→218→193 |
| 30 | [unused_func](#unused_func) | ✅ / ✅ | 8→8→0 | 27→22→0 | 36→32→0 | 2→1→1 | 48→39→0 | 0→3→3 | 0→0→2 | 111→98→32 |
| | **TOTAL** | 30/30 / 20/30 | 231→231→98 | 958→736→167 | 1273→1054→221 | 58→38→36 | 1868→1434→322 | 0→116→106 | 0→0→36 | 4576→3761→2116 |

> **Reading the table**: Each metric cell shows **Original → ConCrat → LLM**. Compiles column shows **ConCrat / LLM**.

## Aggregate Statistics

| Metric | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |
|--------|----------|---------|-----|:----------------:|:----------------:|
| unsafe | 231 | 231 | 98 | 0.0% | 57.6% |
| pthread | 958 | 736 | 167 | 23.2% | 82.6% |
| raw\_ptr | 1273 | 1054 | 221 | 17.2% | 82.6% |
| static\_mut | 58 | 38 | 36 | 34.5% | 37.9% |
| libc | 1868 | 1434 | 322 | 23.2% | 82.8% |
| std\_mutex | 0 | 116 | 106 | — | — |
| std\_arc | 0 | 0 | 105 | — | — |
| std\_rwlock | 0 | 35 | 8 | — | — |
| std\_condvar | 0 | 38 | 12 | — | — |
| std\_thread | 0 | 0 | 36 | — | — |
| lines | 4576 | 3761 | 2116 | 17.8% | 53.8% |

| **Compile success** | — | 30/30 (100%) | 20/30 (67%) | | |

## Safety Features Adoption

| Example | std::sync::Mutex | Arc<Mutex> | RwLock | Condvar | std::thread | join() |
|---------|:---:|:---:|:---:|:---:|:---:|:---:|
| array_const | C,L | ·,· | C,· | C,· | ·,· | ·,· |
| array_main | C,· | ·,· | C,· | C,· | ·,· | ·,· |
| array_simple | C,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_assume | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_assume2 | C,· | ·,· | C,· | C,· | ·,· | ·,· |
| global_check | C,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_condvar | C,L | ·,· | C,· | C,L | ·,· | ·,· |
| global_custom | C,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_main | C,L | ·,L | C,· | C,· | ·,· | ·,· |
| global_nested | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_read | C,L | ·,· | C,· | C,· | ·,L | ·,L |
| global_rwlock | C,· | ·,· | C,L | C,· | ·,L | ·,L |
| global_simple | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_trylock | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| global_while | C,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_alias | C,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_assume | C,L | ·,· | C,· | C,· | ·,· | ·,· |
| struct_condvar | C,L | ·,· | C,· | C,L | ·,L | ·,L |
| struct_dup | C,· | ·,· | C,· | C,· | ·,· | ·,· |
| struct_empty | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_init | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_main | C,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_malloc | C,L | ·,L | C,· | C,L | ·,· | ·,· |
| struct_malloc2 | C,L | ·,· | C,· | C,· | ·,L | ·,L |
| struct_multiple | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_nested | C,L | ·,L | C,· | C,· | ·,· | ·,· |
| struct_simple | C,L | ·,L | C,· | C,· | ·,L | ·,L |
| struct_spin | C,L | ·,L | C,L | C,· | ·,L | ·,L |
| struct_timedwait | C,L | ·,· | C,· | C,L | ·,· | ·,· |
| unused_func | C,L | ·,· | C,· | C,· | ·,L | ·,L |

> **C** = ConCrat uses it, **L** = LLM uses it, **·** = not used

## Per-Example Details

### array_const

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 29 | 22 | 12 | LLM |
| raw\_ptr | 44 | 32 | 4 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 74 | 38 | 4 | LLM |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 160 | 107 | 56 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex; 7 unsafe keyword(s) remain
- **LLM**: Still uses pthread_mutex (2 calls); static mut variables remain: N1; 4 unsafe keyword(s) remain

---

### array_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 5 | LLM |
| pthread | 25 | 22 | 21 | LLM |
| raw\_ptr | 52 | 32 | 13 | LLM |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 92 | 47 | 24 | LLM |
| std\_mutex | 0 | 7 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 217 | 122 | 176 | ConCrat |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex; 7 unsafe keyword(s) remain
- **LLM**: Still uses pthread_mutex (4 calls); static mut variables remain: n1, num_mutex; 5 unsafe keyword(s) remain

---

### array_simple

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 2 | LLM |
| pthread | 25 | 22 | 0 | LLM |
| raw\_ptr | 52 | 32 | 0 | LLM |
| static\_mut | 4 | 2 | 3 | ConCrat |
| libc | 99 | 54 | 0 | LLM |
| std\_mutex | 0 | 7 | 7 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 222 | 121 | 44 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: n3, num_mutex; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: N1, N2, N3; 2 unsafe keyword(s) remain

---

### global_assume

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 0 | LLM |
| pthread | 25 | 22 | 0 | LLM |
| raw\_ptr | 36 | 32 | 0 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 48 | 39 | 0 | LLM |
| std\_mutex | 0 | 3 | 5 | LLM |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 109 | 95 | 30 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex; 8 unsafe keyword(s) remain

---

### global_assume2

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 8 | tie |
| pthread | 25 | 22 | 25 | ConCrat |
| raw\_ptr | 36 | 32 | 36 | ConCrat |
| static\_mut | 2 | 1 | 2 | ConCrat |
| libc | 50 | 41 | 49 | ConCrat |
| std\_mutex | 0 | 3 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 110 | 95 | 110 | ConCrat |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex; 8 unsafe keyword(s) remain
- **LLM**: Still uses pthread_mutex (4 calls); static mut variables remain: n1, num_mutex; 8 unsafe keyword(s) remain

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
| unsafe | 7 | 7 | 7 | tie |
| pthread | 44 | 30 | 17 | LLM |
| raw\_ptr | 44 | 36 | 28 | LLM |
| static\_mut | 4 | 2 | 2 | tie |
| libc | 78 | 59 | 28 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 193 | 155 | 80 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex, cond; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: n1, n2; 7 unsafe keyword(s) remain

---

### global_custom

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 12 | 12 | 0 | LLM |
| pthread | 27 | 22 | 0 | LLM |
| raw\_ptr | 36 | 32 | 0 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 59 | 50 | 0 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 136 | 128 | 50 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex; 12 unsafe keyword(s) remain
- **LLM**: static mut variables remain: N1

---

### global_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 25 | 22 | 6 | LLM |
| raw\_ptr | 36 | 32 | 13 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 48 | 39 | 20 | LLM |
| std\_mutex | 0 | 3 | 4 | LLM |
| std\_arc | 0 | 0 | 8 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 108 | 93 | 67 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: n1; 4 unsafe keyword(s) remain

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
| lines | 147 | 115 | 61 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: n1_mutex, n2_mutex; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: N1, N2

---

### global_read

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 1 | LLM |
| pthread | 25 | 22 | 0 | LLM |
| raw\_ptr | 36 | 32 | 0 | LLM |
| static\_mut | 3 | 2 | 2 | tie |
| libc | 50 | 41 | 0 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 107 | 92 | 31 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: n2, num_mutex; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: N1, N2; 1 unsafe keyword(s) remain

---

### global_rwlock

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 0 | LLM |
| pthread | 29 | 24 | 0 | LLM |
| raw\_ptr | 31 | 31 | 0 | LLM |
| static\_mut | 2 | 1 | 0 | LLM |
| libc | 73 | 47 | 0 | LLM |
| std\_mutex | 0 | 1 | 0 | ConCrat |
| std\_arc | 0 | 0 | 7 | LLM |
| std\_rwlock | 0 | 3 | 5 | LLM |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 116 | 99 | 35 | LLM |

**Remaining Issues:**

- **ConCrat**: static mut variables remain: lock; 8 unsafe keyword(s) remain

---

### global_simple

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 3 | LLM |
| pthread | 25 | 22 | 0 | LLM |
| raw\_ptr | 36 | 32 | 0 | LLM |
| static\_mut | 4 | 2 | 3 | ConCrat |
| libc | 53 | 44 | 0 | LLM |
| std\_mutex | 0 | 3 | 4 | LLM |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 1 | LLM |
| lines | 111 | 96 | 41 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: n3, num_mutex; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: N1, N2, N3; 3 unsafe keyword(s) remain

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
| lines | 137 | 135 | 48 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: m; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: N; 2 unsafe keyword(s) remain

---

### global_while

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 6 | LLM |
| pthread | 30 | 22 | 0 | LLM |
| raw\_ptr | 36 | 32 | 0 | LLM |
| static\_mut | 2 | 1 | 1 | tie |
| libc | 52 | 43 | 0 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 1 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 125 | 112 | 41 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: num_mutex; 8 unsafe keyword(s) remain
- **LLM**: static mut variables remain: N1; 6 unsafe keyword(s) remain

---

### struct_alias

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 10 | 10 | 8 | LLM |
| pthread | 25 | 22 | 23 | ConCrat |
| raw\_ptr | 47 | 35 | 31 | LLM |
| static\_mut | 3 | 3 | 3 | tie |
| libc | 68 | 41 | 30 | LLM |
| std\_mutex | 0 | 5 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 189 | 129 | 173 | ConCrat |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: s1, s2, s3; 10 unsafe keyword(s) remain
- **LLM**: Still uses pthread_mutex (4 calls); static mut variables remain: s1, s2, s3; 8 unsafe keyword(s) remain

---

### struct_assume

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 5 | LLM |
| pthread | 31 | 26 | 6 | LLM |
| raw\_ptr | 45 | 43 | 8 | LLM |
| static\_mut | 0 | 0 | 0 | tie |
| libc | 48 | 47 | 12 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 113 | 115 | 65 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (3 calls); 8 unsafe keyword(s) remain
- **LLM**: 5 unsafe keyword(s) remain

---

### struct_condvar

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 36 | 28 | 0 | LLM |
| raw\_ptr | 41 | 35 | 2 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 74 | 55 | 5 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 192 | 154 | 48 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: s; 7 unsafe keyword(s) remain
- **LLM**: 4 unsafe keyword(s) remain

---

### struct_dup

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 5 | LLM |
| pthread | 28 | 22 | 24 | ConCrat |
| raw\_ptr | 40 | 32 | 13 | LLM |
| static\_mut | 2 | 2 | 2 | tie |
| libc | 64 | 46 | 27 | LLM |
| std\_mutex | 0 | 5 | 0 | ConCrat |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 163 | 125 | 151 | ConCrat |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: s1, s2; 7 unsafe keyword(s) remain
- **LLM**: Still uses pthread_mutex (6 calls); static mut variables remain: s1, s2; 5 unsafe keyword(s) remain

---

### struct_empty

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 31 | 26 | 0 | LLM |
| raw\_ptr | 48 | 42 | 5 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 58 | 48 | 5 | LLM |
| std\_mutex | 0 | 4 | 4 | tie |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 139 | 118 | 48 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (3 calls); static mut variables remain: s1; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: s1; 4 unsafe keyword(s) remain

---

### struct_init

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 0 | LLM |
| pthread | 35 | 26 | 0 | LLM |
| raw\_ptr | 56 | 42 | 0 | LLM |
| static\_mut | 2 | 2 | 0 | LLM |
| libc | 51 | 47 | 0 | LLM |
| std\_mutex | 0 | 5 | 5 | tie |
| std\_arc | 0 | 0 | 8 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 155 | 124 | 50 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (3 calls); static mut variables remain: s1, s2; 7 unsafe keyword(s) remain

---

### struct_main

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 4 | LLM |
| pthread | 25 | 22 | 0 | LLM |
| raw\_ptr | 36 | 32 | 2 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 48 | 39 | 5 | LLM |
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
| unsafe | 7 | 7 | 5 | LLM |
| pthread | 49 | 36 | 6 | LLM |
| raw\_ptr | 56 | 49 | 13 | LLM |
| static\_mut | 0 | 0 | 0 | tie |
| libc | 70 | 68 | 18 | LLM |
| std\_mutex | 0 | 3 | 5 | LLM |
| std\_arc | 0 | 0 | 12 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 184 | 179 | 75 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (3 calls); 7 unsafe keyword(s) remain
- **LLM**: 5 unsafe keyword(s) remain

---

### struct_malloc2

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 4 | LLM |
| pthread | 35 | 26 | 0 | LLM |
| raw\_ptr | 52 | 48 | 2 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 54 | 52 | 6 | LLM |
| std\_mutex | 0 | 4 | 4 | tie |
| std\_arc | 0 | 0 | 4 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 127 | 127 | 57 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (3 calls); static mut variables remain: x; 8 unsafe keyword(s) remain
- **LLM**: static mut variables remain: x; 4 unsafe keyword(s) remain

---

### struct_multiple

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 8 | 8 | 3 | LLM |
| pthread | 25 | 22 | 0 | LLM |
| raw\_ptr | 45 | 33 | 2 | LLM |
| static\_mut | 3 | 3 | 3 | tie |
| libc | 68 | 41 | 2 | LLM |
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
| unsafe | 7 | 7 | 5 | LLM |
| pthread | 25 | 22 | 15 | LLM |
| raw\_ptr | 36 | 32 | 28 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 50 | 41 | 36 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 3 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 132 | 114 | 82 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: s; 7 unsafe keyword(s) remain
- **LLM**: static mut variables remain: s; 5 unsafe keyword(s) remain

---

### struct_simple

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 0 | LLM |
| pthread | 28 | 22 | 0 | LLM |
| raw\_ptr | 40 | 32 | 0 | LLM |
| static\_mut | 1 | 1 | 0 | LLM |
| libc | 64 | 46 | 0 | LLM |
| std\_mutex | 0 | 5 | 7 | LLM |
| std\_arc | 0 | 0 | 9 | LLM |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 153 | 115 | 53 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: s; 7 unsafe keyword(s) remain

---

### struct_spin

**Compiles**: ConCrat ✅ Yes | LLM ✅ Yes

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 7 | 7 | 0 | LLM |
| pthread | 57 | 42 | 0 | LLM |
| raw\_ptr | 53 | 45 | 0 | LLM |
| static\_mut | 0 | 0 | 0 | tie |
| libc | 74 | 70 | 0 | LLM |
| std\_mutex | 0 | 7 | 7 | tie |
| std\_arc | 0 | 0 | 10 | LLM |
| std\_rwlock | 0 | 4 | 3 | ConCrat |
| std\_condvar | 0 | 1 | 0 | ConCrat |
| std\_thread | 0 | 0 | 2 | LLM |
| lines | 200 | 176 | 56 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (3 calls); 7 unsafe keyword(s) remain

---

### struct_timedwait

**Compiles**: ConCrat ✅ Yes | LLM ❌ No

| Metric | Original | ConCrat | LLM | Best |
|--------|:--------:|:-------:|:---:|:----:|
| unsafe | 9 | 9 | 9 | tie |
| pthread | 48 | 28 | 12 | LLM |
| raw\_ptr | 53 | 37 | 21 | LLM |
| static\_mut | 1 | 1 | 1 | tie |
| libc | 88 | 73 | 51 | LLM |
| std\_mutex | 0 | 3 | 3 | tie |
| std\_arc | 0 | 0 | 0 | tie |
| std\_rwlock | 0 | 1 | 0 | ConCrat |
| std\_condvar | 0 | 3 | 3 | tie |
| std\_thread | 0 | 0 | 0 | tie |
| lines | 249 | 218 | 193 | LLM |

**Remaining Issues:**

- **ConCrat**: Still uses pthread_mutex (2 calls); static mut variables remain: s; 9 unsafe keyword(s) remain
- **LLM**: static mut variables remain: S; 9 unsafe keyword(s) remain

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
