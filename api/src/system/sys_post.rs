use common::middlewares::authorization::User;
use dto::common::res::ListData;
use dto::system::sys_post::SysPostResp;
use entity::sys_post;
use services::system;
use silent::{Request, Result};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0

pub async fn get_sort_list(mut req: Request) -> Result<ListData<sys_post::Model>> {
    let page_params = req.params_parse()?;
    let params = req.params_parse()?;
    let db = req.get_config()?;

    let res = system::sys_post::get_sort_list(db, page_params, params).await;
    res.map_err(|e| e.into())
}

/// add 添加

pub async fn add(mut req: Request) -> Result<String> {
    let params = req.json_parse().await?;
    let db = req.get_config()?;
    let user = req.extensions().get::<User>().unwrap();
    let res = system::sys_post::add(db, params, user.id()?).await;
    res.map_err(|e| e.into())
}

/// delete 完全删除

pub async fn delete(mut req: Request) -> Result<String> {
    let params = req.json_parse().await?;
    let db = req.get_config()?;
    let res = system::sys_post::delete(db, params).await;
    res.map_err(|e| e.into())
}

// edit 修改

pub async fn edit(mut req: Request) -> Result<String> {
    let params = req.json_parse().await?;
    let db = req.get_config()?;
    let user = req.extensions().get::<User>().unwrap();
    let res = system::sys_post::edit(db, params, user.id()?).await;
    res.map_err(|e| e.into())
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_by_id(mut req: Request) -> Result<SysPostResp> {
    let params = req.params_parse()?;
    let db = req.get_config()?;
    let res = system::sys_post::get_by_id(db, params).await;
    res
}

/// get_all 获取全部
/// db 数据库连接 使用db.0

pub async fn get_all(req: Request) -> Result<Vec<SysPostResp>> {
    let db = req.get_config()?;
    let res = system::sys_post::get_all(db).await;
    res.map_err(|e| e.into())
}
