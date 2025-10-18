-- Add up migration script here
create table users(
    id integer primary key not null,
    openid text not null,
    session_key text not null,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP

);

create unique index users_openid_index on users(openid);