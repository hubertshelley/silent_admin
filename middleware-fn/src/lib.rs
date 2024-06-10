// 菜单授权
pub mod auth;
// 请求上下文，日志记录
pub mod ctx;

// 操作日志
pub mod oper_log;
// 缓存中间件
#[cfg(feature = "cache-mem")]
pub mod cache;

#[cfg(feature = "cache-skytable")]
pub mod cache_skytable;

//  重新导出
pub use auth::AuthMiddleware as ApiAuth;
#[cfg(feature = "cache-mem")]
pub use cache::CacheMiddleware as Cache;
#[cfg(feature = "cache-skytable")]
pub use cache_skytable::SkyTableCacheMiddleware as SkyTableCache;
pub use ctx::CtxMiddleware as Ctx;
pub use oper_log::OperLogMiddleware as OperLog;
