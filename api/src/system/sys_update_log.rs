use silent::{Request, Result};

use app_service::{service_utils::jwt::Claims, system};
use db::system::models::sys_update_log::SysUpdateLogDeleteReq;
use db::{db_conn, system::prelude::SysUpdateLogModel, DB};

/// add 添加
pub async fn add(mut req: Request) -> Result<String> {
    let user = Claims::from_request_parts(&mut req).await?;
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::add(db, req, &user.id).await;
    res.map_err(|e| e.into())
}

/// delete 完全删除
pub async fn delete(mut req: Request) -> Result<String> {
    let req: SysUpdateLogDeleteReq = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::soft_delete(db, &req.id).await;
    res.map_err(|e| e.into())
}

// edit 修改
pub async fn edit(mut req: Request) -> Result<String> {
    let user = Claims::from_request_parts(&mut req).await?;
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::edit(db, req, &user.id).await;
    res.map_err(|e| e.into())
}

/// get_all 获取全部
pub async fn get_all(_req: Request) -> Result<Vec<SysUpdateLogModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::get_all(db).await;
    res.map_err(|e| e.into())
}
