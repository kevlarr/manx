create table users (
    id serial primary key,
    created timestamp with time zone not null,
    email text not null,
    password text not null
);

create unique index users_email_idx on users (email);

create table teams (
    id serial primary key,

    -- a team must always have an ultimate owner
    owner_id integer not null references users(id) on delete no action,

    created timestamp with time zone not null,
    title text not null,
    key text not null
);

create unique index teams_key_idx on teams(key);

create table members (
    id serial primary key,

    -- deleting a team should delete everything
    team_id integer not null references teams(id) on delete cascade,

    -- if a user loses access to a team, their "name" should stick around
    user_id integer references users(id) on delete set null,

    created timestamp with time zone not null,
    name text not null
);

create unique index members_team_id_user_id_idx on members(team_id, user_id);

create table streams (
    id serial primary key,

    -- deleting a team should delete everything
    team_id integer not null references teams(id) on delete cascade,

    -- members only get deleted with entire team so cascade is fine
    member_id integer not null references members(id) on delete cascade,

    -- deleting a parent stream kills the whole sub-tree
    parent_id integer references streams(id) on delete cascade,

    created timestamp with time zone not null,
    title text not null,
    key text not null
);

create unique index streams_key_idx on streams(key);

create table stream_permissions (
    id serial primary key,

    -- members only get deleted with entire team
    member_id integer not null references members(id) on delete cascade,

    -- no need for permissions if stream gets deleted
    stream_id integer not null references streams(id) on delete cascade,

    created timestamp with time zone not null
);

create unique index stream_permissions_member_id_stream_id_idx on stream_permissions(member_id, stream_id);

create table topics (
    id serial primary key,

    member_id integer not null references members(id) on delete cascade,
    stream_id integer not null references streams(id) on delete cascade,

    created timestamp with time zone not null,
    updated timestamp with time zone not null,
    raw text not null,
    rendered text not null,
    key text not null
);

create unique index topics_key_idx on topics(key);

create table comments (
    id serial primary key,

    member_id integer not null references members(id) on delete cascade,
    topic_id integer not null references topics(id) on delete cascade,

    created timestamp with time zone not null,
    updated timestamp with time zone not null,
    raw text not null,
    rendered text not null
);
