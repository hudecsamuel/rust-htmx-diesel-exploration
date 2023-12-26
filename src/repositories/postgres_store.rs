use async_trait::async_trait;
use time::OffsetDateTime;
use tower_sessions::{
    session::{Id, Record},
    session_store, ExpiredDeletion, SessionStore,
};

use diesel::{
    Insertable, QueryDsl, Queryable, RunQueryDsl,
    Selectable, SelectableHelper, ExpressionMethods
};
// use diesel::sql_types::Uuid;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::users;
use crate::models::user::UserModel;
// use crate::infra::errors::{adapt_infra_error, InfraError};
use diesel::pg::PgConnection;

/// A PostgreSQL session store.
#[derive(Clone, Debug)]
pub struct PostgresStore {
    pool: PgConnection,
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
    pub fn new(pool: PgConnection) -> Self {
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
        let connection = self.pool.get().unwrap();
        sessions::table
            .filter(sessions::expiry_date.lt(OffsetDateTime::now_utc()))
            .delete(self.pool);
        Ok(())
    }
}

#[async_trait]
impl SessionStore for PostgresStore {
    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let query = format!(
            r#"
            insert into "{schema_name}"."{table_name}" (id, data, expiry_date)
            values ($1, $2, $3)
            on conflict (id) do update
            set
              data = excluded.data,
              expiry_date = excluded.expiry_date
            "#,
            schema_name = self.schema_name,
            table_name = self.table_name
        );
        sqlx::query(&query)
            .bind(&record.id.to_string())
            .bind(rmp_serde::to_vec(&record).map_err(SqlxStoreError::Encode)?)
            .bind(record.expiry_date)
            .execute(&self.pool)
            .await
            .map_err(SqlxStoreError::Sqlx)?;

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let query = format!(
            r#"
            select data from "{schema_name}"."{table_name}"
            where id = $1 and expiry_date > $2
            "#,
            schema_name = self.schema_name,
            table_name = self.table_name
        );
        let record_value: Option<(Vec<u8>,)> = sqlx::query_as(&query)
            .bind(session_id.to_string())
            .bind(OffsetDateTime::now_utc())
            .fetch_optional(&self.pool)
            .await
            .map_err(SqlxStoreError::Sqlx)?;

        if let Some((data,)) = record_value {
            Ok(Some(
                rmp_serde::from_slice(&data).map_err(SqlxStoreError::Decode)?,
            ))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let query = format!(
            r#"delete from "{schema_name}"."{table_name}" where id = $1"#,
            schema_name = self.schema_name,
            table_name = self.table_name
        );
        sqlx::query(&query)
            .bind(&session_id.to_string())
            .execute(&self.pool)
            .await
            .map_err(SqlxStoreError::Sqlx)?;

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
