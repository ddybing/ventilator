-- This file initialises the database and all necessary tables.
-- We generated this SQL script by using the built in "SQL Generator" in Jetbrains RustRover.


create table __diesel_schema_migrations
(
    version VARCHAR(50)                         not null
        primary key,
    run_on  TIMESTAMP default CURRENT_TIMESTAMP not null
);

create table users
(
    id        integer not null
        constraint users_pk
            primary key autoincrement,
    username  TEXT    not null,
    password  TEXT    not null,
    login_key text    not null
);

create table posts
(
    id      integer not null
        constraint posts_pk
            primary key autoincrement,
    content TEXT    not null,
    time    INTEGER not null,
    user_id integer not null
        constraint posts_users_id_fk
            references users
);

create unique index posts_id_uindex
    on posts (id);

create unique index users_id_uindex
    on users (id);

create unique index users_login_key_uindex
    on users (login_key);

create unique index users_username_uindex
    on users (username);


