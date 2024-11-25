use super::sys_dept::DeptResp;
use crate::system::sys_post::SysPostResp;
use crate::system::sys_role::SysRoleResp;
use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SysUserAddReq {
    pub user_name: String,
    pub nick_name: String,
    pub password: String,
    pub status: String,
    pub email: Option<String>,
    pub sex: String,
    pub avatar: Option<String>,
    pub remark: Option<String>,
    pub phone_number: Option<String>,
    pub post_ids: Vec<String>,
    pub dept_ids: Option<Vec<String>>,
    pub dept_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SysUserEditReq {
    pub id: String,
    pub user_name: String,
    pub nick_name: String,
    pub status: String,
    pub email: Option<String>,
    pub sex: String,
    pub avatar: String,
    pub remark: Option<String>,
    pub phone_number: Option<String>,
    pub post_ids: Vec<String>,
    pub dept_ids: Option<Vec<String>>,
    pub dept_id: String,
    pub role_ids: Option<Vec<String>>,
    pub role_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileReq {
    pub id: String,
    pub user_nickname: String,
    pub phone_num: String,
    pub user_email: String,
    pub sex: String,
}

#[derive(Debug, Clone, Default, Serialize, FromQueryResult, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResp {
    pub id: String,
    pub user_name: String,
    pub nick_name: String,
    pub status: String,
    pub email: Option<String>,
    pub sex: String,
    pub avatar: String,
    pub dept_id: String,
    pub remark: Option<String>,
    pub admin: bool,
    pub phone_number: Option<String>,
    pub role_id: String,
    pub create_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRoleListResp {
    pub posts: Vec<SysPostResp>,
    pub roles: Vec<SysRoleResp>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserWithDept {
    #[serde(flatten)]
    pub user: UserResp,
    pub dept: DeptResp,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInformation {
    pub user_info: UserWithDept,
    pub post_ids: Vec<String>,
    pub role_ids: Vec<String>,
    pub dept_ids: Vec<String>,
    pub dept_id: String,
    #[serde(flatten)]
    pub post_roles: PostRoleListResp,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SysUserSearchReq {
    pub user_id: Option<String>,
    pub role_id: Option<String>,
    pub user_ids: Option<String>,
    pub user_name: Option<String>,
    pub phone_num: Option<String>,
    pub user_nickname: Option<String>,
    pub user_status: Option<String>,
    pub dept_id: Option<String>,
    pub dept_ids: Option<Vec<String>>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SysUserDeleteReq {
    pub user_ids: Vec<String>,
}

///  用户登录
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserLoginReq {
    ///  用户名
    pub username: String,
    ///  用户密码
    pub password: String,
    pub code: String,
    pub uuid: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub user: UserWithDept,
    pub roles: Vec<String>,
    pub depts: Vec<String>,
    pub permissions: Vec<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPwdReq {
    pub user_id: String,
    pub new_passwd: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePwdReq {
    pub old_passwd: String,
    pub new_passwd: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeStatusReq {
    pub user_id: String,
    pub status: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRoleReq {
    pub user_id: String,
    pub role_id: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeDeptReq {
    pub user_id: String,
    pub dept_id: String,
}