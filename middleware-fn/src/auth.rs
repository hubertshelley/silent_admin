use app_service::service_utils::ApiUtils;
use configs::CFG;
use db::common::ctx::{ReqCtx, UserInfoCtx};
use sea_orm::prelude::async_trait;
use silent::{MiddleWareHandler, MiddlewareResult, Request, Response, Result, SilentError, StatusCode};

pub struct AuthMiddleware;

#[async_trait::async_trait]
impl MiddleWareHandler for AuthMiddleware {
    async fn pre_request(&self, req: &mut Request, _res: &mut Response) -> Result<MiddlewareResult> {
        let ctx = req.extensions().get::<ReqCtx>().expect("ReqCtx not found");
        let user = req.extensions().get::<UserInfoCtx>().expect("user not found");
        // 如果是超级用户，则不需要验证权限，直接放行
        if CFG.system.super_user.contains(&user.id) {
            return Ok(MiddlewareResult::Continue);
        }
        // 验证api权限，如果不在路由表中，则放行，否则验证权限

        if ApiUtils::is_in(&ctx.path).await {
            if ApiUtils::check_api_permission(&ctx.path, &ctx.method, &user.id).await {
                Ok(MiddlewareResult::Continue)
            } else {
                Ok(MiddlewareResult::Error(SilentError::business_error(
                    StatusCode::UNAUTHORIZED,
                    "permission denied".to_string(),
                )))
            }
        } else {
            Ok(MiddlewareResult::Continue)
        }
    }
}
