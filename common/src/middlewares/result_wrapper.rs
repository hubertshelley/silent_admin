use crate::response::Res;
use serde_json::Value;
use silent::prelude::{Next, ResBody, Route};
use silent::{Handler, MiddleWareHandler, Request, Response, Result};

#[derive(Default)]
pub struct ResultWrapper;

impl ResultWrapper {
    pub fn new() -> Self {
        Self
    }

    pub fn register(route: Route) -> Route {
        route.hook(Self::new())
    }
}

#[async_trait::async_trait]
impl MiddleWareHandler for ResultWrapper {
    async fn handle(&self, req: Request, next: &Next) -> Result<Response> {
        let res = match next.call(req).await {
            Ok(mut res) => {
                if let ResBody::Once(body) = &res.body() {
                    let body: Value = match serde_json::from_slice(body.to_vec().as_slice()) {
                        Ok(body) => body,
                        _ => Value::String(String::from_utf8(body.to_vec()).unwrap()),
                    };
                    res.copy_from_response(Res::with_data(body).into());
                }
                res
            }
            Err(err) => {
                let mut res = Response::empty();
                res.copy_from_response(
                    Res::<Value>::with_err(err.status().as_u16(), err.message().as_str()).into(),
                );
                res
            }
        };
        Ok(res)
    }
}
