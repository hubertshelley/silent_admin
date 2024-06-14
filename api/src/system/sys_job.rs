use silent::{Request, Result};

use app_service::{service_utils::jwt::Claims, system, tasks};
use db::system::models::sys_job::JobId;
use db::{
    common::res::{ListData, Res},
    db_conn,
    system::{
        models::sys_job::{SysJobSearchReq, SysJobStatusReq, ValidateReq, ValidateRes},
        SysJobModel,
    },
    DB,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0

pub async fn get_sort_list(mut req: Request) -> Result<Res<ListData<SysJobModel>>> {
    let page_params = req.params_parse()?;
    let req = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job::get_sort_list(db, page_params, req).await;
    Ok(match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}
/// add 添加

pub async fn add(mut req: Request) -> Result<Res<String>> {
    let user = Claims::from_request_parts(&mut req).await?;
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job::add(db, req, user.id).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// delete 完全删除

pub async fn delete(mut req: Request) -> Result<Res<String>> {
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job::delete(db, req).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

// edit 修改

pub async fn edit(mut req: Request) -> Result<Res<String>> {
    let user = Claims::from_request_parts(&mut req).await?;
    let edit_req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job::edit(db, edit_req, user.id).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_by_id(mut req: Request) -> Result<Res<SysJobModel>> {
    let req: SysJobSearchReq = req.params_parse()?;
    let id = match req.job_id {
        None => return Ok(Res::with_err("id不能为空")),
        Some(x) => x,
    };
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job::get_by_id(db, id).await;
    Ok(match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

pub async fn change_status(mut req: Request) -> Result<Res<String>> {
    let req: SysJobStatusReq = req.json_parse().await?;
    //  数据验证
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job::set_status(db, req).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

pub async fn run_task_once(mut req: Request) -> Result<Res<String>> {
    let req: JobId = req.json_parse().await?;
    tasks::run_once_task(req.job_id, req.task_id, true).await;
    Ok(Res::with_msg("任务开始执行"))
}

pub async fn validate_cron_str(mut req: Request) -> Result<Res<ValidateRes>> {
    let req: ValidateReq = req.json_parse().await?;
    let res = system::sys_job::validate_cron_str(req.cron_str);
    Ok(match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}
