extern crate iron;
#[macro_use]
extern crate slog;

#[macro_use]
mod diplomat;
mod components;

use components::context::Context;
use components::router::Router;
use iron::Iron;

fn main() {
    let ctx = Context::new();
    ctx.logger.info("comecou");

    let router = Router::new(ctx);

    match Iron::new(router.router).http("127.0.0.1:3000") {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    }
}
