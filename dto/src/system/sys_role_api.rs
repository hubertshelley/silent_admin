use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SysRoleApiAddReq {
    pub role_id: String,
    pub api: String,
    pub method: Option<String>,
}
