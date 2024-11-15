use crate::middlewares::authorization::User;
use crate::BpmError;
use regex::Regex;
use sea_orm::prelude::async_trait;
use silent::{Handler, MiddleWareHandler, Next, Request, Response, SilentError};

#[derive(Clone, Debug)]
pub struct PermissionMiddleware {
    uncheck_url: Vec<Regex>,
}

impl PermissionMiddleware {
    pub fn new(uncheck_url: Vec<String>) -> Self {
        let uncheck_url = uncheck_url
            .iter()
            .map(|url| Regex::new(url).unwrap())
            .collect();
        Self { uncheck_url }
    }
}

#[async_trait::async_trait]
impl MiddleWareHandler for PermissionMiddleware {
    async fn handle(&self, req: Request, next: &Next) -> Result<Response, SilentError> {
        let path = req.uri().path().to_string();
        for url in &self.uncheck_url {
            if url.is_match(&path) {
                return next.call(req).await;
            }
        }
        if let Some(user) = req.extensions().get::<User>() {
            if user.is_authenticated() {
                return next.call(req).await;
            }
        }
        Err(BpmError::Unauthorized.into())
    }
}
