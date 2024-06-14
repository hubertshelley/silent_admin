use silent::{Request, Result};

use app_service::system;
use db::system::models::sys_api_db::SysApiDbSearchReq;
use db::{common::res::Res, db_conn, system::prelude::SysApiDbModel, DB};

/// add 添加

pub async fn add(mut req: Request) -> Result<Res<String>> {
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_api_db::add(db, req).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// 按id获取
/// db 数据库连接

pub async fn get_by_id(mut req: Request) -> Result<Res<Vec<SysApiDbModel>>> {
    let req: SysApiDbSearchReq = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_api_db::get_by_id(db, &req.api_id).await;
    Ok(match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}
