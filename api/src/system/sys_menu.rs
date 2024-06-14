use silent::{Request, Result};

use app_service::{service_utils::jwt::Claims, system};
use configs::CFG;
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_menu::{LogCacheEditReq, MenuRelated, MenuResp, SysMenuDeleteReq, SysMenuEditReq, SysMenuSearchReq, SysMenuTree, SysMenuTreeAll},
        prelude::SysMenuModel,
    },
    DB,
};

/// get_all_menu_tree 获取全部菜单

pub async fn get_sort_list(mut req: Request) -> Result<Res<ListData<SysMenuModel>>> {
    let page_params = req.params_parse::<PageParams>()?;
    let search_req = req.params_parse::<SysMenuSearchReq>()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::get_sort_list(db, page_params, search_req).await;
    Ok(match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_by_id(mut req: Request) -> Result<Res<MenuResp>> {
    let search_req = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::get_by_id(db, search_req).await;
    Ok(match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// add 添加

pub async fn add(mut req: Request) -> Result<Res<String>> {
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::add(db, req).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// delete 完全删除

pub async fn delete(mut req: Request) -> Result<Res<String>> {
    let req: SysMenuDeleteReq = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::delete(db, &req.id).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

// edit 修改

pub async fn edit(mut req: Request) -> Result<Res<String>> {
    let edit_req: SysMenuEditReq = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::edit(db, edit_req).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}
// update_log_cache_method 修改菜单日志缓存方法
pub async fn update_log_cache_method(mut req: Request) -> Result<Res<String>> {
    let edit_req: LogCacheEditReq = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::update_log_cache_method(db, edit_req).await;
    Ok(match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// get_all_menu_tree 获取全部菜单树

pub async fn get_all_enabled_menu_tree(mut req: Request) -> Result<Res<Vec<SysMenuTreeAll>>> {
    let page_params = req.params_parse::<PageParams>()?;
    let search_req = req.params_parse::<SysMenuSearchReq>()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::get_all_enabled_menu_tree(db, page_params, search_req).await;
    Ok(match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// get_related_api_and_db 获取全部菜单树
pub async fn get_related_api_and_db(mut req: Request) -> Result<Res<ListData<MenuRelated>>> {
    let page_params = req.params_parse::<PageParams>()?;
    let search_req = req.params_parse::<SysMenuSearchReq>()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::get_related_api_and_db(db, page_params, search_req).await;
    Ok(match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}

/// 获取用户路由
pub async fn get_routers(mut req: Request) -> Result<Res<Vec<SysMenuTree>>> {
    let user = Claims::from_request_parts(&mut req).await?;
    let db = DB.get_or_init(db_conn).await;
    //  获取 用户角色
    let role_id = match system::sys_role::get_current_admin_role(db, &user.id).await {
        Ok(x) => x,
        Err(e) => return Ok(Res::with_err(&e.to_string())),
    };

    // 检查是否超管用户
    let res = if CFG.system.super_user.contains(&user.id) {
        system::sys_menu::get_all_router_tree(db).await
    } else {
        system::sys_menu::get_admin_menu_by_role_ids(db, &role_id).await
    };
    Ok(match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    })
}
