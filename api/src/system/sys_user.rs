use common::jwt::AuthBody;
use common::middlewares::authorization::User;
use dto::system::sys_user::PostRoleListResp;
use dto::{
    common::res::ListData,
    system::sys_user::{
        ChangeDeptReq, ChangeRoleReq, ChangeStatusReq, ResetPwdReq,
        UpdateProfileReq, UserInfo, UserInformation, UserLoginReq, UserWithDept,
    },
};
use services::system;
use silent::header::HeaderMap;
use silent::Result;
use silent::{Request, SilentError};
use tokio::join;

/// 获取岗位角色列表
pub async fn get_post_role_list(req: Request) -> Result<PostRoleListResp> {
    let db = req.get_config()?;
    let res = system::sys_user::get_post_role_list(db).await;
    res.map_err(|e| SilentError::from(e))
}
pub async fn get_sort_list(mut req: Request) -> Result<ListData<UserWithDept>> {
    let page_params = req.params_parse()?;
    let params = req.params_parse()?;
    let db = req.get_config()?;
    let res = system::sys_user::get_sort_list(db, page_params, params).await;
    res.map_err(|e| e.into())
}

/// get_user_by_id 获取用户Id获取用户

pub async fn get_by_id(req: Request) -> Result<UserInformation> {
    let user_id: String = req.get_path_params("id")?;
    let db = req.get_config()?;
    let res = system::sys_user::get_user_info_by_id(db, &user_id).await;
    res.map_err(|e| e.into())
}

pub async fn get_profile(req: Request) -> Result<UserInformation> {
    let db = req.get_config()?;
    let user = req.extensions().get::<User>().unwrap();
    let res = system::sys_user::get_user_info_by_id(db, &user.id()?).await;
    res.map_err(|e| e.into())
}

/// add 添加

pub async fn add(mut req: Request) -> Result<String> {
    let add_req = req.json_parse().await?;
    let user = req.extensions().get::<User>().unwrap();
    let db = req.get_config()?;
    let res = system::sys_user::add(db, add_req, user.id()?).await;
    res.map_err(|e| e.into())
}

/// delete 完全删除

pub async fn delete(mut req: Request) -> Result<String> {
    let delete_req = req.json_parse().await?;
    let db = req.get_config()?;
    let res = system::sys_user::delete(db, delete_req).await;
    res.map_err(|e| e.into())
}

// edit 修改

pub async fn edit(mut req: Request) -> Result<String> {
    let edit_req = req.json_parse().await?;
    let user = req.extensions().get::<User>().unwrap();
    let db = req.get_config()?;
    let res = system::sys_user::edit(db, edit_req, user.id()?).await;
    res.map_err(|e| e.into())
}

pub async fn update_profile(mut req: Request) -> Result<String> {
    let params: UpdateProfileReq = req.json_parse().await?;
    let db = req.get_config()?;
    let res = system::sys_user::update_profile(db, params).await;
    res.map_err(|e| e.into())
}

/// 用户登录

pub async fn login(mut req: Request) -> Result<AuthBody> {
    let login_req: UserLoginReq = req.json_parse().await?;
    let header: HeaderMap = req.headers().clone();
    let db = req.get_config()?;
    let res = system::sys_user::login(db, login_req, header).await;
    res.map_err(|e| e.into())
}
/// 获取用户登录信息

pub async fn get_info(req: Request) -> Result<UserInfo> {
    let db = req.get_config()?;
    let user = req.extensions().get::<User>().unwrap();
    let user_id = user.id()?;
    let (role_ids_r, dept_ids_r, user_r) = join!(
        system::sys_user_role::get_role_ids_by_user_id(db, user_id.as_str()),
        system::sys_user_dept::get_dept_ids_by_user_id(db, user_id.as_str()),
        system::sys_user::get_user_info_permission(db, user_id.as_str()),
    );

    let roles = role_ids_r?;
    let depts = dept_ids_r?;
    let (user, permissions) = user_r?;

    let res = UserInfo {
        user,
        roles,
        depts,
        permissions,
    };

    Ok(res)
}

// edit 修改

pub async fn reset_passwd(mut req: Request) -> Result<String> {
    let params: ResetPwdReq = req.json_parse().await?;
    let db = req.get_config()?;
    let res = system::sys_user::reset_passwd(db, params).await;
    res.map_err(|e| e.into())
}

pub async fn update_passwd(mut req: Request) -> Result<String> {
    let params = req.json_parse().await?;
    let user = req.extensions().get::<User>().unwrap();
    let db = req.get_config()?;
    let res = system::sys_user::update_passwd(db, params, &user.id()?).await;
    res.map_err(|e| e.into())
}

// edit 修改

pub async fn change_status(mut req: Request) -> Result<String> {
    let params: ChangeStatusReq = req.json_parse().await?;
    let db = req.get_config()?;
    let res = system::sys_user::change_status(db, params).await;
    res.map_err(|e| e.into())
}
// fresh_token 刷新token

pub async fn fresh_token(req: Request) -> Result<AuthBody> {
    let user = req.extensions().get::<User>().unwrap();
    let res = system::sys_user::fresh_token(user).await;
    res.map_err(|e| e.into())
}

pub async fn change_role(mut req: Request) -> Result<String> {
    let params: ChangeRoleReq = req.json_parse().await?;
    let db = req.get_config()?;
    let res = system::sys_user::change_role(db, params).await;
    res.map_err(|e| e.into())
}

pub async fn change_dept(mut req: Request) -> Result<String> {
    let params: ChangeDeptReq = req.json_parse().await?;
    let db = req.get_config()?;
    let res = system::sys_user::change_dept(db, params).await;
    res.map_err(|e| e.into())
}

pub async fn update_avatar(mut req: Request) -> Result<String> {
    let file_part = req
        .files("files")
        .await
        .ok_or::<SilentError>("请上传文件".to_string().into())?
        .first();
    let file_part = file_part.ok_or::<SilentError>("请上传文件".to_string().into())?;
    let res = system::common::upload_file(file_part).await?;
    let user = req.extensions().get::<User>().unwrap();
    let db = req.get_config()?;
    let res = system::sys_user::update_avatar(db, &res, &user.id()?).await;
    res.map_err(|e| e.into())
}
