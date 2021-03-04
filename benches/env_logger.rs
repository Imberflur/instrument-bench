use criterion::{criterion_group, criterion_main, Criterion};

mod common;
use common::*;

fn instrument_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("env_logger");
    group.measurement_time(core::time::Duration::from_millis(2000));
    // use info to filter out everything since they use debug
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .filter_module("instrument_bench::foo", log::LevelFilter::Error)
        // For some reason tracing spans misreport their module as this to log
        // when they have no extra data attached
        .filter_module("tracing::span", log::LevelFilter::Error)
        .init();

    filter_bench(&mut group);
    group.finish();
}

criterion_group!(benches, instrument_bench);
criterion_main!(benches);
