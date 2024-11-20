use silent::{Request, Result, SilentError};

use dto::common::res::ListData;
use dto::system::sys_oper_log::SysOperLogSearchReq;
use entity::sys_operate_log;
use services::system;

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0

pub async fn get_sort_list(mut req: Request) -> Result<ListData<sys_operate_log::Model>> {
    let page_params = req.params_parse()?;
    let params = req.params_parse()?;
    let db = req.get_config()?;
    let res = system::sys_oper_log::get_sort_list(db, page_params, params).await;
    res.map_err(|e| e.into())
}

/// delete 完全删除

pub async fn delete(mut req: Request) -> Result<String> {
    let params = req.json_parse().await?;
    let db = req.get_config()?;
    let res = system::sys_oper_log::delete(db, params).await;
    res.map_err(|e| e.into())
}

pub async fn clean(req: Request) -> Result<String> {
    //  数据验证
    let db = req.get_config()?;
    let res = system::sys_oper_log::clean(db).await;
    res.map_err(|e| e.into())
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_by_id(mut req: Request) -> Result<sys_operate_log::Model> {
    let params: SysOperLogSearchReq = req.params_parse()?;
    let db = req.get_config()?;
    //  数据验证
    let id = params
        .oper_id
        .ok_or::<SilentError>("id不能为空".to_string().into())?;
    let res = system::sys_oper_log::get_by_id(db, id).await;
    res.map_err(|e| e.into())
}
