use anyhow::anyhow;
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
};
use std::{env, str::FromStr, time::Duration};

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let database_url =
            env::var("DATABASE_URL").map_err(|_| anyhow!("DATABASE_URL must be set"))?;

        let options = SqliteConnectOptions::from_str(&database_url)?
            .create_if_missing(true)
            .foreign_keys(true)
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(Duration::from_secs(10));

        // Create pool
        let pool = SqlitePool::connect_with(options).await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }
}
