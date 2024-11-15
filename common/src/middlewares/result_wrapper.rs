use crate::response::Res;
use sea_orm::prelude::async_trait;
use silent::prelude::{Next, ResBody, Route};
use silent::{Handler, MiddleWareHandler, Request, Response, Result};

#[derive(Default)]
pub struct ResultWrapper;

impl ResultWrapper {
    pub fn new() -> Self {
        Self
    }

    pub fn register(route: Route) -> Route {
        route.root_hook(Self::new())
    }
}

#[async_trait::async_trait]
impl MiddleWareHandler for ResultWrapper {
    async fn handle(&self, req: Request, next: &Next) -> Result<Response> {
        let mut res = next.call(req).await.unwrap_or_else(|err| err.into());
        if let ResBody::Once(body) = &res.body() {
            let body: serde_json::Value = match serde_json::from_slice(body.to_vec().as_slice()) {
                Ok(body) => body,
                _ => serde_json::Value::String(String::from_utf8(body.to_vec()).unwrap()),
            };
            res.copy_from_response(Res::with_data(body).into());
        }
        Ok(res)
    }
}
