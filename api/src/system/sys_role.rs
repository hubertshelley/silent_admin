use common::middlewares::authorization::User;
use dto::common::res::ListData;
use dto::system::sys_role::{SysRoleResp, SysRoleSearchReq};
use entity::sys_role;
use services::system;
use silent::{Request, Result, SilentError};

/// get_list 获取列表
/// page_params 分页参数

pub async fn get_sort_list(mut req: Request) -> Result<ListData<sys_role::Model>> {
    let page_params = req.params_parse()?;
    let params = req.params_parse()?;
    let db = req.get_config()?;
    let res = system::sys_role::get_sort_list(db, page_params, params).await;
    res.map_err(|e| e.into())
}

/// add 添加

pub async fn add(mut req: Request) -> Result<String> {
    let params = req.json_parse().await?;
    let db = req.get_config()?;
    let user = req.extensions().get::<User>().unwrap();
    let res = system::sys_role::add(db, params, &user.id()?).await;
    res.map_err(|e| e.into())
}

/// delete 完全删除

pub async fn delete(mut req: Request) -> Result<String> {
    let delete_req = req.json_parse().await?;
    let db = req.get_config()?;
    let res = system::sys_role::delete(db, delete_req).await;
    res.map_err(|e| e.into())
}

// edit 修改

pub async fn edit(mut req: Request) -> Result<String> {
    let edit_req = req.json_parse().await?;
    let db = req.get_config()?;
    let user = req.extensions().get::<User>().unwrap();
    let res = system::sys_role::edit(db, edit_req, &user.id()?).await;
    res.map_err(|e| e.into())
}

// set_status 修改状态

pub async fn change_status(mut req: Request) -> Result<String> {
    let params = req.params_parse()?;
    let db = req.get_config()?;
    let res = system::sys_role::set_status(db, params).await;
    res.map_err(|e| e.into())
}
// set_data_scope 修改数据权限范围

pub async fn set_data_scope(mut req: Request) -> Result<String> {
    let params = req.json_parse().await?;
    let db = req.get_config()?;
    let res = system::sys_role::set_data_scope(db, params).await;
    res.map_err(|e| e.into())
}

/// get_user_by_id 获取用户Id获取用户

pub async fn get_by_id(mut req: Request) -> Result<SysRoleResp> {
    let params = req.params_parse()?;
    let db = req.get_config()?;
    let res = system::sys_role::get_by_id(db, params).await;
    res.map_err(|e| e.into())
}

/// get_all 获取全部

pub async fn get_all(req: Request) -> Result<Vec<SysRoleResp>> {
    let db = req.get_config()?;
    let res = system::sys_role::get_all(db).await;
    res.map_err(|e| e.into())
}

/// get_role_menu 获取角色授权菜单id数组

pub async fn get_role_menu(mut req: Request) -> Result<Vec<String>> {
    let params: SysRoleSearchReq = req.params_parse()?;
    let db = req.get_config()?;
    let role_id = params
        .role_id
        .ok_or::<SilentError>("role_id不能为空".to_string().into())?;
    let api_ids = match system::sys_menu::get_role_permissions(db, &role_id).await {
        Ok((_, x)) => x,
        Err(e) => return Err(e.into()),
    };
    Ok(api_ids)
}

/// get_role_dept 获取角色授权部门id数组

pub async fn get_role_dept(mut req: Request) -> Result<Vec<String>> {
    let params: SysRoleSearchReq = req.params_parse()?;
    let db = req.get_config()?;
    let role_id = params
        .role_id
        .ok_or::<SilentError>("role_id不能为空".to_string().into())?;
    let res = system::sys_dept::get_dept_by_role_id(db, role_id).await;
    res.map_err(|e| e.into())
}

// pub async fn get_auth_users_by_role_id(Query(mut req): Query<UserSearchReq>,
// Query(page_params): Query<PageParams>) -> Res<ListData<UserWithDept>> {
//     let db = req.get_config()?;
//     let role_id = match req.role_id.clone() {
//         None => return Res::with_err("角色Id不能为空"),
//         Some(id) => id,
//     };
//     let user_ids = match system::sys_role::get_auth_users_by_role_id(db,
// &role_id).await {         Ok(x) => x,
//         Err(e) => return Res::with_err(&e.to_string()),
//     };
//     req.user_ids = Some(user_ids.join(","));
//     let res = system::sys_user::get_sort_list(db, page_params, req).await;
//     match res {
//         Ok(x) => Res::with_data(x),
//         Err(e) => Res::with_err(&e.to_string()),
//     }
// }

// pub async fn get_un_auth_users_by_role_id(Query(mut req):
// Query<UserSearchReq>, Query(page_params): Query<PageParams>) ->
// Res<ListData<UserResp>> {     let db = req.get_config()?;
//     let role_id = match req.role_id.clone() {
//         None => return Res::with_err("角色Id不能为空"),
//         Some(id) => id,
//     };
//     let user_ids = match system::sys_role::get_auth_users_by_role_id(db,
// &role_id).await {         Ok(x) => x,
//         Err(e) => return Res::with_err(&e.to_string()),
//     };
//     req.user_ids = Some(user_ids.join(","));
//     let res = system::sys_user::get_un_auth_user(db, page_params, req).await;
//     match res {
//         Ok(x) => Res::with_data(x),
//         Err(e) => Res::with_err(&e.to_string()),
//     }
// }

// // edit 修改

// pub async fn update_auth_role(user: Claims, Json(req):
// Json<UpdateAuthRoleReq>) -> Res<String> {     let db =
// req.get_config()?;
//     match system::sys_role::add_role_by_user_id(db, &req.user_id,
// req.role_ids, user.id).await {         Ok(_) =>
// Res::with_msg("角色授权更新成功"),         Err(e) =>
// Res::with_err(&e.to_string()),     }
// }

// pub async fn add_auth_user(user: Claims, Json(req):
// Json<AddOrCancelAuthRoleReq>) -> Res<String> {     let db =
// req.get_config()?;     let res =
// system::sys_role::add_role_with_user_ids(db, req.clone().user_ids,
// req.role_id, user.id).await;     match res {
//         Ok(_) => Res::with_msg("授权成功"),
//         Err(e) => Res::with_err(&e.to_string()),
//     }
// }

// pub async fn cancel_auth_user(Json(req): Json<AddOrCancelAuthRoleReq>) ->
// Res<String> {     let db = req.get_config()?;
//     let res = system::sys_role::cancel_auth_user(db, req).await;
//     match res {
//         Ok(_) => Res::with_msg("取消授权成功"),
//         Err(e) => Res::with_err(&e.to_string()),
//     }
// }
