use silent::{Request, Result};

use app_service::system;
use db::{
    common::res::{ListData, Res},
    db_conn,
    system::{models::sys_job_log::SysJobLogCleanReq, prelude::SysJobLogModel},
    DB,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0

pub async fn get_sort_list(mut req: Request) -> Result<ListData<SysJobLogModel>> {
    let page_params = req.params_parse()?;
    let req = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job_log::get_sort_list(db, page_params, req).await;
    res.map_err(|e| e.into())
}

/// delete 完全删除

pub async fn delete(mut req: Request) -> Result<String> {
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job_log::delete(db, req).await;
    res.map_err(|e| e.into())
}

pub async fn clean(mut req: Request) -> Result<String> {
    let req: SysJobLogCleanReq = req.json_parse().await?;
    //  数据验证
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job_log::clean(db, req.job_id).await;
    res.map_err(|e| e.into())
}
