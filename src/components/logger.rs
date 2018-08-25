extern crate slog;
extern crate slog_async;
extern crate slog_json;
extern crate std;

use self::slog::Drain;
use self::slog::*;
use iron::prelude::*;
use iron::{typemap, BeforeMiddleware};

pub struct LoggerMiddleware {
    pub logger: Logger,
}

impl LoggerMiddleware {
    pub fn new() -> LoggerMiddleware {
        let drain = slog_json::Json::new(std::io::stdout())
            .set_pretty(true)
            .add_default_keys()
            .build()
            .fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        let logger = slog::Logger::root(drain, o!());
        LoggerMiddleware { logger }
    }
}

pub struct Value(Logger);

impl typemap::Key for LoggerMiddleware {
    type Value = Value;
}

impl BeforeMiddleware for LoggerMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let logger = self.logger.new(o!("route" => format!("{}", req.url)));
        req.extensions.insert::<LoggerMiddleware>(Value(logger));
        Ok(())
    }
}

pub trait LoggerReqExt {
    fn get_logger(&self) -> Logger;
}

impl<'a, 'b> LoggerReqExt for Request<'a, 'b> {
    fn get_logger(&self) -> Logger {
        let &Value(ref logger) = self.extensions.get::<LoggerMiddleware>().unwrap();

        logger.clone()
    }
}
