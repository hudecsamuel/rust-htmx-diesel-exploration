use async_trait::async_trait;
use axum_login::AuthnBackend;
use diesel::{
    result::Error, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};
// use diesel::sql_types::Uuid;
use crate::db::schema::users;
use crate::models::user::UserModel;
use crate::PgPool;
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// use crate::infra::errors::{adapt_infra_error, InfraError};
use diesel::pg::PgConnection;

use super::user_repository::{self, Credentials, UserDb};

#[derive(Debug, Clone)]
pub struct Backend {
    pool: PgPool,
}

impl Backend {
    /// Create a new Backend for axum login auth with the provided connection pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = UserDb;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let mut connection = self.pool.get().unwrap();

        let user = tokio::task::spawn_blocking(move || {
            user_repository::get_by_email(&mut connection, creds.email.to_string())
        })
        .await
        .unwrap()
        .unwrap();

        let verify_result = verify_password(creds.password, &user.password)
            .ok()
            .is_some();

        match verify_result {
            true => Ok(Some(user)),
            false => Ok(None),
        }
    }

    async fn get_user(&self, user_id: &Uuid) -> Result<Option<UserDb>, Self::Error> {
        let mut connection = self.pool.get().unwrap();

        let user_id = user_id.clone();
        let user = tokio::task::spawn_blocking(move || {
            user_repository::get_by_id(&mut connection, user_id)
        })
        .await
        .unwrap();

        match user {
            Ok(user) => Ok(Some(user)),
            Err(_) => Ok(None),
        }
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<Backend>;
