use silent::{Request, Result};

use app_service::{service_utils::jwt::Claims, system};
use db::{
    common::res::{ListData, Res},
    db_conn,
    system::prelude::SysDictDataModel,
    DB,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(mut req: Request) -> Result<Res<ListData<SysDictDataModel>>> {
    let page_params = req.params_parse()?;
    let req = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_data::get_sort_list(db, page_params, req).await;
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
    let res = system::sys_dict_data::add(db, req, user.id).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// delete 完全删除
pub async fn delete(mut req: Request) -> Result<Res<String>> {
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_data::delete(db, req).await;
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
    let res = system::sys_dict_data::edit(db, req, user.id).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_by_id(mut req: Request) -> Result<Res<SysDictDataModel>> {
    let req = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_data::get_by_id(db, req).await;
    Ok(match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_by_type(mut req: Request) -> Result<Res<Vec<SysDictDataModel>>> {
    let req = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    Ok(match system::sys_dict_data::get_by_type(db, req).await {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// get_all 获取全部
/// db 数据库连接 使用db.0

pub async fn get_all(_req: Request) -> Result<Res<Vec<SysDictDataModel>>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_data::get_all(db).await;
    Ok(match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}
