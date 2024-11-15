use common::get_db_conn;
use common::log::init_log;
use common::service::{init_route, init_server};
use migration::MigratorTrait;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let mut _guard = Vec::new();
    init_log(&mut _guard);
    let db = get_db_conn().await;
    let pending_migrations_count = migration::Migrator::get_pending_migrations(&db)
        .await
        .expect("获取待迁移数据失败")
        .len();
    if pending_migrations_count > 0 {
        if let Err(e) = migration::Migrator::up(&db, Some(pending_migrations_count as u32)).await {
            eprintln!("Failed to migrate: {}", e);
            std::process::exit(1);
        }
    }

    let route = init_route(api::get_routes());
    init_server(db).serve(route).await
}
