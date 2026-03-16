# Clippy Concurrency Evaluation

Lints checked: `clippy::arc_with_non_send_sync`, `clippy::await_holding_lock`, `clippy::mutex_atomic`, `clippy::mut_mutex_lock`

## Summary

- Total examples: 30
- Concrat warning-free examples: 0
- LLM warning-free examples: 0
- Concrat total warning hits: 60
- LLM total warning hits: 60

## Detail

| Example | Version | clippy_ok | warnings | lints |
|---|---|---|---:|---|
| array_const | original | no | 2 | clippy::arc_with_non_send_sync |
| array_const | concrat | no | 2 | clippy::arc_with_non_send_sync |
| array_const | llm | no | 2 | clippy::arc_with_non_send_sync |
| array_main | original | no | 2 | clippy::arc_with_non_send_sync |
| array_main | concrat | no | 2 | clippy::arc_with_non_send_sync |
| array_main | llm | no | 2 | clippy::arc_with_non_send_sync |
| array_simple | original | no | 2 | clippy::arc_with_non_send_sync |
| array_simple | concrat | no | 2 | clippy::arc_with_non_send_sync |
| array_simple | llm | no | 2 | clippy::arc_with_non_send_sync |
| global_assume | original | no | 2 | clippy::arc_with_non_send_sync |
| global_assume | concrat | no | 2 | clippy::arc_with_non_send_sync |
| global_assume | llm | no | 2 | clippy::arc_with_non_send_sync |
| global_assume2 | original | no | 2 | clippy::arc_with_non_send_sync |
| global_assume2 | concrat | no | 2 | clippy::arc_with_non_send_sync |
| global_assume2 | llm | no | 2 | clippy::arc_with_non_send_sync |
| global_check | original | no | 2 | clippy::arc_with_non_send_sync |
| global_check | concrat | no | 2 | clippy::arc_with_non_send_sync |
| global_check | llm | no | 2 | clippy::arc_with_non_send_sync |
| global_condvar | original | no | 2 | clippy::arc_with_non_send_sync |
| global_condvar | concrat | no | 2 | clippy::arc_with_non_send_sync |
| global_condvar | llm | no | 2 | clippy::arc_with_non_send_sync |
| global_custom | original | no | 2 | clippy::arc_with_non_send_sync |
| global_custom | concrat | no | 2 | clippy::arc_with_non_send_sync |
| global_custom | llm | no | 2 | clippy::arc_with_non_send_sync |
| global_main | original | no | 2 | clippy::arc_with_non_send_sync |
| global_main | concrat | no | 2 | clippy::arc_with_non_send_sync |
| global_main | llm | no | 2 | clippy::arc_with_non_send_sync |
| global_nested | original | no | 2 | clippy::arc_with_non_send_sync |
| global_nested | concrat | no | 2 | clippy::arc_with_non_send_sync |
| global_nested | llm | no | 2 | clippy::arc_with_non_send_sync |
| global_read | original | no | 2 | clippy::arc_with_non_send_sync |
| global_read | concrat | no | 2 | clippy::arc_with_non_send_sync |
| global_read | llm | no | 2 | clippy::arc_with_non_send_sync |
| global_rwlock | original | no | 2 | clippy::arc_with_non_send_sync |
| global_rwlock | concrat | no | 2 | clippy::arc_with_non_send_sync |
| global_rwlock | llm | no | 2 | clippy::arc_with_non_send_sync |
| global_simple | original | no | 2 | clippy::arc_with_non_send_sync |
| global_simple | concrat | no | 2 | clippy::arc_with_non_send_sync |
| global_simple | llm | no | 2 | clippy::arc_with_non_send_sync |
| global_trylock | original | no | 2 | clippy::arc_with_non_send_sync |
| global_trylock | concrat | no | 2 | clippy::arc_with_non_send_sync |
| global_trylock | llm | no | 2 | clippy::arc_with_non_send_sync |
| global_while | original | no | 2 | clippy::arc_with_non_send_sync |
| global_while | concrat | no | 2 | clippy::arc_with_non_send_sync |
| global_while | llm | no | 2 | clippy::arc_with_non_send_sync |
| struct_alias | original | no | 2 | clippy::arc_with_non_send_sync |
| struct_alias | concrat | no | 2 | clippy::arc_with_non_send_sync |
| struct_alias | llm | no | 2 | clippy::arc_with_non_send_sync |
| struct_assume | original | no | 2 | clippy::arc_with_non_send_sync |
| struct_assume | concrat | no | 2 | clippy::arc_with_non_send_sync |
| struct_assume | llm | no | 2 | clippy::arc_with_non_send_sync |
| struct_condvar | original | no | 2 | clippy::arc_with_non_send_sync |
| struct_condvar | concrat | no | 2 | clippy::arc_with_non_send_sync |
| struct_condvar | llm | no | 2 | clippy::arc_with_non_send_sync |
| struct_dup | original | no | 2 | clippy::arc_with_non_send_sync |
| struct_dup | concrat | no | 2 | clippy::arc_with_non_send_sync |
| struct_dup | llm | no | 2 | clippy::arc_with_non_send_sync |
| struct_empty | original | no | 2 | clippy::arc_with_non_send_sync |
| struct_empty | concrat | no | 2 | clippy::arc_with_non_send_sync |
| struct_empty | llm | no | 2 | clippy::arc_with_non_send_sync |
| struct_init | original | no | 2 | clippy::arc_with_non_send_sync |
| struct_init | concrat | no | 2 | clippy::arc_with_non_send_sync |
| struct_init | llm | no | 2 | clippy::arc_with_non_send_sync |
| struct_main | original | no | 2 | clippy::arc_with_non_send_sync |
| struct_main | concrat | no | 2 | clippy::arc_with_non_send_sync |
| struct_main | llm | no | 2 | clippy::arc_with_non_send_sync |
| struct_malloc | original | no | 2 | clippy::arc_with_non_send_sync |
| struct_malloc | concrat | no | 2 | clippy::arc_with_non_send_sync |
| struct_malloc | llm | no | 2 | clippy::arc_with_non_send_sync |
| struct_malloc2 | original | no | 2 | clippy::arc_with_non_send_sync |
| struct_malloc2 | concrat | no | 2 | clippy::arc_with_non_send_sync |
| struct_malloc2 | llm | no | 2 | clippy::arc_with_non_send_sync |
| struct_multiple | original | no | 2 | clippy::arc_with_non_send_sync |
| struct_multiple | concrat | no | 2 | clippy::arc_with_non_send_sync |
| struct_multiple | llm | no | 2 | clippy::arc_with_non_send_sync |
| struct_nested | original | no | 2 | clippy::arc_with_non_send_sync |
| struct_nested | concrat | no | 2 | clippy::arc_with_non_send_sync |
| struct_nested | llm | no | 2 | clippy::arc_with_non_send_sync |
| struct_simple | original | no | 2 | clippy::arc_with_non_send_sync |
| struct_simple | concrat | no | 2 | clippy::arc_with_non_send_sync |
| struct_simple | llm | no | 2 | clippy::arc_with_non_send_sync |
| struct_spin | original | no | 2 | clippy::arc_with_non_send_sync |
| struct_spin | concrat | no | 2 | clippy::arc_with_non_send_sync |
| struct_spin | llm | no | 2 | clippy::arc_with_non_send_sync |
| struct_timedwait | original | no | 2 | clippy::arc_with_non_send_sync |
| struct_timedwait | concrat | no | 2 | clippy::arc_with_non_send_sync |
| struct_timedwait | llm | no | 2 | clippy::arc_with_non_send_sync |
| unused_func | original | no | 2 | clippy::arc_with_non_send_sync |
| unused_func | concrat | no | 2 | clippy::arc_with_non_send_sync |
| unused_func | llm | no | 2 | clippy::arc_with_non_send_sync |
