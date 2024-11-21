use silent::prelude::{HandlerAppend, Route};

use super::{monitor, system};

pub fn get_routes() -> Route {
    // 路由配置
    Route::new("")
        // 文件上传api
        // .nest_service(&CFG.web.upload_url, get_service(ServeDir::new(&CFG.web.upload_dir)))
        // 无需授权Api.通用模块
        .append(
            Route::new("comm").append(
                Route::new("")
                    .append(Route::new("login").post(system::login)) // 登录
                    .append(Route::new("get_captcha").get(system::get_captcha)) // 获取验证码
                    .append(Route::new("logout").post(system::logout)),
            ),
        ) // 退出登录
        // 系统管理模块
        .append(Route::new("system").append(system::system_api()))
        // 监控模块
        .append(Route::new("monitor").append(monitor::monitor_api())) // 操作日志
    //  测试模块
    // .append(Route::new("test").append(test::test_api()))
}
