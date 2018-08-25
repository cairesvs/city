extern crate router;

use components::context::Context;
use diplomat::http_in::IndexHandler;

pub struct Router {
    pub router: router::Router,
}

impl Router {
    pub fn new(ctx: Context) -> Router {
        let mut r = router::Router::new();
        r.get("/", IndexHandler { ctx }, "index");
        Router { router: r }
    }
}
