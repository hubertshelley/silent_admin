use crate::service_utils::{self};
use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use common::jwt::{AuthBody, AuthPayload};
use common::middlewares::authorization::User;
use common::snowflake::next_id;
use common::utils;
use configs::CFG;
use dto::system::sys_user::PostRoleListResp;
use dto::{
    common::res::{ListData, PageParams},
    system::{
        sys_dept::DeptResp,
        sys_user::{
            ChangeDeptReq, ChangeRoleReq, ChangeStatusReq, ResetPwdReq, SysUserAddReq,
            SysUserDeleteReq, SysUserEditReq, SysUserSearchReq, UpdateProfileReq, UpdatePwdReq,
            UserInformation, UserLoginReq, UserResp, UserWithDept,
        },
    },
};
use entity::prelude::SysUser;
use entity::{sys_dept, sys_user, sys_user_dept};
use sea_orm::{
    sea_query::Expr, ColumnTrait, DatabaseConnection, EntityTrait, JoinType, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
};
use silent::header::HeaderMap;
use silent::prelude::argon2::{make_password, verify_password};
use silent::SilentError;
use tokio::join;

/// 获取岗位角色列表
pub async fn get_post_role_list(db: &DatabaseConnection) -> Result<PostRoleListResp> {
    let roles = crate::system::sys_role::get_all(db)
        .await
        .map_err(Into::<SilentError>::into)?;
    let posts = crate::system::sys_post::get_all(db)
        .await
        .map_err(Into::<SilentError>::into)?;
    Ok(PostRoleListResp { roles, posts })
}

/// get_user_list 获取用户列表
/// page_params 分页参数
pub async fn get_sort_list(
    db: &DatabaseConnection,
    page_params: PageParams,
    req: SysUserSearchReq,
) -> Result<ListData<UserWithDept>> {
    let txn = db.begin().await?;
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    let mut s = SysUser::find()
        .join_rev(
            JoinType::LeftJoin,
            sys_user_dept::Entity::belongs_to(sys_user::Entity)
                .from(sys_user_dept::Column::UserId)
                .to(sys_user::Column::Id)
                .into(),
        )
        .join_rev(
            JoinType::LeftJoin,
            sys_dept::Entity::belongs_to(sys_user_dept::Entity)
                .from(sys_dept::Column::Id)
                .to(sys_user_dept::Column::DeptId)
                .into(),
        )
        .select_also(sys_dept::Entity);
    // 不查找删除数据
    s = s.filter(sys_user::Column::DelFlag.eq(0));
    // 查询条件
    if let Some(x) = req.user_id {
        if !x.is_empty() {
            s = s.filter(sys_user::Column::Id.eq(x));
        }
    }
    if let Some(x) = req.user_ids {
        if !x.is_empty() {
            let y: Vec<&str> = x.split(',').collect();
            s = s.filter(sys_user::Column::Id.is_in(y));
        }
    }

    if let Some(x) = req.user_name {
        if !x.is_empty() {
            s = s.filter(sys_user::Column::UserName.contains(&x));
        }
    }
    if let Some(x) = req.phone_num {
        if !x.is_empty() {
            s = s.filter(sys_user::Column::UserName.contains(&x));
        }
    }
    if let Some(x) = req.user_status {
        if !x.is_empty() {
            s = s.filter(sys_user::Column::Status.eq(x));
        }
    }
    if let Some(x) = req.dept_id {
        if !x.is_empty() {
            s = s.filter(sys_user_dept::Column::DeptId.eq(x));
        }
    }
    if let Some(x) = req.dept_ids {
        if !x.is_empty() {
            s = s.filter(sys_user_dept::Column::DeptId.is_in(x));
        }
    }
    if let Some(x) = req.begin_time {
        if !x.is_empty() {
            let x = x + " 00:00:00";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_user::Column::CreateTime.gte(t));
        }
    }
    if let Some(x) = req.end_time {
        if !x.is_empty() {
            let x = x + " 23:59:59";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_user::Column::CreateTime.lte(t));
        }
    }
    // 获取全部数据条数
    let total = s.clone().count(&txn).await?;
    // 获取全部数据条数
    let paginator = s
        .order_by_asc(sys_user::Column::Id)
        // .into_model::<UserResp>()
        .paginate(&txn, page_per_size);
    let total_pages = paginator.num_pages().await?;
    let users = paginator.fetch_page(page_num - 1).await?;
    let mut list: Vec<UserWithDept> = Vec::new();
    for m in users {
        let user_dept = match m.1 {
            Some(v) => UserWithDept {
                user: UserResp {
                    id: m.0.id.clone(),
                    user_name: m.0.user_name.clone(),
                    nick_name: m.0.nick_name.clone(),
                    status: m.0.status.clone().unwrap_or("0".to_owned()),
                    email: m.0.email.clone(),
                    sex: m.0.sex.clone().unwrap_or("0".to_owned()),
                    avatar: m.0.avatar.clone().unwrap_or("".to_owned()),
                    dept_id: m.0.dept_id.clone().unwrap_or("".to_owned()),
                    remark: m.0.remark.clone(),
                    admin: CFG.system.super_user.contains(&m.0.id),
                    phone_number: m.0.phone_number.clone(),
                    role_id: "".to_owned(),
                    create_time: Some(m.0.create_time),
                },
                dept: DeptResp {
                    id: v.id.clone(),
                    parent_id: v.parent_id.clone().unwrap_or("".to_owned()),
                    dept_name: v.dept_name.clone().unwrap_or("".to_owned()),
                    order_num: v.order_num.unwrap_or(0),
                    leader: v.leader.clone(),
                    phone: v.phone.clone(),
                    email: v.email.clone(),
                    status: v.status.clone().unwrap_or("0".to_owned()),
                    create_time: v.create_time,
                },
            },
            None => return Err(anyhow!("{}无部门信息", m.0.user_name)),
        };

        list.push(user_dept);
    }
    txn.commit().await?;
    let res = ListData {
        total,
        list,
        total_pages,
        page_num,
    };

    Ok(res)
}

pub async fn get_un_auth_user(
    db: &DatabaseConnection,
    page_params: PageParams,
    req: SysUserSearchReq,
) -> Result<ListData<UserResp>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    let mut s = SysUser::find();
    // 不查找删除数据
    s = s.filter(sys_user::Column::DelFlag.eq(0));
    // 查询条件
    if let Some(x) = req.user_ids {
        let y: Vec<&str> = x.split(',').collect();
        s = s.filter(sys_user::Column::Id.is_not_in(y));
    }
    if let Some(x) = req.user_name {
        s = s.filter(sys_user::Column::UserName.contains(x));
    }
    if let Some(x) = req.phone_num {
        s = s.filter(sys_user::Column::UserName.contains(x));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 获取全部数据条数
    let paginator = s
        .order_by_asc(sys_user::Column::Id)
        .into_model()
        .paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await?;
    let list = paginator.fetch_page(page_num - 1).await?;
    let res = ListData {
        total,
        list,
        total_pages,
        page_num,
    };
    Ok(res)
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接
pub async fn get_by_id(db: &DatabaseConnection, user_id: &str) -> Result<UserWithDept> {
    let user_s = SysUser::find()
        .filter(sys_user::Column::DelFlag.eq(0))
        .filter(sys_user::Column::Id.eq(user_id))
        .one(db)
        .await?;

    let user = match user_s {
        None => return Err(anyhow!("用户不存在")),
        Some(u) => {
            let dept_s = sys_dept::Entity::find()
                .filter(sys_dept::Column::Id.eq(u.dept_id.clone()))
                .one(db)
                .await?;
            match dept_s {
                None => return Err(anyhow!("{}无部门信息", user_id)),
                Some(v) => UserWithDept {
                    user: UserResp {
                        id: u.id.clone(),
                        user_name: u.user_name.clone(),
                        nick_name: u.nick_name.clone(),
                        status: u.status.clone().unwrap_or("0".to_owned()),
                        email: u.email.clone(),
                        sex: u.sex.clone().unwrap_or("0".to_owned()),
                        avatar: u.avatar.clone().unwrap_or("".to_owned()),
                        dept_id: u.dept_id.clone().unwrap_or("".to_owned()),
                        remark: u.remark.clone(),
                        admin: CFG.system.super_user.contains(&u.id),
                        phone_number: u.phone_number.clone(),
                        role_id: "".to_owned(),
                        create_time: Some(u.create_time),
                    },
                    dept: DeptResp {
                        id: v.id.clone(),
                        parent_id: v.parent_id.clone().unwrap_or("".to_owned()),
                        dept_name: v.dept_name.clone().unwrap_or("".to_owned()),
                        order_num: v.order_num.unwrap_or(0),
                        leader: v.leader.clone(),
                        phone: v.phone.clone(),
                        email: v.email.clone(),
                        status: v.status.unwrap_or("0".to_owned()),
                        create_time: v.create_time,
                    },
                },
            }
        }
    };

    // let s = SysUser::find()
    //     .join_rev(
    //         JoinType::LeftJoin,
    //         sys_dept::Entity::belongs_to(sys_user::Entity)
    //             .from(sys_dept::Column::DeptId)
    //             .to(sys_user::Column::DeptId)
    //             .into(),
    //     )
    //     .select_also(sys_dept::Entity)
    //     .filter(sys_user::Column::DeletedAt.is_null())
    //     .filter(sys_user::Column::Id.eq(user_id))
    //     .one(db)
    //     .await?;
    // let user = match s {
    //     Some(m) => match m.1 {
    //         Some(v) => UserWithDept {
    //             user: UserResp {
    //                 id: m.0.id.clone(),
    //                 user_name: m.0.user_name.clone(),
    //                 user_nickname: m.0.user_nickname.clone(),
    //                 user_status: m.0.user_status.clone(),
    //                 user_email: m.0.user_email.clone(),
    //                 sex: m.0.sex.clone(),
    //                 avatar: m.0.avatar.clone(),
    //                 dept_id: m.0.dept_id.clone(),
    //                 remark: m.0.remark.clone(),
    //                 is_admin: m.0.is_admin.clone(),
    //                 phone_num: m.0.phone_num.clone(),
    //                 role_id: m.0.role_id.clone(),
    //                 created_at: Some(m.0.created_at),
    //             },
    //             dept: DeptResp {
    //                 dept_id: v.dept_id.clone(),
    //                 parent_id: v.parent_id.clone(),
    //                 dept_name: v.dept_name.clone(),
    //                 order_num: v.order_num,
    //                 leader: v.leader.clone(),
    //                 phone: v.phone.clone(),
    //                 email: v.email.clone(),
    //                 status: v.status,
    //             },
    //         },
    //         None => return Err(anyhow!("{}无部门信息", user_id)),
    //     },
    //     None => return Err(anyhow!("用户不存在")),
    // };

    Ok(user)
}

/// add 添加
pub async fn add(db: &DatabaseConnection, req: SysUserAddReq, c_user_id: String) -> Result<String> {
    let uid = next_id()?;
    let passwd = make_password(req.password)?;
    let user = sys_user::ActiveModel {
        id: Set(uid.clone()),
        user_name: Set(req.user_name),
        nick_name: Set(req.nick_name),
        password: Set(Some(passwd)),
        status: Set(Some(req.status)),
        email: Set(req.email),
        sex: Set(Some(req.sex)),
        dept_id: Set(Some(req.dept_id.clone())),
        remark: Set(req.remark),
        phone_number: Set(req.phone_number),
        avatar: Set(req.avatar),
        ..Default::default()
    };

    let txn = db.begin().await?;
    SysUser::insert(user).exec(&txn).await?;
    // 添加职位信息
    super::sys_post::add_post_by_user_id(&txn, &uid, req.post_ids).await?;

    // 添加角色信息
    // 先删除原有的角色信息，再添加新的角色信息
    // super::sys_user_role::delete_user_role(&txn, &uid).await?;
    //
    // super::sys_user_role::edit_user_role(&txn, &uid, req.role_ids, &c_user_id).await?;
    // 删除原有部门信息
    super::sys_user_dept::delete_user_dept(&txn, &uid).await?;
    // 添加新的部门信息
    super::sys_user_dept::edit_user_dept(&txn, &uid, vec![req.dept_id], &c_user_id).await?;

    txn.commit().await?;

    Ok("用户添加成功".to_string())
}

pub async fn reset_passwd(db: &DatabaseConnection, req: ResetPwdReq) -> Result<String> {
    let passwd = make_password(req.new_passwd)?;
    let now: NaiveDateTime = Local::now().naive_local();
    let txn = db.begin().await?;
    // 更新用户信息
    SysUser::update_many()
        .col_expr(sys_user::Column::Password, Expr::value(passwd))
        .col_expr(sys_user::Column::UpdateTime, Expr::value(now))
        .filter(sys_user::Column::Id.eq(req.user_id))
        .exec(&txn)
        .await?;
    // user.update(&txn).await?;
    txn.commit().await?;
    let res = "密码更新成功".to_string();

    Ok(res)
}

pub async fn update_passwd(
    db: &DatabaseConnection,
    req: UpdatePwdReq,
    user_id: &str,
) -> Result<String> {
    match SysUser::find()
        .filter(sys_user::Column::Id.eq(user_id))
        .one(db)
        .await?
    {
        None => return Err(anyhow!("用户不存在")),
        Some(x) => {
            if !verify_password(x.password.unwrap_or("".to_string()), req.old_passwd)? {
                return Err(anyhow!("旧密码错误,请检查重新输入"));
            }
        }
    };
    reset_passwd(
        db,
        ResetPwdReq {
            user_id: user_id.to_string(),
            new_passwd: req.new_passwd,
        },
    )
    .await
}

pub async fn change_status(db: &DatabaseConnection, req: ChangeStatusReq) -> Result<String> {
    let now: NaiveDateTime = Local::now().naive_local();
    // 更新
    let txn = db.begin().await?;
    // 更新用户信息
    SysUser::update_many()
        .col_expr(sys_user::Column::Status, Expr::value(req.status))
        .col_expr(sys_user::Column::UpdateTime, Expr::value(now))
        .filter(sys_user::Column::Id.eq(req.user_id))
        .exec(&txn)
        .await?;
    // user.update(&txn).await?;
    txn.commit().await?;
    let res = "用户状态更新成功".to_string();

    Ok(res)
}

pub async fn update_profile(db: &DatabaseConnection, req: UpdateProfileReq) -> Result<String> {
    let now: NaiveDateTime = Local::now().naive_local();
    let txn = db.begin().await?;
    // 更新用户信息
    SysUser::update_many()
        .col_expr(sys_user::Column::NickName, Expr::value(req.user_nickname))
        .col_expr(sys_user::Column::PhoneNumber, Expr::value(req.phone_num))
        .col_expr(sys_user::Column::Email, Expr::value(req.user_email))
        .col_expr(sys_user::Column::Sex, Expr::value(req.sex))
        .col_expr(sys_user::Column::UpdateTime, Expr::value(now))
        .filter(sys_user::Column::Id.eq(req.id))
        .exec(&txn)
        .await?;
    // user.update(&txn).await?;
    txn.commit().await?;
    let res = "用户状态更新成功".to_string();

    Ok(res)
}

pub async fn change_role(_db: &DatabaseConnection, _req: ChangeRoleReq) -> Result<String> {
    // let txn = db.begin().await?;
    // // 更新用户信息
    // SysUser::update_many()
    //     .col_expr(sys_user::Column::RoleId, Expr::value(req.role_id))
    //     .filter(sys_user::Column::Id.eq(req.user_id))
    //     .exec(&txn)
    //     .await?;
    // // user.update(&txn).await?;
    // txn.commit().await?;
    let res = "用户角色切换成功".to_string();

    Ok(res)
}

pub async fn change_dept(db: &DatabaseConnection, req: ChangeDeptReq) -> Result<String> {
    let txn = db.begin().await?;
    // 更新用户信息
    SysUser::update_many()
        .col_expr(sys_user::Column::DeptId, Expr::value(req.dept_id))
        .filter(sys_user::Column::Id.eq(req.user_id))
        .exec(&txn)
        .await?;
    // user.update(&txn).await?;
    txn.commit().await?;
    let res = "用户部门切换成功".to_string();

    Ok(res)
}

pub async fn update_avatar(db: &DatabaseConnection, img: &str, user_id: &str) -> Result<String> {
    let txn = db.begin().await?;
    // 更新用户信息
    SysUser::update_many()
        .col_expr(sys_user::Column::Avatar, Expr::value(img))
        .filter(sys_user::Column::Id.eq(user_id))
        .exec(&txn)
        .await?;
    // user.update(&txn).await?;
    txn.commit().await?;
    let res = "用户头像更新成功".to_string();

    Ok(res)
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, req: SysUserDeleteReq) -> Result<String> {
    let mut s = SysUser::delete_many();

    s = s.filter(sys_user::Column::Id.is_in(req.clone().user_ids));

    // 开始删除
    let txn = db.begin().await?;
    // 删除用户
    let d = s.exec(&txn).await?;
    // 删除用户职位数据
    super::sys_post::delete_post_by_user_id(&txn, req.user_ids.clone()).await?;
    // 删除用户角色数据
    super::sys_user_role::delete_user_role_by_user_ids(&txn, req.user_ids.clone(), None).await?;
    // 删除用户部门数据
    super::sys_user_dept::delete_user_dept_by_user_ids(&txn, req.user_ids).await?;

    txn.commit().await?;
    match d.rows_affected {
        0 => Err(anyhow!("用户不存在")),
        i => Ok(format!("成功删除{}条用户数据", i)),
    }
}

// edit 修改
pub async fn edit(
    db: &DatabaseConnection,
    req: SysUserEditReq,
    c_user_id: String,
) -> Result<String> {
    let uid = req.id;
    // 更新
    let txn = db.begin().await?;
    // 更新用户信息
    sys_user::Entity::update_many()
        .col_expr(sys_user::Column::UserName, Expr::value(req.user_name))
        .col_expr(sys_user::Column::NickName, Expr::value(req.nick_name))
        .col_expr(sys_user::Column::Status, Expr::value(req.status))
        .col_expr(sys_user::Column::Email, Expr::value(req.email))
        .col_expr(sys_user::Column::Sex, Expr::value(req.sex))
        .col_expr(sys_user::Column::DeptId, Expr::value(req.dept_id.clone()))
        .col_expr(sys_user::Column::Remark, Expr::value(req.remark))
        // .col_expr(sys_user::Column::IsAdmin, Expr::value(req.is_admin))
        .col_expr(sys_user::Column::PhoneNumber, Expr::value(req.phone_number))
        .col_expr(
            sys_user::Column::UpdateTime,
            Expr::value(Local::now().naive_local()),
        )
        // .col_expr(sys_user::Column::RoleId, Expr::value(req.role_id))
        .filter(sys_user::Column::Id.eq(uid.clone()))
        .exec(&txn)
        .await?;

    //  更新岗位信息
    // 1.先删除用户岗位关系
    super::sys_post::delete_post_by_user_id(&txn, vec![uid.clone()]).await?;
    // 2.插入用户岗位关系
    super::sys_post::add_post_by_user_id(&txn, &uid, req.post_ids).await?;
    // 更新用户角色信息
    // 先删除原有的角色信息，再添加新的角色信息
    super::sys_user_role::delete_user_role(&txn, &uid).await?;

    super::sys_user_role::edit_user_role(&txn, &uid, req.role_ids.unwrap_or(vec![]), &c_user_id)
        .await?;
    // 删除原有部门信息
    super::sys_user_dept::delete_user_dept(&txn, &uid).await?;
    let mut dept_ids = req.dept_ids.unwrap_or(vec![]);
    dept_ids.push(req.dept_id.clone());
    // 添加新的部门信息
    super::sys_user_dept::edit_user_dept(&txn, &uid, dept_ids, &c_user_id).await?;

    txn.commit().await?;
    Ok("用户数据更新成功".to_string())
}

/// 用户登录
pub async fn login(
    db: &DatabaseConnection,
    login_req: UserLoginReq,
    header: HeaderMap,
) -> Result<AuthBody> {
    let mut msg = "登录成功".to_string();
    let mut status = "0".to_string();
    // 验证验证码
    if utils::rand_utils::encrypt_password(&login_req.code.to_lowercase(), "") != login_req.uuid {
        msg = "验证码错误".to_string();
        status = "0".to_string();
        set_login_info(
            header,
            "".to_string(),
            login_req.username.clone(),
            msg.clone(),
            status.clone(),
            None,
            None,
        )
        .await;
        return Err(anyhow!("验证码错误"));
    }
    // 根据用户名获取用户信息
    let user = match SysUser::find()
        .filter(sys_user::Column::UserName.eq(login_req.username.clone()))
        .one(db)
        .await?
    {
        Some(user) => {
            if &user.status.clone().unwrap() != "0" {
                msg = "用户已被禁用".to_string();
                status = "0".to_string();
                set_login_info(
                    header,
                    "".to_string(),
                    login_req.username.clone(),
                    msg.clone(),
                    status.clone(),
                    None,
                    None,
                )
                .await;
                return Err(anyhow!("用户已被禁用"));
            } else {
                user
            }
        }
        None => {
            msg = "用户不存在".to_string();
            status = "0".to_string();
            set_login_info(
                header,
                "".to_string(),
                login_req.username.clone(),
                msg.clone(),
                status.clone(),
                None,
                None,
            )
            .await;
            return Err(anyhow!("用户不存在"));
        }
    };
    //  验证密码是否正确
    if !verify_password(user.password.unwrap_or("".to_string()), login_req.password)? {
        msg = "密码错误".to_string();
        status = "0".to_string();
        set_login_info(
            header,
            "".to_string(),
            login_req.username.clone(),
            msg.clone(),
            status.clone(),
            None,
            None,
        )
        .await;
        return Err(anyhow!("密码不正确"));
    };
    // 注册JWT
    let claims = AuthPayload {
        id: user.id.clone(),              // 用户id
        name: login_req.username.clone(), // 用户名
    };
    let token_id = next_id()?;
    let token = common::jwt::authorize(claims.clone()).await?;
    // 成功登录后
    //  写入登录日志

    set_login_info(
        header,
        user.id.to_string(),
        login_req.username.clone(),
        msg.clone(),
        status.clone(),
        Some(token_id),
        Some(token.clone()),
    )
    .await;

    Ok(token)
}

/// 用户登录
pub async fn fresh_token(user: &User) -> Result<AuthBody> {
    // 注册JWT
    let claims = AuthPayload {
        id: user.clone().id()?,            // 用户id
        name: user.clone().claims()?.name, // 用户名
    };
    let token = common::jwt::authorize(claims.clone()).await?;
    let token_id = next_id()?;
    // 成功登录后
    // 更新原始在线日志
    super::sys_user_online::update_online(token_id, token.clone().exp).await?;

    Ok(token)
}

pub async fn set_login_info(
    header: HeaderMap,
    u_id: String,
    user: String,
    msg: String,
    status: String,
    token_id: Option<String>,
    token: Option<AuthBody>,
) {
    let u = service_utils::get_client_info(header).await;
    // 写入登录日志
    let u2 = u.clone();
    let status2 = status.clone();
    // 如果成功，写入在线日志
    if status == "1" {
        if let (Some(token_id), Some(token)) = (token_id, token) {
            super::sys_user_online::add(u, u_id, token_id, token.clone().exp).await;
        }
    };
    tokio::spawn(async move {
        super::sys_login_log::add(u2, user, msg, status2).await;
    });
}

/// 按id 获取用户信息
pub async fn get_user_info_by_id(db: &DatabaseConnection, id: &str) -> Result<UserInformation> {
    match get_by_id(db, id).await {
        Err(e) => Err(e),
        Ok(user) => {
            let (post_ids_r, role_ids_r, dept_ids_r) = join!(
                super::sys_post::get_post_ids_by_user_id(db, &user.user.id),
                super::sys_user_role::get_role_ids_by_user_id(db, &user.user.id),
                super::sys_user_dept::get_dept_ids_by_user_id(db, &user.user.id),
            );
            let post_ids = match post_ids_r {
                Ok(x) => x,
                Err(e) => return Err(anyhow!(e.to_string())),
            };
            let role_ids = match role_ids_r {
                Ok(x) => x,
                Err(e) => return Err(anyhow!(e.to_string())),
            };
            let dept_ids = match dept_ids_r {
                Ok(x) => x,
                Err(e) => return Err(anyhow!(e.to_string())),
            };
            let res = UserInformation {
                user_info: user.clone(),
                dept_id: user.user.dept_id,
                post_ids,
                role_ids,
                dept_ids,
                post_roles: get_post_role_list(db).await?,
            };
            Ok(res)
        }
    }
}

/// 获取用户信息以及权限
pub async fn get_user_info_permission(
    db: &DatabaseConnection,
    user_id: &str,
) -> Result<(UserWithDept, Vec<String>)> {
    //  获取用户信息
    let user_info = get_by_id(db, user_id).await?;

    // 检查是否超管用户
    let permissions = if CFG.system.super_user.contains(&user_id.to_string()) {
        vec!["*:*:*".to_string()]
    } else {
        let (apis, _) = super::sys_menu::get_role_permissions(db, &user_info.user.role_id).await?;
        apis
    };
    Ok((user_info, permissions))
}

#[cfg(test)]
mod tests {
    use silent::prelude::argon2;

    #[test]
    fn test_generate_password() {
        let password = "e10adc3949ba59abbe56e057f20f883e".to_string();
        let hash = argon2::make_password(password).unwrap();
        println!("{:?}", hash);
    }
}