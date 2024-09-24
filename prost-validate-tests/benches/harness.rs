use criterion::{criterion_group, criterion_main, Criterion};
use prost_validate_tests::CASES;

fn reflect_validate() {
    for (name, f) in CASES.iter() {
        let (message, failures) = f();
        match prost_reflect_validate::ValidatorExt::validate(&*message) {
            Ok(_) => assert_eq!(failures, 0, "{name}: unexpected validation success"),
            Err(err) => assert!(failures > 0, "{name}: unexpected validation failure: {err}"),
        }
    }
}

fn derive_validate() {
    for (name, f) in CASES.iter() {
        let (message, failures) = f();
        match ::prost_validate::Validator::validate(&*message) {
            Ok(_) => assert_eq!(failures, 0, "{name}: unexpected validation success"),
            Err(err) => assert!(failures > 0, "{name}: unexpected validation failure: {err}"),
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("harness reflect", |b| b.iter(|| reflect_validate()));
    c.bench_function("harness derive", |b| b.iter(|| derive_validate()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
