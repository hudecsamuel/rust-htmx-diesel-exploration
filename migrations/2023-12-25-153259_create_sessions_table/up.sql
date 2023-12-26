create table sessions (
    id text primary key not null,
    data bytea not null,
    expiry_date timestamptz not null
)