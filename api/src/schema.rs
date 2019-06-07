table! {
    comments (id) {
        id -> Int4,
        member_id -> Int4,
        topic_id -> Int4,
        created -> Timestamptz,
        updated -> Timestamptz,
        raw -> Text,
        rendered -> Text,
    }
}

table! {
    members (id) {
        id -> Int4,
        team_id -> Int4,
        user_id -> Nullable<Int4>,
        created -> Timestamptz,
        name -> Text,
    }
}

table! {
    stream_permissions (id) {
        id -> Int4,
        member_id -> Int4,
        stream_id -> Int4,
        created -> Timestamptz,
    }
}

table! {
    streams (id) {
        id -> Int4,
        team_id -> Int4,
        member_id -> Int4,
        parent_id -> Nullable<Int4>,
        created -> Timestamptz,
        title -> Text,
        key -> Text,
    }
}

table! {
    teams (id) {
        id -> Int4,
        owner_id -> Int4,
        created -> Timestamptz,
        title -> Text,
        key -> Text,
    }
}

table! {
    topics (id) {
        id -> Int4,
        member_id -> Int4,
        stream_id -> Int4,
        created -> Timestamptz,
        updated -> Timestamptz,
        raw -> Text,
        rendered -> Text,
        key -> Text,
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

joinable!(comments -> members (member_id));
joinable!(comments -> topics (topic_id));
joinable!(members -> teams (team_id));
joinable!(members -> users (user_id));
joinable!(stream_permissions -> members (member_id));
joinable!(stream_permissions -> streams (stream_id));
joinable!(streams -> members (member_id));
joinable!(streams -> teams (team_id));
joinable!(teams -> users (owner_id));
joinable!(topics -> members (member_id));
joinable!(topics -> streams (stream_id));

allow_tables_to_appear_in_same_query!(
    comments,
    members,
    stream_permissions,
    streams,
    teams,
    topics,
    users,
);
