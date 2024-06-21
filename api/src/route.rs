use silent::prelude::{HandlerAppend, Route};

use configs::CFG;

use super::{system, test};

pub fn api() -> Route {
    let result_wrapper = middleware_fn::ResultWrapper::new();
    // 路由配置
    Route::new("")
        .hook(result_wrapper)
        // 文件上传api
        // .nest_service(&CFG.web.upload_url, get_service(ServeDir::new(&CFG.web.upload_dir)))
        // 无需授权Api.通用模块
        .append(Route::new("comm").append(no_auth_api()))
        // 系统管理模块
        .append(Route::new("system").append(set_auth_middleware(system::system_api())))
        //  测试模块
        .append(Route::new("test").append(test_api()))
}

// 无需授权api
fn no_auth_api() -> Route {
    Route::new("")
        .append(Route::new("login").post(system::login)) // 登录
        .append(Route::new("get_captcha").get(system::get_captcha)) // 获取验证码
        .append(Route::new("log_out").post(system::log_out)) // 退出登录
}

// 设置授权路由的中间件
fn set_auth_middleware(router: Route) -> Route {
    let api_auth = middleware_fn::ApiAuth {};
    let ctx = middleware_fn::Ctx {};
    let router = router.hook(ctx).hook(api_auth);
    let router = match &CFG.log.enable_oper_log {
        true => {
            let oper_log = middleware_fn::OperLog {};
            router.hook(oper_log)
        }
        false => router,
    };
    match CFG.server.cache_time {
        0 => router,
        _ => {
            if CFG.server.cache_method == 0 {
                let cache = middleware_fn::Cache {};
                router.hook(cache)
            } else {
                let cache = middleware_fn::SkyTableCache {};
                router.hook(cache)
            }
        }
    }
}

// 测试api
pub fn test_api() -> Route {
    let router = test::test_api();

    let router = match &CFG.log.enable_oper_log {
        true => {
            let oper_log = middleware_fn::OperLog {};
            router.hook(oper_log)
        }
        false => router,
    };
    let api_auth = middleware_fn::ApiAuth {};
    let ctx = middleware_fn::Ctx {};
    router.hook(api_auth).hook(ctx)
}
