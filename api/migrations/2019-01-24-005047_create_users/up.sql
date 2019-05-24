create table users (
    id serial primary key,
    created timestamp with time zone not null,
    email text not null,
    password text not null
);

create unique index users_email_idx on users (email);
