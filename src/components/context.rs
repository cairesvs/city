use components::logger::AppLogger;

#[derive(Clone)]
pub struct Context {
    pub environment: &'static str,
    pub logger: AppLogger,
}

impl Context {
    pub fn new() -> Context {
        let logger = AppLogger::new();
        Context {
            environment: "development",
            logger,
        }
    }
}
