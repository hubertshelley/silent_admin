use db::common::res::Res;
use sea_orm::prelude::async_trait;
use silent::prelude::ResBody;
use silent::{MiddleWareHandler, MiddlewareResult, Response, Result};

pub struct ResultWrapper;

impl ResultWrapper {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl MiddleWareHandler for ResultWrapper {
    async fn after_response(&self, res: &mut Response) -> Result<MiddlewareResult> {
        if res.headers().get("content-type").unwrap() == "application/json" {
            if let ResBody::Once(body) = &res.body() {
                let body: serde_json::Value = serde_json::from_slice(body.to_vec().as_slice())?;
                res.copy_from_response(Res::with_data(body).into());
            }
        }
        Ok(MiddlewareResult::Continue)
    }
}
