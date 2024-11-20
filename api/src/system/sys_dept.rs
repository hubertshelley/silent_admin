use anyhow::anyhow;
use common::middlewares::authorization::User;
use dto::system::sys_dept::SysDeptSearchReq;
use dto::{
    common::res::ListData,
    system::sys_dept::{DeptResp, RespTree},
};
use entity::sys_dept;
use services::system;
use silent::{Request, Result, SilentError};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0

pub async fn get_sort_list(mut req: Request) -> Result<ListData<sys_dept::Model>> {
    let page_params = req.params_parse()?;
    let params = req.params_parse()?;
    let db = req.get_config()?;
    let res = system::sys_dept::get_sort_list(db, page_params, params).await;
    res.map_err(|e| e.into())
}
/// add 添加

pub async fn add(mut req: Request) -> Result<String> {
    let params = req.json_parse().await?;
    let user = req.extensions().get::<User>().unwrap();
    let db = req.get_config()?;
    let res = system::sys_dept::add(db, params, user.id()?).await;
    res.map_err(|e| e.into())
}

/// delete 完全删除

pub async fn delete(mut req: Request) -> Result<String> {
    let params = req.json_parse().await?;
    let db = req.get_config()?;
    let res = system::sys_dept::delete(db, params).await;
    res.map_err(|e| e.into())
}

// edit 修改

pub async fn edit(mut req: Request) -> Result<String> {
    let params = req.json_parse().await?;
    let user = req.extensions().get::<User>().unwrap();
    let db = req.get_config()?;
    let res = system::sys_dept::edit(db, params, user.id()?).await;
    res.map_err(|e| e.into())
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_by_id(mut req: Request) -> Result<DeptResp> {
    let params: SysDeptSearchReq = req.params_parse()?;
    let dept_id = params
        .dept_id
        .ok_or::<SilentError>(anyhow!("参数错误").into())?;
    let db = req.get_config()?;
    let res = system::sys_dept::get_by_id(db, &dept_id).await;
    res.map_err(|e| e.into())
}

/// get_all 获取全部
pub async fn get_all(req: Request) -> Result<Vec<DeptResp>> {
    let db = req.get_config()?;
    let res = system::sys_dept::get_all(db).await;
    res.map_err(|e| e.into())
}

pub async fn get_dept_tree(req: Request) -> Result<Vec<RespTree>> {
    let db = req.get_config()?;
    let res = system::sys_dept::get_dept_tree(db).await;
    res.map_err(|e| e.into())
}
