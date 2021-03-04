fn main() {
    #[cfg(feature = "test_output")]
    test_output();
    #[cfg(not(feature = "test_output"))]
    run_benches();
}

#[cfg(not(feature = "test_output"))]
fn run_benches() {
    use std::process::Command;
    let bench_feature = |feature| {
        println!("Starting benchmark for {}", feature);
        Command::new("cargo")
            .args(&[
                "bench",
                "--features",
                feature,
                //"--target-dir",
                //&[feature, "_tgt"].concat(),
            ])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    };

    bench_feature("env_logger_");
    bench_feature("fmt_subscriber");
    bench_feature("tracing_tracy");
    bench_feature("tracy_client");
}

#[cfg(feature = "test_env_logger")]
fn test_output() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .filter_module("instrument_bench::foo", log::LevelFilter::Error)
        // For some reason tracing spans misreport their module as this to log
        // when they have no extra data attached
        .filter_module("tracing::span", log::LevelFilter::Error)
        .init();

    test_output_inner();
}

#[cfg(feature = "test_tracing_subscriber")]
fn test_output() {
    use tracing_subscriber::filter::{EnvFilter, LevelFilter};
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::default()
        .add_directive(LevelFilter::DEBUG.into())
        .add_directive("instrument_bench::foo=error".parse().unwrap());

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    test_output_inner();
}

#[cfg(feature = "test_output")]
fn test_output_inner() {
    instrument_bench::log_message();
    instrument_bench::tracing_message();
    instrument_bench::tracing_span();
    instrument_bench::tracing_span_message();
    instrument_bench::foo::log_message();
    instrument_bench::foo::tracing_message();
    instrument_bench::foo::tracing_span();
    instrument_bench::foo::tracing_span_message();
}
