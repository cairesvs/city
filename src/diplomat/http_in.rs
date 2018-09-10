use components::context::Context;
use components::json::JsonResponse;
use fixtures::data::*;
use iron::status;
use iron::{Handler, IronResult, Request, Response};

#[derive(Clone)]
pub struct IndexHandler {
    ctx: Context,
}

impl IndexHandler {
    pub fn new(ctx: Context) -> IndexHandler {
        IndexHandler { ctx }
    }
}

impl Handler for IndexHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        self.ctx.logger.info("ronaldo");
        Ok(Response::with((status::Ok, JsonResponse::new(make_data()))))
    }
}

#[derive(Clone)]
pub struct ListHandler {
    ctx: Context,
}

impl ListHandler {
    pub fn new(ctx: Context) -> ListHandler {
        ListHandler { ctx }
    }
}

impl Handler for ListHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        self.ctx.logger.info("romario");
        Ok(Response::with((status::Ok, JsonResponse::new(make_data()))))
    }
}
