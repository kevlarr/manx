table! {
    organization_users (id) {
        id -> Int4,
        created -> Timestamptz,
        name -> Text,
        user_id -> Nullable<Int4>,
        organization_id -> Int4,
    }
}

table! {
    organizations (id) {
        id -> Int4,
        created -> Timestamptz,
        title -> Text,
        uri_key -> Text,
        user_id -> Nullable<Int4>,
    }
}

table! {
    stream_topics (id) {
        id -> Int4,
        created -> Timestamptz,
        raw -> Text,
        rendered -> Text,
        organization_user_id -> Int4,
        stream_id -> Int4,
    }
}

table! {
    stream_users (id) {
        id -> Int4,
        created -> Timestamptz,
        organization_user_id -> Int4,
        stream_id -> Int4,
    }
}

table! {
    streams (id) {
        id -> Int4,
        created -> Timestamptz,
        name -> Text,
        uri_key -> Text,
        global -> Bool,
        organization_id -> Int4,
        organization_user_id -> Int4,
        parent_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        created -> Timestamptz,
        email -> Text,
        password -> Text,
    }
}

joinable!(organization_users -> organizations (organization_id));
joinable!(organization_users -> users (user_id));
joinable!(organizations -> users (user_id));
joinable!(stream_topics -> organization_users (organization_user_id));
joinable!(stream_topics -> streams (stream_id));
joinable!(stream_users -> organization_users (organization_user_id));
joinable!(stream_users -> streams (stream_id));
joinable!(streams -> organization_users (organization_user_id));
joinable!(streams -> organizations (organization_id));

allow_tables_to_appear_in_same_query!(
    organization_users,
    organizations,
    stream_topics,
    stream_users,
    streams,
    users,
);
