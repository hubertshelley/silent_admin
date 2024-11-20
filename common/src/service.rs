use crate::global_error_handler::exception_handler;
use crate::middlewares::authorization::JWTAuthorizationMiddleware;
use crate::middlewares::permission::PermissionMiddleware;
use crate::middlewares::result_wrapper::ResultWrapper;
use configs::CFG;
use sea_orm::DatabaseConnection;
use silent::middlewares::{Cors, ExceptionHandler};
use silent::prelude::{RootRoute, Route, RouteService, Server};
use silent::Configs;
use tracing::info;

pub fn init_server(db: DatabaseConnection) -> Server {
    let mut config = Configs::new();
    config.insert(db);
    info!("Service initialized");
    Server::new()
        .bind(CFG.server.address.parse().unwrap())
        .with_configs(config)
}

pub fn init_route(route: Route) -> RootRoute {
    // let api_auth = middleware_fn::ApiAuth {};
    // let ctx = middleware_fn::Ctx {};
    // let router = router.hook(ctx).hook(api_auth);
    // let router = match &CFG.log.enable_oper_log {
    //     true => {
    //         let oper_log = middleware_fn::OperLog {};
    //         router.hook(oper_log)
    //     }
    //     false => router,
    // };
    // match CFG.server.cache_time {
    //     0 => router,
    //     _ => {
    //         if CFG.server.cache_method == 0 {
    //             let cache = middleware_fn::Cache {};
    //             router.hook(cache)
    //         } else {
    //             let cache = middleware_fn::SkyTableCache {};
    //             router.hook(cache)
    //         }
    //     }
    // }
    let mut route = ResultWrapper::register(route);
    let authorization = JWTAuthorizationMiddleware::default();
    let permission =
        PermissionMiddleware::new(vec![".*/getToken.*".to_string(), ".*/comm.*".to_string()]);
    let cors = Cors::new().origin("*").headers("*");
    let exception_handler = ExceptionHandler::new(exception_handler);
    info!("Routes initialized");
    route.path = CFG.server.api_prefix.clone();
    route
        .root_hook(exception_handler)
        .root_hook(cors)
        .hook(authorization)
        .hook(permission)
        .route()
}
