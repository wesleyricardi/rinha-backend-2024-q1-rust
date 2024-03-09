use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};

pub async fn get_postgres_pool() -> Result<Pool<Postgres>, Error> {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or("DATABASE_URL=postgres://postgres:123456789@database:5432/rinha".to_owned());

    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(30))
        .max_connections(5)
        .connect(&db_url)
        .await
}
