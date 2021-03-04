use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use instrument_bench::*;

// filtering things
#[allow(dead_code)]
fn filter_bench(group: &mut BenchmarkGroup<WallTime>) {
    group.bench_function("level filtered log message", |b| b.iter(log_message));
    group.bench_function("level filtered tracing message", |b| {
        b.iter(tracing_message)
    });
    group.bench_function("level filtered tracing span", |b| b.iter(tracing_span));
    group.bench_function("level filtered tracing span with log data", |b| {
        b.iter(tracing_span_message)
    });
    group.bench_function("module filtered log message", |b| b.iter(foo::log_message));
    group.bench_function("module filtered tracing message", |b| {
        b.iter(foo::tracing_message)
    });
    group.bench_function("module filtered tracing span", |b| {
        b.iter(foo::tracing_span)
    });
    group.bench_function("module filtered tracing span with log data", |b| {
        b.iter(foo::tracing_span_message)
    });
}

#[cfg(feature = "env_logger_")]
fn instrument(c: &mut Criterion) {
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
    // filtered out log message via log:env_logger
    // filtered out log message via tracing:env_logger
    // filtered out tracing span via env_logger
    // filtered out tracing span with log data via env_logger
    // module filtered out log message via log:env_logger
    // module filtered out log message via tracing:env_logger
    // module filtered out tracing span via env_logger
    // module filtered out tracing span with log data via env_logger
    filter_bench(&mut group);
    group.finish();
}

#[cfg(feature = "fmt_subscriber")]
fn instrument(c: &mut Criterion) {
    let mut group = c.benchmark_group("fmt_subscriber");
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

    // filtered out log message via log:tracing_subscriber
    // filtered out log message via tracing:tracing_subscriber
    // filtered out tracing span via tracing-subscriber
    // filtered out tracing span with log data via tracing-subscriber
    // module filtered out log message via log:tracing_subscriber
    // module filtered out log message via tracing:tracing_subscriber
    // module filtered out tracing span via tracing-subscriber
    // module filtered out tracing span with log data via tracing-subscriber
    filter_bench(&mut group);
    // active span via tracing-subscriber
    group.bench_function("active tracing span", |b| b.iter(tracing_active_span));
    group.finish();
}

#[cfg(feature = "tracing_tracy")]
fn instrument(c: &mut Criterion) {
    let mut group = c.benchmark_group("tracing_tracy");
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

    // see above
    filter_bench(&mut group);
    // active span via tracing-tracy
    group.measurement_time(core::time::Duration::from_millis(200));
    group.bench_function("active tracing span", |b| b.iter(tracing_active_span));
    group.finish();
}

#[cfg(feature = "tracy_client")]
fn instrument(c: &mut Criterion) {
    let mut group = c.benchmark_group("tracy_client");
    // tracy-client span
    group.measurement_time(core::time::Duration::from_millis(200));
    group.bench_function("tracy client span", |b| b.iter(tracy_client_span));
    group.finish();
}

#[allow(dead_code)]
fn tracy_client_span() {
    tracy_client::Span::new("name", "function", "file.rs", 42, 0);
}

criterion_group!(benches, instrument);
criterion_main!(benches);
