use async_trait::async_trait;
use tower_sessions::{
    session::{Id, Record},
    session_store, ExpiredDeletion, SessionStore,
};
use crate::PgPool;

use super::session_repository;

/// A PostgreSQL session store.
#[derive(Clone, Debug)]
pub struct PostgresStore {
    pool: PgPool,
    schema_name: String,
    table_name: String,
}

impl PostgresStore {
    /// Create a new PostgreSQL store with the provided connection pool.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tower_sessions::{sqlx::PgPool, PostgresStore};
    ///
    /// # tokio_test::block_on(async {
    /// let database_url = std::option_env!("DATABASE_URL").unwrap();
    /// let pool = PgPool::connect(database_url).await.unwrap();
    /// let session_store = PostgresStore::new(pool);
    /// # })
    /// ```
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            schema_name: "sessions".to_string(),
            table_name: "session".to_string(),
        }
    }
}

#[async_trait]
impl ExpiredDeletion for PostgresStore {
    async fn delete_expired(&self) -> session_store::Result<()> {
        let mut connection = self.pool.get().unwrap();

        let results = tokio::task::spawn_blocking(move || {
            session_repository::delete_expired(&mut connection)
        })
        .await
        .unwrap();

        Ok(())
    }
}

#[async_trait]
impl SessionStore for PostgresStore {
    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let mut connection = self.pool.get().unwrap();

        let record_close = record.clone();
        let results =
            tokio::task::spawn_blocking(move || session_repository::save(&mut connection, &record_close))
                .await
                .unwrap();

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let mut connection = self.pool.get().unwrap();

        let session_id = session_id.clone();
        let session = tokio::task::spawn_blocking(move || {
            session_repository::get_by_id(&mut connection, session_id.to_string())
        })
        .await
        .unwrap()
        .unwrap();

        Ok(Some(rmp_serde::from_slice(&session.data).unwrap()))
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let mut connection = self.pool.get().unwrap();

        let session_id = session_id.clone();
        let result = tokio::task::spawn_blocking(move || {
            session_repository::delete_by_id(&mut connection, session_id.to_string())
        }).await;

        Ok(())
    }
}

/// A valid PostreSQL identifier must start with a letter or underscore
/// (including letters with diacritical marks and non-Latin letters). Subsequent
/// characters in an identifier or key word can be letters, underscores, digits
/// (0-9), or dollar signs ($). See https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-IDENTIFIERS for details.
fn is_valid_identifier(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .next()
            .map(|c| c.is_alphabetic() || c == '_')
            .unwrap_or_default()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '$')
}
