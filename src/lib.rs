pub fn log_message() {
    log::debug!("Hi from log!");
}

pub fn tracing_message() {
    tracing::debug!("Hi from tracing!");
}

pub fn tracing_span() {
    tracing::debug_span!("tracing span").entered().exit();
}

pub fn tracing_span_message() {
    tracing::debug_span!("tracing span with data", "Hi!")
        .entered()
        .exit();
}

pub mod foo {
    pub fn log_message() {
        log::info!("Hi from log!");
    }

    pub fn tracing_message() {
        tracing::info!("Hi from tracing!");
    }

    pub fn tracing_span() {
        tracing::info_span!("tracing span").entered().exit();
    }

    pub fn tracing_span_message() {
        tracing::info_span!("tracing span with data", "Hi!")
            .entered()
            .exit();
    }
}

// Use error level so we can filter out the debugs without filtering this one out
pub fn tracing_active_span() {
    tracing::error_span!("!!").entered().exit();
}
