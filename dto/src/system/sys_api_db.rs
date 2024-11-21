use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SysApiDbAddEditReq {
    pub api_id: String,
    pub dbs: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysApiDbSearchReq {
    pub api_id: String,
}
