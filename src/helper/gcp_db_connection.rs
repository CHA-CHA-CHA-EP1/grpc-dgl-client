use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

pub async fn gcp_create_db_pool(
    host: &str,
    user: &str,
    password: &str,
    database: &str,
) -> Result<Pool<MySql>, sqlx::Error> {
    let database_url = format!("mysql://{}:{}@{}/{}", user, password, host, database);

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
