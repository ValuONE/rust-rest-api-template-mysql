use std::error::Error;

use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
}

pub async fn initialize_pool(db_url: String, connections: u32) -> Result<AppState, Box<dyn Error>> {
    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(connections)
        .connect(&db_url)
        .await?;

    Ok(AppState { pool })
}