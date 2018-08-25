use components::context::Context;
use iron::status;
use iron::{Handler, IronResult, Request, Response};

pub struct IndexHandler {
    pub ctx: Context,
}

impl Handler for IndexHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        self.ctx.logger.info("ronaldo");
        Ok(Response::with((status::Ok, "ronaldo")))
    }
}
