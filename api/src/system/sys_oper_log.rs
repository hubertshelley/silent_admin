use silent::{Request, Result};

use app_service::system;
use db::system::models::sys_oper_log::SysOperLogSearchReq;
use db::{
    common::res::{ListData, Res},
    db_conn,
    system::prelude::SysOperLogModel,
    DB,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0

pub async fn get_sort_list(mut req: Request) -> Result<Res<ListData<SysOperLogModel>>> {
    let page_params = req.params_parse()?;
    let req = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_oper_log::get_sort_list(db, page_params, req).await;
    Ok(match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// delete 完全删除

pub async fn delete(mut req: Request) -> Result<Res<String>> {
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_oper_log::delete(db, req).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

pub async fn clean(_req: Request) -> Result<Res<String>> {
    //  数据验证
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_oper_log::clean(db).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_by_id(mut req: Request) -> Result<Res<SysOperLogModel>> {
    let req: SysOperLogSearchReq = req.params_parse()?;
    let id = match req.oper_id {
        None => return Ok(Res::with_err("id不能为空")),
        Some(x) => x,
    };
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_oper_log::get_by_id(db, id).await;
    Ok(match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}
