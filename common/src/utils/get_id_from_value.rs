use crate::{Result, SilentAdminError};
use serde_json::Value;

pub trait GetIdFromValue {
    fn get_id(&self) -> Result<String>;
}

impl GetIdFromValue for Value {
    fn get_id(&self) -> Result<String> {
        match self.get("id") {
            None => Err(SilentAdminError::serde_with_msg("id不能为空")),
            Some(Value::String(id)) => Ok(id.clone()),
            _ => Err(SilentAdminError::serde_with_msg("id必须为字符串")),
        }
    }
}
