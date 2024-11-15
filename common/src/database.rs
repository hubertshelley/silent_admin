use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn get_db_conn() -> DatabaseConnection {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let schema = std::env::var("DATABASE_SCHEMA").unwrap_or("public".to_string());
    let connect_options = ConnectOptions::new(db_url)
        .set_schema_search_path(schema)
        .to_owned();

    Database::connect(connect_options)
        .await
        .expect("Failed to connect to database")
}
