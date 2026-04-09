#[test]
fn miri_smoke_test() {
    for _ in 0..100 {
        main();
    }
}