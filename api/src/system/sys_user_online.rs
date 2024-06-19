use silent::{Request, Result};

use app_service::{service_utils::jwt::Claims, system};
use db::{
    common::res::{ListData, Res},
    db_conn,
    system::prelude::SysUserOnlineModel,
    DB,
};

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(mut req: Request) -> Result<ListData<SysUserOnlineModel>> {
    let page_params = req.params_parse()?;
    let req = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user_online::get_sort_list(db, page_params, req).await;
    res.map_err(|e| e.into())
}

/// 删除
pub async fn delete(mut req: Request) -> Result<String> {
    let delete_req = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user_online::delete(db, delete_req).await;
    res.map_err(|e| e.into())
}

/// 登出
pub async fn log_out(mut req: Request) -> Result<String> {
    let user = Claims::from_request_parts(&mut req).await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user_online::log_out(db, user.token_id).await;
    res.map_err(|e| e.into())
}
