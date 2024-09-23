use criterion::{criterion_group, criterion_main, Criterion};
use prost_validate_tests::{TestCase, CASES};

fn validate() {
    for (name, f) in CASES.iter() {
        let (message, failures) = f();
        let case = TestCase { message, failures };
        match case.message.validate() {
            Ok(_) => assert_eq!(case.failures, 0, "{name}: unexpected validation success"),
            Err(err) => assert!(
                case.failures > 0,
                "{name}: unexpected validation failure: {err}"
            ),
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("harness", |b| b.iter(|| validate()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
