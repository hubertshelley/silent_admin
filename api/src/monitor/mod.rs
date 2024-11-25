mod server_info;
mod sys_login_log;
mod sys_oper_log;

use silent::prelude::{HandlerAppend, Route};

pub fn monitor_api() -> Route {
    Route::new("")
        .append(Route::new("logininfor").append(sys_login_log_api())) // 登录日志模块
        .append(Route::new("operlog").append(sys_oper_log_api())) // 操作日志
        .append(Route::new("server").get(server_info::get_server_info)) // 服务器信息
        .append(Route::new("server-event").get(server_info::get_server_info_sse))
    // 服务器信息(SSE)
}

pub fn sys_login_log_api() -> Route {
    Route::new("")
        .append(Route::new("list").get(sys_login_log::get_sort_list)) // 获取全部登录日志
        .append(Route::new("clean").delete(sys_login_log::clean)) // 清空登录日志
        .append(Route::new("delete").delete(sys_login_log::delete)) // 硬删除登录日志
}

fn sys_oper_log_api() -> Route {
    Route::new("")
        .append(Route::new("list").get(sys_oper_log::get_sort_list)) // 获取全部操作日志
        .append(Route::new("get_by_id").get(sys_oper_log::get_by_id)) // 按id获取操作日志
        .append(Route::new("clean").delete(sys_oper_log::clean)) // 清空操作日志
        .append(Route::new("delete").delete(sys_oper_log::delete)) // 硬删除操作日志
}
