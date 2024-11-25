use sea_orm::{entity::prelude::DateTime, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SysPostSearchReq {
    pub id: Option<String>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SysPostAddReq {
    pub code: String,
    pub name: String,
    pub sort: i32,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysPostDeleteReq {
    pub post_ids: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SysPostEditReq {
    pub id: String,
    pub code: String,
    pub name: String,
    pub sort: i32,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct SysPostResp {
    pub id: String,
    pub code: String,
    pub name: String,
    pub sort: i32,
    pub status: String,
    pub remark: Option<String>,
    pub create_time: DateTime,
}
