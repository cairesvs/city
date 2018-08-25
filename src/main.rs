extern crate iron;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde_json;

#[macro_use]
mod diplomat;
mod components;
#[macro_use]
mod fixtures;

use components::context::Context;
use components::json::JsonResponseMiddleware;
use components::router::Services;
use iron::prelude::*;
use iron::Iron;

fn main() {
    let ctx = Context::new();
    ctx.logger.info("comecou");

    let services = Services::new(ctx);
    let router = services.router;

    let mut chain = Chain::new(router);
    chain.link_after(JsonResponseMiddleware::new());

    match Iron::new(chain).http("127.0.0.1:3000") {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    }
}
