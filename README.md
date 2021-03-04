# instrument-bench
`instrument-bench` is a set of benchmarks comparing the overhead of various
logging and instrumentation crates. These include:
#### Backends
* [`log`][log]
* [`tracing`][tracing]

#### Frontends
* [`env_logger`][env_logger]
* [`tracing-subscriber`][tracing-subscriber] (with the [`fmt`][fmt] subscriber layer)

#### Direct instrumentation
* [`tracy-client`][tracy-client]

[log]: https://crates.io/crates/log
[tracing]: https://crates.io/crates/tracing
[env_logger]: https://crates.io/crates/env_logger
[tracing-subscriber]: https://crates.io/crates/tracing-subscriber
[tracy-client]: https://crates.io/crates/tracing-subscriber
[fmt]: https://docs.rs/tracing-subscriber/0.2.16/tracing_subscriber/fmt/index.html

The backends/frontends listed here are benchmarked in all possible combinations since they are all compatible.
# How to use
```sh
cargo bench
```


# Benchmark results
Note: I did not obtain these in a controlled environment and they can vary wildy
(-50/+100 percent). However, the general relationships that are more than a factor
of 2 apart should hold.


### Benchmarks with instrumentation suppressed by a global or module specific filter.
|instrumentation                         |`env_logger` |`tracing-subscriber`|`tracing-tracy`   |
|:---------------------------------------|:------------|:-------------------|:-----------------|
|**`Level::Info`**                       |             |                    |                  |
|`log::debug!("")`                       |1.6 ns       |1.6 ns              |1.9 ns            |
|`tracing::debug!("")`                   |2.6 ns       |3.8 ns              |3.4 ns            |
|`tracing::debug_span!("")`              |39.4 ns      |13.9 ns             |14.4 ns           |
|`tracing::debug_span!("", "extra info")`|48.3 ns      |18.7 ns             |15.0 ns           |
|**`"mycrate::foo=error"`\***            |             |                    |                  |
|`log::info!("")`                        |24.0 ns      |81.7 ns             |44.3 ns           |
|`tracing::info!("")`                    |12.5 ns      |3.0 ns              |2.7 ns            |
|`tracing::info_span!("")`               |76.9 ns      |19.4 ns             |13.9 ns           |
|`tracing::info_span!("", "extra info")` |86.32 ns     |19.7 ns             |14.5 ns           |

**\***For the `info_span!("")` "tracing::span=error" is needed to suppres it rather than using the module path because tracing spans report this as their module name to `log` frontends for spans that have no extra fields attached.

### Benchmarks with an active span.
| crate            |  time (ns) |
|:-----------------|:-----------|
|`tracy-subscriber`| 600        |
|`tracy-tracing`   | 1062       |
|`tracy-client`    | 205        |
