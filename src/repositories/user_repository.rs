use axum_login::AuthUser;
use diesel::{
    ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper,
};
use password_auth::generate_hash;
// use diesel::sql_types::Uuid;
use crate::db::schema::users;
use crate::models::user::UserModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// use crate::infra::errors::{adapt_infra_error, InfraError};
use diesel::pg::PgConnection;

#[derive(Serialize, Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDb {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl UserDb {
    pub fn to_model(&self) -> UserModel {
        UserModel {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
        }
    }
}

impl AuthUser for UserDb {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes() // We use the password hash as the auth
                                 // hash--what this means
                                 // is when the user changes their password the
                                 // auth session becomes invalid.
    }
}


#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUserDb {
    pub name: String,
    pub email: String,
    pub password: String,
}

// This allows us to extract the authentication fields from forms. We use this
// to authenticate requests with the backend.
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
    pub next: Option<String>,
}

pub fn get_users(conn: &mut PgConnection) -> Result<Vec<UserDb>, diesel::result::Error> {
    let result = users::table.select(UserDb::as_select()).get_results(conn)?;

    let users: Vec<UserDb> = result
        .into_iter()
        .map(|user_db| UserDb {
            id: user_db.id,
            name: user_db.name,
            email: user_db.email,
            password: user_db.password,
        })
        .collect();

    Ok(users)
}

pub fn create_user(
    connection: &mut PgConnection,
    user: NewUserDb,
) -> Result<UserDb, diesel::result::Error> {
    let result = diesel::insert_into(users::table)
        .values(NewUserDb {
            name: user.name,
            email: user.email,
            password: generate_hash(user.password),
        })
        .returning(UserDb::as_returning())
        .get_result(connection)?;

    Ok(UserDb {
        id: result.id,
        name: result.name,
        email: result.email,
        password: result.password,
    })
}

pub fn get_by_id(
    connection: &mut PgConnection,
    user_id: Uuid,
) -> Result<UserDb, diesel::result::Error> {
    let result = users::table
        .select(UserDb::as_select())
        .filter(users::id.eq(user_id))
        .first::<UserDb>(connection)?;

    Ok(UserDb {
        id: result.id,
        name: result.name,
        email: result.email,
        password: result.password,
    })
}

pub fn get_by_email(
    connection: &mut PgConnection,
    email: String,
) -> Result<UserDb, diesel::result::Error> {
    let result = users::table
        .select(UserDb::as_select())
        .filter(users::email.eq(email))
        .first::<UserDb>(connection)?;

    Ok(UserDb {
        id: result.id,
        name: result.name,
        email: result.email,
        password: result.password,
    })
}
