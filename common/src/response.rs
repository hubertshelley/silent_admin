use std::fmt::Debug;

use chrono::Local;
use serde::Serialize;

/// 数据统一返回格式
#[derive(Debug, Serialize, Default)]
pub struct Res<T> {
    pub code: Option<u16>,
    pub data: Option<T>,
    pub msg: Option<String>,
    pub time: i64,
}

impl<T: Serialize> Res<T> {
    pub fn with_data(data: T) -> Self {
        Self {
            code: Some(0),
            data: Some(data),
            msg: Some("请求成功".to_string()),
            time: Local::now().timestamp_millis(),
        }
    }
    pub fn with_err(code: u16, err: &str) -> Self {
        Self {
            code: Some(code),
            data: None,
            msg: Some(err.to_string()),
            time: Local::now().timestamp_millis(),
        }
    }
}
