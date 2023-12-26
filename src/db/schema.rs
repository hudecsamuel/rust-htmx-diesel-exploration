// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Text,
        data -> Bytea,
        expiry_date -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
