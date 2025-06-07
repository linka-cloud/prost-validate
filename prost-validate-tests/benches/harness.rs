use criterion::{criterion_group, criterion_main, Criterion};
use prost_validate_tests::cases::CASES;

#[cfg(feature = "reflect")]
fn reflect_validate() {
    for (name, f) in CASES.iter() {
        let (message, failures) = f();
        match prost_reflect_validate::ValidatorExt::validate(message.as_ref()) {
            Ok(_) => assert_eq!(failures, 0, "{name}: unexpected validation success"),
            Err(err) => assert!(failures > 0, "{name}: unexpected validation failure: {err}"),
        }
    }
}

#[cfg(feature = "derive")]
fn derive_validate() {
    for (name, f) in CASES.iter() {
        let (message, failures) = f();
        match prost_validate::Validator::validate(message.as_ref()) {
            Ok(_) => assert_eq!(failures, 0, "{name}: unexpected validation success"),
            Err(err) => assert!(failures > 0, "{name}: unexpected validation failure: {err}"),
        }
    }
}

#[cfg(feature = "derive")]
fn derive_validate_all() {
    for (name, f) in CASES.iter() {
        let (message, failures) = f();
        let errors = prost_validate::Validator::validate_all(message.as_ref());
        let res = if errors.is_empty() {
            "success"
        } else {
            "failure"
        };
        assert_eq!(
            failures as usize,
            errors.len(),
            "{name}: unexpected validation {res}: {errors:?}"
        );
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    #[cfg(feature = "reflect")]
    c.bench_function("harness reflect", |b| b.iter(reflect_validate));
    #[cfg(feature = "derive")]
    c.bench_function("harness derive", |b| b.iter(derive_validate));
    #[cfg(feature = "derive")]
    c.bench_function("harness derive_all", |b| b.iter(derive_validate_all));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
