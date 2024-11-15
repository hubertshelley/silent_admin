mod database;
mod error;
mod global_error_handler;
pub mod jwt;
pub mod log;
pub mod middlewares;
pub mod pagination;
pub mod query;
mod response;
pub mod service;
pub mod snowflake;
pub mod utils;

pub use database::get_db_conn;
pub use error::BpmError;
pub type Result<T> = std::result::Result<T, BpmError>;
pub use sea_orm::DbConn;
