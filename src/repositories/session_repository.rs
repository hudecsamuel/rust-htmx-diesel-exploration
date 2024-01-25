use crate::db::schema::sessions;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::upsert::excluded;
use diesel::{
    ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tower_sessions::session::Record;
use uuid::Uuid;

#[derive(Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SessionDb {
    pub id: String,
    pub data: Vec<u8>,
    pub expiry_date: DateTime<Utc>,
}

pub fn save(connection: &mut PgConnection, record: &Record) -> Result<(), diesel::result::Error> {
    let naive_datetime =
        NaiveDateTime::from_timestamp_opt(record.expiry_date.unix_timestamp(), 0).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);

    let new_session = SessionDb {
        id: record.id.to_string(),
        data: rmp_serde::to_vec(&record).map_err(|_| diesel::result::Error::RollbackTransaction)?,
        expiry_date: datetime,
    };

    diesel::insert_into(sessions::table)
        .values(&new_session)
        .on_conflict(sessions::id)
        .do_update()
        .set((
            sessions::data.eq(excluded(sessions::data)),
            sessions::expiry_date.eq(excluded(sessions::expiry_date)),
        ))
        .execute(connection)?;

    Ok(())
}

pub fn delete_expired(connection: &mut PgConnection) -> Result<(), diesel::result::Error> {
    diesel::delete(sessions::table.filter(sessions::expiry_date.lt(Utc::now()))).execute(connection)?;

    Ok(())
}

pub fn get_by_id(connection: &mut PgConnection, id: String) -> Result<SessionDb, diesel::result::Error> {
    let session = sessions::table
        .filter(sessions::id.eq(id))
        .first::<SessionDb>(connection)?;

    Ok(session)
}

pub fn delete_by_id(connection: &mut PgConnection, id: String) -> Result<(), diesel::result::Error> {
    diesel::delete(sessions::table.filter(sessions::id.eq(id))).execute(connection)?;

    Ok(())
}
