use criterion::{criterion_group, criterion_main, Criterion};

mod common;
use common::*;

fn instrument_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("tracing-tracy");
    group.measurement_time(core::time::Duration::from_millis(2000));

    use tracing_subscriber::filter::{EnvFilter, LevelFilter};
    use tracing_subscriber::prelude::*;

    let tracy_layer = tracing_tracy::TracyLayer::new().with_stackdepth(0);
    let filter_layer = EnvFilter::default()
        .add_directive(LevelFilter::INFO.into())
        .add_directive("instrument_bench::foo=error".parse().unwrap());
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(tracy_layer)
        .init();

    filter_bench(&mut group);
    // active span via tracing-tracy
    group.measurement_time(core::time::Duration::from_millis(200));
    group.bench_function("active tracing span", |b| b.iter(tracing_active_span));

    group.finish();
}

criterion_group!(benches, instrument_bench);
criterion_main!(benches);
