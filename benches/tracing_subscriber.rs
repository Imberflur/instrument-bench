use criterion::{criterion_group, criterion_main, Criterion};

mod common;
use common::*;

fn instrument_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("tracy-subscriber");
    group.measurement_time(core::time::Duration::from_millis(2000));

    use tracing_subscriber::filter::{EnvFilter, LevelFilter};
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::default()
        .add_directive(LevelFilter::INFO.into())
        .add_directive("instrument_bench::foo=error".parse().unwrap());
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    filter_bench(&mut group);
    // active span via tracing-subscriber
    group.bench_function("active tracing span", |b| b.iter(tracing_active_span));

    group.finish();
}

criterion_group!(benches, instrument_bench);
criterion_main!(benches);
