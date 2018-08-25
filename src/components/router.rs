extern crate router;

use components::context::Context;
use diplomat::http_in::{IndexHandler, ListHandler};

pub struct Services {
    pub router: router::Router,
}

impl Services {
    pub fn new(ctx: Context) -> Services {
        let mut r = router::Router::new();
        r.get("/", IndexHandler { ctx: ctx.clone() }, "index");
        r.get("/list", ListHandler { ctx: ctx.clone() }, "list");
        Services { router: r }
    }
}
