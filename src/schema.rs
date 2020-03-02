table! {
    admin (id) {
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
        content -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    inbox (id) {
        id -> Integer,
        userId -> Integer,
        messageId -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    message (id) {
        id -> Integer,
        sent_time -> Timestamp,
        status -> Integer,
        content_id -> Integer,
        user_id_triggered -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user (id) {
        id -> Integer,
        user_name -> Varchar,
        created_at -> Timestamp,
        update_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    admin,
    content,
    inbox,
    message,
    user,
);
