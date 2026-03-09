use sqlx::migrate::{MigrateDatabase, MigrateError};
use sqlx::{Error as SqlxError, Pool, Sqlite, SqlitePool};

#[derive(Debug, thiserror::Error)]
pub enum DBPoolError {
    #[error("couldn't check if db exists: {0}")]
    CouldntCheckIfDbExists(#[source] SqlxError),
    #[error("couldn't create database: {0}")]
    CouldntCreateDatabase(#[source] SqlxError),
    #[error("couldn't connect to database: {0}")]
    CouldntConnectToDB(#[source] SqlxError),
    #[error("couldn't migrate database: {0}")]
    CouldntMigrateDB(#[source] MigrateError),
}

pub async fn get_db_pool(url: &str) -> Result<Pool<Sqlite>, DBPoolError> {
    let db_exists = Sqlite::database_exists(url)
        .await
        .map_err(DBPoolError::CouldntCheckIfDbExists)?;

    if !db_exists {
        Sqlite::create_database(url)
            .await
            .map_err(DBPoolError::CouldntCreateDatabase)?;
    }

    let pool = SqlitePool::connect(url)
        .await
        .map_err(DBPoolError::CouldntConnectToDB)?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(DBPoolError::CouldntMigrateDB)?;

    Ok(pool)
}
