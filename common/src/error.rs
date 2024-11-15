use sea_orm::DbErr;
use silent::{SilentError, StatusCode};
use thiserror::Error;

/// SilentError is the error type for the `silent` library.
#[derive(Error, Debug)]
pub enum BpmError {
    SonyFlakeError(#[from] sonyflake::Error),
    DbErr(#[from] DbErr),
    SerdeJson(#[from] serde_json::Error),
    DatabaseErr(String),
    BusinessError {
        /// 错误码
        code: StatusCode,
        /// 错误信息
        msg: String,
    },
    Unauthorized,
}

impl BpmError {
    pub fn code_msg<T: Into<String>>(code: StatusCode, msg: T) -> Self {
        BpmError::BusinessError {
            code,
            msg: msg.into(),
        }
    }
    pub fn msg<T: Into<String>>(msg: T) -> Self {
        BpmError::BusinessError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            msg: msg.into(),
        }
    }
    pub fn num_msg<T: Into<String>>(num: u16, msg: T) -> Self {
        BpmError::BusinessError {
            code: StatusCode::from_u16(num).unwrap(),
            msg: msg.into(),
        }
    }
    pub fn data_not_found() -> Self {
        Self::code_msg(StatusCode::NOT_FOUND, "数据不存在")
    }
    pub fn not_found_with_msg<T: Into<String>>(msg: T) -> Self {
        Self::code_msg(StatusCode::NOT_FOUND, msg)
    }
    pub fn serde_with_msg<T: Into<String>>(msg: T) -> Self {
        Self::code_msg(StatusCode::UNPROCESSABLE_ENTITY, msg)
    }
    pub fn internal_server_error() -> Self {
        Self::code_msg(StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误")
    }
    /// 数据或其他重复内容冲突错误
    pub fn duplicate<T: Into<String>>(msg: T) -> Self {
        Self::num_msg(517, msg)
    }
}

impl std::fmt::Display for BpmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BpmError::SonyFlakeError(e) => write!(f, "SonyFlakeError: {}", e),
            BpmError::DbErr(e) => write!(f, "DbErr: {}", e),
            BpmError::BusinessError { code, msg } => {
                write!(f, "BusinessError: code: {}, msg: {}", code, msg)
            }
            BpmError::DatabaseErr(e) => write!(f, "DatabaseErr: {}", e),
            BpmError::SerdeJson(e) => write!(f, "SerdeJson: {}", e),
            BpmError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl From<BpmError> for SilentError {
    fn from(value: BpmError) -> Self {
        match value {
            BpmError::SonyFlakeError(e) => {
                SilentError::business_error(StatusCode::from_u16(551).unwrap(), e.to_string())
            }
            BpmError::DbErr(e) => {
                SilentError::business_error(StatusCode::from_u16(516).unwrap(), e.to_string())
            }
            BpmError::DatabaseErr(e) => {
                SilentError::business_error(StatusCode::from_u16(516).unwrap(), e)
            }
            BpmError::BusinessError { code, msg } => SilentError::business_error(code, msg),
            BpmError::SerdeJson(e) => e.into(),
            BpmError::Unauthorized => {
                SilentError::business_error(StatusCode::UNAUTHORIZED, "无效的认证信息".to_string())
            }
        }
    }
}
