use common::middlewares::authorization::User;
use dto::common::res::ListData;
use entity::sys_user_online::Model as SysUserOnlineModel;
use services::system;
use silent::{Request, Result};

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(mut req: Request) -> Result<ListData<SysUserOnlineModel>> {
    let page_params = req.params_parse()?;
    let params = req.params_parse()?;
    let db = req.get_config()?;
    let res = system::sys_user_online::get_sort_list(db, page_params, params).await;
    res.map_err(|e| e.into())
}

/// 删除
pub async fn delete(mut req: Request) -> Result<String> {
    let delete_req = req.params_parse()?;
    let db = req.get_config()?;
    let res = system::sys_user_online::delete(db, delete_req).await;
    res.map_err(|e| e.into())
}

/// 登出
pub async fn log_out(req: Request) -> Result<String> {
    let user = req.extensions().get::<User>().unwrap();
    let db = req.get_config()?;
    let res = system::sys_user_online::log_out(db, user.id()?).await;
    res.map_err(|e| e.into())
}
