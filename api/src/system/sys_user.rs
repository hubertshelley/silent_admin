use headers::HeaderMap;
use silent::Result;
use silent::{Request, SilentError, StatusCode};
use tokio::join;

use app_service::{
    service_utils::jwt::{AuthBody, Claims},
    system,
};
use db::{
    common::res::{ListData, Res},
    db_conn,
    system::models::sys_user::{
        ChangeDeptReq, ChangeRoleReq, ChangeStatusReq, ResetPwdReq, SysUserSearchReq, UpdateProfileReq, UserInfo, UserInformation, UserLoginReq, UserWithDept,
    },
    DB,
};

/// get_user_list 获取用户列表
/// page_params 分页参数

pub async fn get_sort_list(mut req: Request) -> Result<ListData<UserWithDept>> {
    let page_params = req.params_parse()?;
    let req = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::get_sort_list(db, page_params, req).await;
    res.map_err(|e| e.into())
}

/// get_user_by_id 获取用户Id获取用户

pub async fn get_by_id(mut req: Request) -> Result<UserInformation> {
    let req: SysUserSearchReq = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    let user_id = req.user_id.ok_or("用户id不能为空".into());
    let res = system::sys_user::get_user_info_by_id(db, &user_id).await;
    res.map_err(|e| e.into())
}

pub async fn get_profile(mut req: Request) -> Result<UserInformation> {
    let user = Claims::from_request_parts(&mut req).await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::get_user_info_by_id(db, &user.id).await;
    res.map_err(|e| e.into())
}

/// add 添加

pub async fn add(mut req: Request) -> Result<String> {
    let user = Claims::from_request_parts(&mut req).await?;
    let add_req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::add(db, add_req, user.id).await;
    res.map_err(|e| e.into())
}

/// delete 完全删除

pub async fn delete(mut req: Request) -> Result<String> {
    let delete_req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::delete(db, delete_req).await;
    res.map_err(|e| e.into())
}

// edit 修改

pub async fn edit(mut req: Request) -> Result<String> {
    let user = Claims::from_request_parts(&mut req).await?;
    let edit_req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::edit(db, edit_req, user.id).await;
    res.map_err(|e| e.into())
}

pub async fn update_profile(mut req: Request) -> Result<String> {
    let req: UpdateProfileReq = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::update_profile(db, req).await;
    res.map_err(|e| e.into())
}

/// 用户登录

pub async fn login(mut req: Request) -> Result<AuthBody> {
    let login_req: UserLoginReq = req.json_parse().await?;
    let header: HeaderMap = req.headers().clone();
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::login(db, login_req, header).await;
    res.map_err(|e| e.into())
}
/// 获取用户登录信息

pub async fn get_info(mut req: Request) -> Result<UserInfo> {
    let user = Claims::from_request_parts(&mut req).await?;
    let db = DB.get_or_init(db_conn).await;

    let (role_ids_r, dept_ids_r, user_r) = join!(
        system::sys_user_role::get_role_ids_by_user_id(db, &user.id),
        system::sys_user_dept::get_dept_ids_by_user_id(db, &user.id),
        system::sys_user::get_user_info_permission(db, &user.id),
    );

    let roles = role_ids_r?;
    let depts = dept_ids_r?;
    let (user, permissions) = user_r?;

    let res = UserInfo { user, roles, depts, permissions };

    Ok(res)
}

// edit 修改

pub async fn reset_passwd(mut req: Request) -> Result<String> {
    let req: ResetPwdReq = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::reset_passwd(db, req).await;
    res.map_err(|e| e.into())
}

pub async fn update_passwd(mut req: Request) -> Result<String> {
    let user = Claims::from_request_parts(&mut req).await?;
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::update_passwd(db, req, &user.id).await;
    res.map_err(|e| e.into())
}

// edit 修改

pub async fn change_status(mut req: Request) -> Result<String> {
    let req: ChangeStatusReq = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::change_status(db, req).await;
    res.map_err(|e| e.into())
}
// fresh_token 刷新token

pub async fn fresh_token(mut req: Request) -> Result<AuthBody> {
    let user = Claims::from_request_parts(&mut req).await?;
    let res = system::sys_user::fresh_token(user).await;
    res.map_err(|e| e.into())
}

pub async fn change_role(mut req: Request) -> Result<String> {
    let req: ChangeRoleReq = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::change_role(db, req).await;
    res.map_err(|e| e.into())
}

pub async fn change_dept(mut req: Request) -> Result<String> {
    let req: ChangeDeptReq = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::change_dept(db, req).await;
    res.map_err(|e| e.into())
}

pub async fn update_avatar(mut req: Request) -> Result<String> {
    let user = Claims::from_request_parts(&mut req).await?;
    let file_part = req.files("files").await.ok_or("请上传文件".into())?.first();
    let file_part = file_part.ok_or("请上传文件".into())?;
    let res = system::common::upload_file(file_part).await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::update_avatar(db, &res, &user.id).await;
    res.map_err(|e| e.into())
}
