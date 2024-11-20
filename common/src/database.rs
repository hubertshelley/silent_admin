use configs::CFG;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::OnceCell;

//  异步初始化数据库
pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_db_conn() -> DatabaseConnection {
    let connect_options = ConnectOptions::new(CFG.database.link.to_owned());

    Database::connect(connect_options)
        .await
        .expect("Failed to connect to database")
}
