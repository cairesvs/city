use components::context::Context;
use components::json::JsonResponse;
use fixtures::data::*;
use iron::status;
use iron::{Handler, IronResult, Request, Response};

pub struct IndexHandler {
    pub ctx: Context,
}

impl Handler for IndexHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let data = make_data();
        self.ctx.logger.info("ronaldo");
        Ok(Response::with((status::Ok, JsonResponse::new(data))))
    }
}
