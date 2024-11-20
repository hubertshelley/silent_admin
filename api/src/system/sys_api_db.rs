use silent::{Request, Result};

use services::system;
use dto::system::models::sys_api_db::SysApiDbSearchReq;
use dto::{db_conn, system::prelude::SysApiDbModel, DB};

/// add 添加

pub async fn add(mut req: Request) -> Result<String> {
    let req = req.json_parse().await?;
    let db = req.get_config()?;
    system::sys_api_db::add(db, req).await.map_err(|e| e.into())
}

/// 按id获取
/// db 数据库连接

pub async fn get_by_id(mut req: Request) -> Result<Vec<SysApiDbModel>> {
    let req: SysApiDbSearchReq = req.params_parse()?;
    let db = req.get_config()?;
    let res = system::sys_api_db::get_by_id(db, &req.api_id).await;
    res.map_err(|e| e.into())
}
