use silent::{Handler, MiddleWareHandler, Next, Request, Response, Result as SilentResult};

use configs::CFG;
use dto::common::ctx::ReqCtx;
use crate::middlewares::authorization::AuthUser;

pub struct CtxMiddleware;

#[async_trait::async_trait]
impl MiddleWareHandler for CtxMiddleware {
    async fn handle(&self, mut req: Request, next: &Next) -> SilentResult<Response> {
        let _user = req.extensions().get::<AuthUser>().unwrap();
        let ori_uri_path = req.uri().path().to_owned();
        let path = ori_uri_path.replacen(&(CFG.server.api_prefix.clone() + "/"), "", 1);
        let method = req.method().to_string();
        let path_params = req.uri().query().unwrap_or("").to_string();

        let req_ctx = ReqCtx {
            ori_uri: if path_params.is_empty() { ori_uri_path } else { ori_uri_path + "?" + &path_params },
            path,
            path_params,
            method: method.to_string(),
            // user: UserInfo {
            //     id: user.id,
            //     token_id: user.token_id,
            //     name: user.name,
            // },
            data: "".to_string(),
        };

        req.extensions_mut().insert(req_ctx);
        next.call(req).await
    }
}
