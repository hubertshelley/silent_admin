mod common;
// mod sys_api_db;
mod sys_dept;
mod sys_dict_data;
mod sys_dict_type;
// mod sys_job;
// mod sys_job_log;
mod sys_login_log;
mod sys_menu;
mod sys_oper_log;
mod sys_post;
mod sys_role; // 角色管理
              // mod sys_update_log;
mod sys_user;
mod sys_user_online;

pub use common::get_captcha;
use silent::prelude::{HandlerAppend, Route};
pub use sys_user::login;
pub use sys_user_online::logout;

pub fn system_api() -> Route {
    Route::new("")
        .append(Route::new("user").append(sys_user_api())) // 用户管理模块
        .append(Route::new("dict/type").append(sys_dict_type_api())) // 字典类型模块
        .append(Route::new("dict/data").append(sys_dict_data_api())) // 字典数据模块
        .append(Route::new("post").append(sys_post_api())) // 岗位模块
        .append(Route::new("dept").append(sys_dept_api())) // 部门模块
        .append(Route::new("role").append(sys_role_api())) // 角色模块
        .append(Route::new("menu").append(sys_menu_api())) // 路由 菜单 模块
        .append(Route::new("login-log").append(sys_login_log_api())) // 登录日志模块
        .append(Route::new("online").append(sys_user_online_api())) // 在线用户
        .append(Route::new("job").append(sys_job_api())) // 定时任务
        .append(Route::new("job_log").append(sys_job_log_api())) // 定时任务日志
        .append(Route::new("oper_log").append(sys_oper_log_api())) // 操作日志
        .append(Route::new("api_db").append(sys_api_db_api())) // 操作日志
        .append(Route::new("monitor").append(sys_monitor_api())) // 操作日志
        .append(Route::new("update_log").append(sys_update_log_api())) // 更新日志
}

fn sys_user_api() -> Route {
    Route::new("")
        .append(Route::new("list").get(sys_user::get_sort_list)) // 获取全部用户
        .append(Route::new("get_by_id").get(sys_user::get_by_id)) // 按id获取用户
        .append(Route::new("get_profile").get(sys_user::get_profile)) // 按当前获取用户信息
        .append(Route::new("update_profile").put(sys_user::update_profile)) // 更新用户信息
        .append(Route::new("add").post(sys_user::add)) // 添加用户
        .append(Route::new("edit").put(sys_user::edit)) // 更新用户
        .append(Route::new("delete").delete(sys_user::delete)) // 硬删除用户
        .append(Route::new("get_info").get(sys_user::get_info)) // 获取用户信息
        .append(Route::new("reset_passwd").put(sys_user::reset_passwd)) // 重置密码
        .append(Route::new("update_passwd").put(sys_user::update_passwd)) // 重置密码
        .append(Route::new("change_status").put(sys_user::change_status)) // 修改状态
        .append(Route::new("change_role").put(sys_user::change_role)) // 切换角色
        .append(Route::new("change_dept").put(sys_user::change_dept)) // 切换部门
        .append(Route::new("fresh_token").put(sys_user::fresh_token)) // 修改状态
        .append(Route::new("update_avatar").post(sys_user::update_avatar)) // 修改头像
}

fn sys_dict_type_api() -> Route {
    Route::new("")
        .append(Route::new("list").get(sys_dict_type::get_sort_list)) // 获取全部字典类型
        .append(Route::new("get_all").get(sys_dict_type::get_all)) // 获取全部字典类型
        .append(Route::new("get_by_id").get(sys_dict_type::get_by_id)) // 按id获取字典类型
        .append(Route::new("add").post(sys_dict_type::add)) // 添加字典类型
        .append(Route::new("edit").put(sys_dict_type::edit)) // 更新字典类型
        .append(Route::new("delete").delete(sys_dict_type::delete)) // 硬删除字典类型
}

fn sys_dict_data_api() -> Route {
    Route::new("")
        .append(Route::new("list").get(sys_dict_data::get_sort_list)) // 获取全部字典数据
        .append(Route::new("get_all").get(sys_dict_data::get_all)) // 获取全部字典数据
        .append(Route::new("get_by_id").get(sys_dict_data::get_by_id)) // 按id获取字典数据
        .append(Route::new("get_by_type").get(sys_dict_data::get_by_type)) // 按id获取字典数据
        .append(Route::new("add").post(sys_dict_data::add)) // 添加字典数据
        .append(Route::new("edit").put(sys_dict_data::edit)) // 更新字典数据
        .append(Route::new("delete").delete(sys_dict_data::delete)) // 硬删除字典数据
}

fn sys_post_api() -> Route {
    Route::new("")
        .append(Route::new("list").get(sys_post::get_sort_list)) // 获取全部岗位
        .append(Route::new("get_all").get(sys_post::get_all)) // 获取全部岗位
        .append(Route::new("get_by_id").get(sys_post::get_by_id)) // 按id获取岗位
        .append(Route::new("add").post(sys_post::add)) // 添加岗位
        .append(Route::new("edit").put(sys_post::edit)) // 更新岗位
        .append(Route::new("delete").delete(sys_post::delete)) // 硬删除岗位
}

fn sys_dept_api() -> Route {
    Route::new("")
        .append(Route::new("list").get(sys_dept::get_sort_list)) // 获取全部部门
        .append(Route::new("get_all").get(sys_dept::get_all)) // 获取全部部门
        .append(Route::new("get_dept_tree").get(sys_dept::get_dept_tree)) // 获取部门树
        .append(Route::new("get_by_id").get(sys_dept::get_by_id)) // 按id获取部门
        .append(Route::new("add").post(sys_dept::add)) // 添加部门
        .append(Route::new("edit").put(sys_dept::edit)) // 更新部门
        .append(Route::new("delete").delete(sys_dept::delete)) // 硬删除部门
}

fn sys_role_api() -> Route {
    Route::new("")
        .append(Route::new("list").get(sys_role::get_sort_list)) // 获取全部角色
        .append(Route::new("get_all").get(sys_role::get_all)) // 获取全部角色
        .append(Route::new("get_by_id").get(sys_role::get_by_id)) // 按id获取角色
        .append(Route::new("add").post(sys_role::add)) // 添加角色
        .append(Route::new("edit").put(sys_role::edit)) // 更新角色
        .append(Route::new("change_status").put(sys_role::change_status)) // 设置状态
        .append(Route::new("set_data_scope").put(sys_role::set_data_scope)) // 设置数据权限范围
        .append(Route::new("delete").delete(sys_role::delete)) // 硬删除角色
        .append(Route::new("get_role_menu").get(sys_role::get_role_menu)) // 获取角色菜单
        .append(Route::new("get_role_dept").get(sys_role::get_role_dept)) // 获取角色部门
}

fn sys_menu_api() -> Route {
    Route::new("")
        .append(Route::new("list").get(sys_menu::get_sort_list)) // 获取全部菜单
        .append(Route::new("get_by_id").get(sys_menu::get_by_id)) // 按id获取菜单
        .append(Route::new("add").post(sys_menu::add)) // 添加菜单
        .append(Route::new("edit").put(sys_menu::edit)) // 更新菜单
        .append(Route::new("update_log_cache_method").put(sys_menu::update_log_cache_method)) // 更新api缓存方式和日志记录方式
        .append(Route::new("delete").delete(sys_menu::delete)) // 硬删除菜单
        .append(Route::new("get_all_enabled_menu_tree").get(sys_menu::get_all_enabled_menu_tree)) // 获取全部正常的路由菜单树
        .append(Route::new("get_routers").get(sys_menu::get_routers)) // 获取用户菜单树
        .append(Route::new("get_auth_list").get(sys_menu::get_related_api_and_db))
}

fn sys_login_log_api() -> Route {
    Route::new("")
        .append(Route::new("list").get(sys_login_log::get_sort_list)) // 获取全部登录日志
        .append(Route::new("clean").delete(sys_login_log::clean)) // 清空登录日志
        .append(Route::new("delete").delete(sys_login_log::delete)) // 硬删除登录日志
}
fn sys_user_online_api() -> Route {
    Route::new("")
        .append(Route::new("list").get(sys_user_online::get_sort_list)) // 获取全部在线用户
        .append(Route::new("delete").delete(sys_user_online::delete)) // 硬删除在线用户
}

fn sys_job_api() -> Route {
    Route::new("")
    // .append(Route::new("list").get(sys_job::get_sort_list)) // 获取全部定时任务
    // .append(Route::new("get_by_id").get(sys_job::get_by_id)) // 按id获取定时任务
    // .append(Route::new("change_status").put(sys_job::change_status)) // 设置状态
    // .append(Route::new("run_task_once").put(sys_job::run_task_once)) // 立即执行一次
    // .append(Route::new("add").post(sys_job::add)) // 添加定时任务
    // .append(Route::new("edit").put(sys_job::edit)) // 更新定时任务
    // .append(Route::new("delete").delete(sys_job::delete)) // 硬删除定时任务
    // .append(Route::new("validate_cron_str").post(sys_job::validate_cron_str))
    // 验证cron_str
}

fn sys_job_log_api() -> Route {
    Route::new("")
    // .append(Route::new("list").get(sys_job_log::get_sort_list)) // 获取全部定时任务日志
    // .append(Route::new("clean").delete(sys_job_log::clean)) // 清空定时任务日志
    // .append(Route::new("delete").delete(sys_job_log::delete)) // 硬删除定时任务日志
}
fn sys_oper_log_api() -> Route {
    Route::new("")
        .append(Route::new("list").get(sys_oper_log::get_sort_list)) // 获取全部操作日志
        .append(Route::new("get_by_id").get(sys_oper_log::get_by_id)) // 按id获取操作日志
        .append(Route::new("clean").delete(sys_oper_log::clean)) // 清空操作日志
        .append(Route::new("delete").delete(sys_oper_log::delete)) // 硬删除操作日志
}
fn sys_api_db_api() -> Route {
    Route::new("")
    // .append(Route::new("get_by_id").get(sys_api_db::get_by_id)) // 按id获取
    // .append(Route::new("add").post(sys_api_db::add)) // 添加
}

fn sys_monitor_api() -> Route {
    Route::new("")
        .append(Route::new("server").get(common::get_server_info)) // 服务器信息
        .append(Route::new("server-event").get(common::get_server_info_sse)) // 服务器信息(SSE)
}

fn sys_update_log_api() -> Route {
    Route::new("")
    // .append(Route::new("add").get(sys_update_log::add)) // 添加
    // .append(Route::new("edit").get(sys_update_log::edit)) // 更新
    // .append(Route::new("delete").get(sys_update_log::delete)) // 硬删除
    // .append(Route::new("get_all").get(sys_update_log::get_all)) // 获取全部
}
