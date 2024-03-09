use sqlx::PgPool;

use super::config::database::get_postgres_pool;

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
}

pub async fn get_app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    let pg_pool = get_postgres_pool().await?;

    Ok(AppState { pg_pool })
}
