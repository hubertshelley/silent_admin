use silent::{Request, Result};

use services::{service_utils::jwt::Claims, system};
use dto::system::models::sys_update_log::SysUpdateLogDeleteReq;
use dto::{db_conn, system::prelude::SysUpdateLogModel, DB};

/// add 添加
pub async fn add(mut req: Request) -> Result<String> {
    let user = Claims::from_request_parts(&mut req).await?;
    let req = req.json_parse().await?;
    let db = req.get_config()?;
    let res = system::sys_update_log::add(db, req, &user.id).await;
    res.map_err(|e| e.into())
}

/// delete 完全删除
pub async fn delete(mut req: Request) -> Result<String> {
    let req: SysUpdateLogDeleteReq = req.json_parse().await?;
    let db = req.get_config()?;
    let res = system::sys_update_log::soft_delete(db, &req.id).await;
    res.map_err(|e| e.into())
}

// edit 修改
pub async fn edit(mut req: Request) -> Result<String> {
    let user = Claims::from_request_parts(&mut req).await?;
    let req = req.json_parse().await?;
    let db = req.get_config()?;
    let res = system::sys_update_log::edit(db, req, &user.id).await;
    res.map_err(|e| e.into())
}

/// get_all 获取全部
pub async fn get_all(_req: Request) -> Result<Vec<SysUpdateLogModel>> {
    let db = req.get_config()?;
    let res = system::sys_update_log::get_all(db).await;
    res.map_err(|e| e.into())
}
