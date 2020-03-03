create table admins
(
    id         int auto_increment
        primary key,
    user_id    int                                 not null,
    user_role  int                                 not null comment '1 - Admin
5 - God',
    created_at timestamp default CURRENT_TIMESTAMP not null,
    updated_at timestamp default CURRENT_TIMESTAMP not null,
    constraint admins_user_id_uindex
        unique (user_id)
);

create table content
(
    id              int auto_increment
        primary key,
    message_content varchar(1024)                       not null,
    created_at      timestamp default CURRENT_TIMESTAMP not null,
    updated_at      timestamp default CURRENT_TIMESTAMP not null
);

create table inboxs
(
    id         int auto_increment
        primary key,
    user_id     int                                 not null,
    message_id  VARCHAR(36)                                 not null,
    status     int                                 not null comment '10 - unread
20 - read, 30 - archived',
    created_at timestamp default CURRENT_TIMESTAMP not null,
    updated_at timestamp default CURRENT_TIMESTAMP not null
);

create index inbox_status_index
    on inboxs (status, user_id);

create table messages
(
    id                VARCHAR(36)                         not null
        primary key,
    sent_time         timestamp default CURRENT_TIMESTAMP not null,
    content_id        int                                 not null,
    user_id_triggered int                                 not null comment 'The user who triggered this message',
    created_at        timestamp default CURRENT_TIMESTAMP not null,
    updated_at        timestamp default CURRENT_TIMESTAMP not null
);

create index messages_sent_time_index
    on messages (sent_time);

create index messages_user_id_triggered_index
    on messages (user_id_triggered);

create table users
(
    id         int auto_increment
        primary key,
    username   varchar(256)                        not null,
    created_at timestamp default CURRENT_TIMESTAMP not null,
    updated_at timestamp default CURRENT_TIMESTAMP not null,
    constraint table_name_username_uindex
        unique (username)
);

