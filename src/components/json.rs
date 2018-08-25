extern crate iron;
extern crate serde;
extern crate serde_json;

use iron::headers::ContentType;
use iron::modifier::Modifier;
use iron::prelude::*;
use iron::{typemap, AfterMiddleware};

use self::serde::ser::Serialize as ToJson;
use self::serde_json::value::{self, Value as Json};

#[derive(Clone)]
pub struct JsonResponse {
    value: Json,
}

impl JsonResponse {
    pub fn new<T: ToJson>(value: T) -> JsonResponse {
        JsonResponse {
            value: value::to_value(&value).unwrap_or(Json::Null),
        }
    }
}

pub struct JsonResponseMiddleware;

impl JsonResponseMiddleware {
    pub fn new() -> Self {
        JsonResponseMiddleware {}
    }
}

impl typemap::Key for JsonResponseMiddleware {
    type Value = JsonResponse;
}

impl Modifier<Response> for JsonResponse {
    fn modify(self, resp: &mut Response) {
        resp.extensions.insert::<JsonResponseMiddleware>(self);
    }
}

impl AfterMiddleware for JsonResponseMiddleware {
    fn after(&self, _: &mut Request, r: Response) -> IronResult<Response> {
        let mut resp = r;
        let header_body = resp
            .extensions
            .remove::<JsonResponseMiddleware>()
            .and_then(|j| {
                serde_json::to_string(&j.value)
                    .ok()
                    .map(|json_string| (ContentType::json(), json_string))
            });

        match header_body {
            Some((content_type, body)) => resp.set_mut(body).headers.set(content_type),
            None => (),
        }
        Ok(resp)
    }

    fn catch(&self, req: &mut Request, mut err: IronError) -> IronResult<Response> {
        err.response = try!(self.after(req, err.response));
        Err(err)
    }
}
