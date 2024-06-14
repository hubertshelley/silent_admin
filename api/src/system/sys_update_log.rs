use silent::{Request, Result};

use app_service::{service_utils::jwt::Claims, system};
use db::system::models::sys_update_log::SysUpdateLogDeleteReq;
use db::{common::res::Res, db_conn, system::prelude::SysUpdateLogModel, DB};

/// add 添加
pub async fn add(mut req: Request) -> Result<Res<String>> {
    let user = Claims::from_request_parts(&mut req).await?;
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::add(db, req, &user.id).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// delete 完全删除
pub async fn delete(mut req: Request) -> Result<Res<String>> {
    let req: SysUpdateLogDeleteReq = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::soft_delete(db, &req.id).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

// edit 修改
pub async fn edit(mut req: Request) -> Result<Res<String>> {
    let user = Claims::from_request_parts(&mut req).await?;
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::edit(db, req, &user.id).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// get_all 获取全部
pub async fn get_all(_req: Request) -> Result<Res<Vec<SysUpdateLogModel>>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::get_all(db).await;
    Ok(match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}
