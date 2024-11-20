use silent::{Request, Result};

use dto::common::res::ListData;
use entity::sys_login_info_record;
use services::system;

/// get_list 获取列表
/// page_params 分页参数

pub async fn get_sort_list(mut req: Request) -> Result<ListData<sys_login_info_record::Model>> {
    let page_params = req.params_parse()?;
    let params = req.params_parse()?;
    let db = req.get_config()?;
    let res = system::sys_login_log::get_sort_list(db, page_params, params).await;
    res.map_err(|e| e.into())
}

pub async fn delete(mut req: Request) -> Result<String> {
    let delete_req = req.json_parse().await?;
    let db = req.get_config()?;
    let res = system::sys_login_log::delete(db, delete_req).await;
    res.map_err(|e| e.into())
}

pub async fn clean(req: Request) -> Result<String> {
    let db = req.get_config()?;
    let res = system::sys_login_log::clean(db).await;
    res.map_err(|e| e.into())
}
