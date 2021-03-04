use criterion::{criterion_group, criterion_main, Criterion};

fn instrument_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("tracy_client");
    // tracy-client span
    group.measurement_time(core::time::Duration::from_millis(200));
    group.bench_function("tracy client span", |b| b.iter(tracy_client_span));
    group.finish();
}

fn tracy_client_span() {
    tracy_client::Span::new("name", "function", "file.rs", 42, 0);
}

criterion_group!(benches, instrument_bench);
criterion_main!(benches);
