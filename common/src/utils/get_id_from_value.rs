use crate::{BpmError, Result};
use serde_json::Value;

pub trait GetIdFromValue {
    fn get_id(&self) -> Result<String>;
}

impl GetIdFromValue for Value {
    fn get_id(&self) -> Result<String> {
        match self.get("id") {
            None => Err(BpmError::serde_with_msg("id不能为空")),
            Some(Value::String(id)) => Ok(id.clone()),
            _ => Err(BpmError::serde_with_msg("id必须为字符串")),
        }
    }
}
