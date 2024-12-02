use silent::{Configs, Response, SilentError};

use db::common::res::Res;
//  重新导出
pub use route::api;

// api 模块
mod system; // 系统模块
mod test; // 测试模块

//  路由模块
mod route;

pub async fn exception_handler(err: SilentError, _configs: Configs) -> Response {
    Res::<()>::with_err_code(&err.message(), err.status().as_u16()).into()
}
