extern crate router;

use components::context::Context;
use diplomat::http_in::{IndexHandler, ListHandler};

pub struct Services {
    router: router::Router,
}

impl Services {
    pub fn new(ctx: Context) -> Services {
        let mut r = router::Router::new();
        r.get("/", IndexHandler::new(ctx.clone()), "index");
        r.get("/list", ListHandler::new(ctx.clone()), "list");
        Services { router: r }
    }

    pub fn get_router(self) -> router::Router {
        self.router
    }
}
