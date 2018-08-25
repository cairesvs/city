extern crate router;

use components::context::Context;
use diplomat::http_in::IndexHandler;

pub struct Services {
    pub router: router::Router,
}

impl Services {
    pub fn new(ctx: Context) -> Services {
        let mut r = router::Router::new();
        r.get("/", IndexHandler { ctx }, "index");
        Services { router: r }
    }
}
