use silent::{Request, Result};

use app_service::{service_utils::jwt::Claims, system};
use db::{
    common::res::ListData,
    db_conn,
    system::{models::sys_post::SysPostResp, prelude::SysPostModel},
    DB,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0

pub async fn get_sort_list(mut req: Request) -> Result<ListData<SysPostModel>> {
    let page_params = req.params_parse()?;
    let req = req.params_parse()?;

    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_post::get_sort_list(db, page_params, req).await;
    res.map_err(|e| e.into())
}

/// add 添加

pub async fn add(mut req: Request) -> Result<String> {
    let user = Claims::from_request_parts(&mut req).await?;
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_post::add(db, req, user.id).await;
    res.map_err(|e| e.into())
}

/// delete 完全删除

pub async fn delete(mut req: Request) -> Result<String> {
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_post::delete(db, req).await;
    res.map_err(|e| e.into())
}

// edit 修改

pub async fn edit(mut req: Request) -> Result<String> {
    let user = Claims::from_request_parts(&mut req).await?;
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_post::edit(db, req, user.id).await;
    res.map_err(|e| e.into())
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_by_id(mut req: Request) -> Result<SysPostResp> {
    let req = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_post::get_by_id(db, req).await;
    res.map_err(|e| e.into())
}

/// get_all 获取全部
/// db 数据库连接 使用db.0

pub async fn get_all(_req: Request) -> Result<Vec<SysPostResp>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_post::get_all(db).await;
    res.map_err(|e| e.into())
}
