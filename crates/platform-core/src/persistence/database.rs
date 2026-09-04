use crate::error::{PlatformError, Result};
use crate::persistence::schema::run_migrations;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Clone)]
pub struct PlatformDatabase {
    pool: Pool<Sqlite>,
}

impl PlatformDatabase {
    pub async fn in_memory() -> Result<Self> {
        let options = SqliteConnectOptions::from_str("sqlite::memory:")?
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;

        run_migrations(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn open(db_path: &Path) -> Result<Self> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let db_str = db_path.to_str().ok_or_else(|| {
            PlatformError::Internal("Invalid database path encoding".to_string())
        })?;

        let options = SqliteConnectOptions::from_str(&format!("sqlite://{db_str}"))?
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        run_migrations(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn default_local() -> Result<Self> {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("goose")
            .join("platform.db");
        Self::open(&data_dir).await
    }

    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
}
