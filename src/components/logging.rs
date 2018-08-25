extern crate slog;
extern crate slog_async;
extern crate slog_json;
extern crate std;

use slog::Drain;
use slog::*;

pub struct AppLogger {
    logger: Logger,
}

impl AppLogger {
    pub fn new() -> AppLogger {
        let drain = slog_json::Json::new(std::io::stdout())
            .set_pretty(true)
            .add_default_keys()
            .build()
            .fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        let logger = slog::Logger::root(drain, o!());
        return AppLogger { logger: logger };
    }

    pub fn info(&self, message: &str) {
        info!(self.logger, "{}", message);
    }
}
