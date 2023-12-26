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

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDb {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUserDb {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub fn get_users(conn: &mut PgConnection) -> Result<Vec<UserModel>, diesel::result::Error> {
    let result = users::table.select(UserDb::as_select()).get_results(conn)?;

    let users: Vec<UserModel> = result
        .into_iter()
        .map(|user_db| UserModel {
            id: user_db.id,
            name: user_db.name,
            email: user_db.email,
        })
        .collect();

    Ok(users)
}

pub fn create_user(
    connection: &mut PgConnection,
    user: NewUserDb,
) -> Result<UserModel, diesel::result::Error> {
    let result = diesel::insert_into(users::table)
        .values(user)
        .returning(UserDb::as_returning())
        .get_result(connection)?;

    Ok(UserModel {
        id: result.id,
        name: result.name,
        email: result.email,
    })
}

pub fn get_user_by_id(
    connection: &mut PgConnection,
    user_id: Uuid,
) -> Result<UserModel, diesel::result::Error> {
    let result = users::table
        .select(UserDb::as_select())
        .filter(users::id.eq(user_id))
        .first::<UserDb>(connection)?;

    Ok(UserModel {
        id: result.id,
        name: result.name,
        email: result.email,
    })
}

