create table organizations (
    id serial primary key,
    created timestamp with time zone not null,

    title text not null,
    uri_key text not null, -- eg. the identifier visible in a URI

    -- organizations should out-live creators
    user_id integer
        references users(id) on delete set null
);

create table organization_users (
    id serial primary key,
    created timestamp with time zone not null,

    name text not null,

    user_id integer
        -- if a user loses access to an org, their "name" should stick around
        references users(id) on delete set null,

    organization_id integer not null
        -- deleting an org. should delete everything
        references organizations(id) on delete cascade
);

create table streams (
    id serial primary key,
    created timestamp with time zone not null,

    name text not null,
    uri_key text not null,
    global boolean not null,

    organization_id integer not null
        -- deleting an org. should delete everything
        references organizations(id) on delete cascade,

    organization_user_id integer not null
        -- org. users only get deleted with entire org, so clear assoc. streams
        references organization_users(id) on delete cascade,

    parent_id integer
        -- deleting a parent stream kills the whole sub-tree
        references streams(id) on delete cascade
);

create table stream_users (
    id serial primary key,
    created timestamp with time zone not null,

    organization_user_id integer not null
        -- org. users only get deleted with entire org, so clear assoc. users
        references organization_users(id) on delete cascade,

    stream_id integer not null
        -- these store nothing extra, so delete with stream
        references streams(id) on delete cascade
);

create table stream_topics (
    id serial primary key,
    created timestamp with time zone not null,

    raw text not null,
    rendered text not null,

    organization_user_id integer not null
        -- org. users only get deleted with entire org, so clear assoc. topics
        references organization_users(id) on delete cascade,

    stream_id integer not null
        references streams(id) on delete cascade
);

create unique index organizations_uri_key_idx
on organizations(uri_key);

create unique index organization_users_organization_id_user_id_idx
on organization_users(organization_id, user_id);

create unique index streams_uri_key_idx
on streams(uri_key);

-- uri keys only need be unique within an org.
create unique index streams_organization_id_uri_key_idx
on streams(organization_id, uri_key);

-- only one global stream per organization
create unique index streams_organization_id_global_idx
on streams(organization_id) where global = true;

create unique index stream_users_stream_id_organization_user_id_idx
on stream_users(stream_id, organization_user_id);
