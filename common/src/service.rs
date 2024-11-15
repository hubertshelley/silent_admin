use crate::global_error_handler::exception_handler;
use crate::middlewares::authorization::JWTAuthorizationMiddleware;
use crate::middlewares::permission::PermissionMiddleware;
use crate::middlewares::result_wrapper::ResultWrapper;
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
        .bind("0.0.0.0:40000".parse().unwrap())
        .with_configs(config)
}

pub fn init_route(route: Route) -> RootRoute {
    let route = ResultWrapper::register(route);
    let authorization = JWTAuthorizationMiddleware::default();
    let permission = PermissionMiddleware::new(vec![".*/getToken.*".to_string()]);
    let cors = Cors::new().origin("*").headers("*");
    let exception_handler = ExceptionHandler::new(exception_handler);
    info!("Routes initialized");
    route
        .root_hook(exception_handler)
        .root_hook(cors)
        .hook(authorization)
        .hook(permission)
        .route()
}
