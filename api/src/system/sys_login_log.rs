use silent::{Request, Result};

use app_service::system;
use db::{
    common::res::{ListData, Res},
    db_conn,
    system::prelude::SysLoginLogModel,
    DB,
};

/// get_list 获取列表
/// page_params 分页参数

pub async fn get_sort_list(mut req: Request) -> Result<Res<ListData<SysLoginLogModel>>> {
    let page_params = req.params_parse()?;
    let req = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_login_log::get_sort_list(db, page_params, req).await;
    Ok(match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

pub async fn delete(mut req: Request) -> Result<Res<String>> {
    let delete_req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_login_log::delete(db, delete_req).await;
    println!("{:?}", res);
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

pub async fn clean(_req: Request) -> Result<Res<String>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_login_log::clean(db).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}
