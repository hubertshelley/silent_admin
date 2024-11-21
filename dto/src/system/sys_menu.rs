use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use entity::sys_menu;

#[derive(Deserialize, Clone)]
pub struct SysMenuSearchReq {
    pub id: Option<String>,
    pub pid: Option<String>,
    pub menu_name: Option<String>,
    pub menu_type: Option<String>,
    pub menu_types: Option<String>,
    pub method: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SysMenuTreeAll {
    #[serde(flatten)]
    pub menu: sys_menu::Model,
    pub children: Option<Vec<SysMenuTreeAll>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, FromQueryResult, Default)]
#[serde(rename_all = "camelCase")]
pub struct MenuResp {
    pub id: String,
    pub name: String,
    pub parent_id: String,
    pub order_num: i32,
    pub path: String,
    pub component: Option<String>,
    pub query: Option<String>,
    pub is_frame: i32,
    pub is_cache: i32,
    pub menu_type: String,
    pub visible: String,
    pub status: String,
    pub icon: String,
    pub remark: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MenuRelated {
    #[serde(flatten)]
    pub menu: sys_menu::Model,
    pub dbs: Vec<String>,
    pub apis: Vec<String>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserMenu {
    pub id: String,
    pub parent_id: String,
    #[serde(skip_serializing)]
    pub menu_name: String,
    #[serde(skip_serializing)]
    pub menu_type: String,
    #[serde(skip_serializing)]
    pub is_frame: i32,
    pub name: String,
    pub path: String,
    pub hidden: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<String>,
    pub component: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_show: Option<bool>,
    pub meta: Meta,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub title: String,
    pub icon: String,
    pub no_cache: bool,
    pub link: Option<String>,
    #[serde(skip_serializing)]
    pub hidden: bool,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct SysMenuTree {
    #[serde(flatten)]
    pub user_menu: UserMenu,
    pub children: Option<Vec<SysMenuTree>>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SysMenuAddReq {
    pub parent_id: String,
    pub path: Option<String>,
    pub name: String,
    pub icon: Option<String>,
    pub menu_type: String,
    pub query: Option<String>,
    pub order_num: i32,
    pub status: String,
    pub method: Option<String>,
    pub component: Option<String>,
    pub visible: String,
    pub is_frame: i32,
    pub is_cache: i32,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysMenuDeleteReq {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysMenuEditReq {
    pub id: String,
    pub parent_id: String,
    pub path: String,
    pub name: String,
    pub icon: Option<String>,
    pub menu_type: String,
    pub query: Option<String>,
    pub order_num: i32,
    pub status: String,
    pub method: Option<String>,
    pub component: Option<String>,
    pub visible: String,
    pub is_frame: i32,
    pub is_cache: i32,
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCacheEditReq {
    pub id: String,
    pub log_method: String,
    pub data_cache_method: String,
}
