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

pub async fn get_sort_list(mut req: Request) -> Result<Res<ListData<UserWithDept>>> {
    let page_params = req.params_parse()?;
    let req = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::get_sort_list(db, page_params, req).await;
    let res = match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    };
    Ok(res)
}

/// get_user_by_id 获取用户Id获取用户

pub async fn get_by_id(mut req: Request) -> Result<Res<UserInformation>> {
    let req: SysUserSearchReq = req.params_parse()?;
    let db = DB.get_or_init(db_conn).await;
    let res = match req.user_id {
        Some(user_id) => match system::sys_user::get_user_info_by_id(db, &user_id).await {
            Err(e) => Res::with_err(&e.to_string()),
            Ok(res) => Res::with_data(res),
        },
        None => Res::with_msg("用户id不能为空"),
    };
    Ok(res)
}

pub async fn get_profile(mut req: Request) -> Result<Res<UserInformation>> {
    let user = Claims::from_request_parts(&mut req).await?;
    let db = DB.get_or_init(db_conn).await;
    let res = match system::sys_user::get_user_info_by_id(db, &user.id).await {
        Err(e) => Res::with_err(&e.to_string()),
        Ok(res) => Res::with_data(res),
    };
    Ok(res)
}

/// add 添加

pub async fn add(mut req: Request) -> Result<Res<String>> {
    let user = Claims::from_request_parts(&mut req).await?;
    let add_req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::add(db, add_req, user.id).await;
    let res = match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    };
    Ok(res)
}

/// delete 完全删除

pub async fn delete(mut req: Request) -> Result<Res<String>> {
    let delete_req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::delete(db, delete_req).await;
    let res = match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    };
    Ok(res)
}

// edit 修改

pub async fn edit(mut req: Request) -> Result<Res<String>> {
    let user = Claims::from_request_parts(&mut req).await?;
    let edit_req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::edit(db, edit_req, user.id).await;
    let res = match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    };
    Ok(res)
}

pub async fn update_profile(mut req: Request) -> Result<Res<String>> {
    let req: UpdateProfileReq = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::update_profile(db, req).await;
    let res = match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    };
    Ok(res)
}

/// 用户登录

pub async fn login(mut req: Request) -> Result<Res<AuthBody>> {
    let login_req: UserLoginReq = req.json_parse().await?;
    let header: HeaderMap = req.headers().clone();
    let db = DB.get_or_init(db_conn).await;
    let res = match system::sys_user::login(db, login_req, header).await {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    };
    Ok(res)
}
/// 获取用户登录信息

pub async fn get_info(mut req: Request) -> Result<Res<UserInfo>> {
    let user = Claims::from_request_parts(&mut req).await?;
    let db = DB.get_or_init(db_conn).await;

    let (role_ids_r, dept_ids_r, user_r) = join!(
        system::sys_user_role::get_role_ids_by_user_id(db, &user.id),
        system::sys_user_dept::get_dept_ids_by_user_id(db, &user.id),
        system::sys_user::get_user_info_permission(db, &user.id),
    );

    let roles = match role_ids_r {
        Ok(x) => x,
        Err(e) => return Ok(Res::with_err(&e.to_string())),
    };
    let depts = match dept_ids_r {
        Ok(x) => x,
        Err(e) => return Ok(Res::with_err(&e.to_string())),
    };
    let (user, permissions) = match user_r {
        Ok((x, y)) => (x, y),
        Err(e) => return Ok(Res::with_err(&e.to_string())),
    };

    let res = UserInfo { user, roles, depts, permissions };

    Ok(Res::with_data(res))
}

// edit 修改

pub async fn reset_passwd(mut req: Request) -> Result<Res<String>> {
    let req: ResetPwdReq = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::reset_passwd(db, req).await;
    let res = match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    };
    Ok(res)
}

pub async fn update_passwd(mut req: Request) -> Result<Res<String>> {
    let user = Claims::from_request_parts(&mut req).await?;
    let req = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::update_passwd(db, req, &user.id).await;
    let res = match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    };
    Ok(res)
}

// edit 修改

pub async fn change_status(mut req: Request) -> Result<Res<String>> {
    let req: ChangeStatusReq = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::change_status(db, req).await;
    let res = match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    };
    Ok(res)
}
// fresh_token 刷新token

pub async fn fresh_token(mut req: Request) -> Result<Res<AuthBody>> {
    let user = Claims::from_request_parts(&mut req).await?;
    let res = system::sys_user::fresh_token(user).await;
    let res = match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    };
    Ok(res)
}

pub async fn change_role(mut req: Request) -> Result<Res<String>> {
    let req: ChangeRoleReq = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::change_role(db, req).await;
    let res = match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    };
    Ok(res)
}

pub async fn change_dept(mut req: Request) -> Result<Res<String>> {
    let req: ChangeDeptReq = req.json_parse().await?;
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::change_dept(db, req).await;
    let res = match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    };
    Ok(res)
}

pub async fn update_avatar(mut req: Request) -> Result<Res<String>> {
    let user = Claims::from_request_parts(&mut req).await?;
    let file_part = req
        .files("files")
        .await
        .ok_or(SilentError::business_error(StatusCode::BAD_REQUEST, "请上传文件".to_string()))?
        .first();
    if file_part.is_none() {
        return Ok(Res::with_msg("请上传文件"));
    }
    let file_part = file_part.unwrap();
    let res = system::common::upload_file(file_part).await;
    match res {
        Ok(x) => {
            let db = DB.get_or_init(db_conn).await;
            let res = system::sys_user::update_avatar(db, &x, &user.id).await;
            match res {
                Ok(y) => Ok(Res::with_data_msg(x, &y)),
                Err(e) => Ok(Res::with_err(&e.to_string())),
            }
        }
        Err(e) => Ok(Res::with_err(&e.to_string())),
    }
}
