use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SysOperLogSearchReq {
    pub oper_id: Option<String>,
    pub title: Option<String>,
    pub oper_name: Option<String>,
    pub operator_type: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysOperLogDeleteReq {
    pub oper_log_ids: Vec<String>,
}
