table! {
    admins (id) {
        id -> Integer,
        user_id -> Integer,
        user_role -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    content (id) {
        id -> Integer,
        message_content -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    inboxs (id) {
        id -> Integer,
        userId -> Integer,
        messageId -> Integer,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    messages (id) {
        id -> Integer,
        sent_time -> Timestamp,
        content_id -> Integer,
        user_id_triggered -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    admins,
    content,
    inboxs,
    messages,
    users,
);
