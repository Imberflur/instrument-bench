use criterion::{measurement::WallTime, BenchmarkGroup};
use instrument_bench::*;

pub use instrument_bench::tracing_active_span;

// filtering things
#[allow(dead_code)]
pub fn filter_bench(group: &mut BenchmarkGroup<WallTime>) {
    // filtered out log message via log:tracing_subscriber
    // filtered out log message via tracing:tracing_subscriber
    // filtered out tracing span via tracing-subscriber
    // filtered out tracing span with log data via tracing-subscriber
    // module filtered out log message via log:tracing_subscriber
    // module filtered out log message via tracing:tracing_subscriber
    // module filtered out tracing span via tracing-subscriber
    // module filtered out tracing span with log data via tracing-subscriber

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
