use silent::middleware::middlewares::Cors;
use silent::middlewares::CorsType;
use silent::prelude::{Route, RouteService};
use silent::prelude::Server;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

use app_service::{service_utils, tasks};
use configs::CFG;
use utils::my_env::{self, RT};

// #[tokio::main]
fn main() {
    RT.block_on(async {
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", &CFG.log.log_level);
        }
        my_env::setup();

        // 系统变量设置
        let log_env = my_env::get_log_level();

        //  日志设置
        let format = my_env::get_log_format();

        // 文件输出
        let file_appender = tracing_appender::rolling::hourly(&CFG.log.dir, &CFG.log.file);
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        // 标准控制台输出
        let (std_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
        let logger = Registry::default()
            .with(EnvFilter::from_default_env().add_directive(log_env.into()))
            .with(fmt::Layer::default().with_writer(std_non_blocking).event_format(format.clone()).pretty())
            .with(fmt::Layer::default().with_writer(non_blocking).event_format(format))
            // .with(console_layer)
            ;
        tracing::subscriber::set_global_default(logger).unwrap();

        // apis全局初始化
        service_utils::ApiUtils::init_all_api().await;
        // 定时任务初始化
        tasks::timer_task_init().await.expect("定时任务初始化失败");

        //  跨域
        let cors = Cors::new().origin(CorsType::Any).methods(CorsType::Any).headers(CorsType::Any).credentials(true);
        // 顺序不对可能会导致数据丢失，无法在某些位置获取数据
        // let static_files_service = get_service(
        //     ServeDir::new(&CFG.web.dir)
        //         .not_found_service(handle_404.into_service())
        //         .append_index_html_on_directories(true),
        // );

        let route = Route::new("")
            .hook(cors)
            .append(Route::new(&CFG.server.api_prefix).append(api::api()))
            .append(Route::new("").with_static(&CFG.web.dir))
            .route()
            .set_exception_handler(api::exception_handler);
        Server::new().bind(CFG.server.address.parse().expect("Invalid server address")).serve(route).await;
    })
}
// async fn handle_404() -> (StatusCode, &'static str) {
//     (StatusCode::NOT_FOUND, "Not found")
// }
