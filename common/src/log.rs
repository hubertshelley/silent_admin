use configs::CFG;
use silent::prelude::logger;
use tracing::metadata::LevelFilter;
use tracing::{info, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::{Compact, Format};
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

pub fn init_log(guards: &mut Vec<WorkerGuard>) {
    // 日志级别
    let log_level = CFG.log.log_level.clone();

    // 系统变量设置
    let log_level = get_log_level(log_level);

    //  日志设置
    let format = get_log_format();

    // 文件输出
    let file_appender = tracing_appender::rolling::daily(CFG.log.dir.clone(), CFG.log.file.clone());
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    guards.push(guard);

    // 标准控制台输出
    let (std_non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
    guards.push(guard);
    logger::registry()
        // .with(logger::fmt::layer().with_filter(LevelFilter::from_level(log_level)))
        .with(
            logger::fmt::layer()
                .with_writer(std_non_blocking)
                .event_format(format.clone())
                .pretty()
                .with_filter(LevelFilter::from_level(log_level)),
        )
        .with(
            logger::fmt::layer()
                .with_writer(non_blocking)
                .event_format(format)
                .pretty()
                .with_filter(LevelFilter::from_level(log_level)),
        )
        .init();
    info!("log init success");
}

pub fn get_log_level(log_level: String) -> Level {
    match log_level.as_str() {
        "TRACE" => Level::TRACE,
        "DEBUG" => Level::DEBUG,
        "INFO" => Level::INFO,
        "WARN" => Level::WARN,
        "ERROR" => Level::ERROR,
        _ => Level::INFO,
    }
}

pub fn get_log_format() -> Format<Compact, ChronoLocal> {
    logger::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_timer(ChronoLocal::rfc_3339())
        .compact()
}
