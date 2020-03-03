create table admin
(
    id         int auto_increment
        primary key,
    user_id    int                                 not null,
    user_role  int                                 not null comment '1 - Admin
5 - God',
    created_at timestamp default CURRENT_TIMESTAMP not null,
    updated_at timestamp default CURRENT_TIMESTAMP not null,
    constraint admin_user_id_uindex
        unique (user_id)
);

create table content
(
    id         int auto_increment
        primary key,
    content    varchar(1024)                       not null,
    created_at timestamp default CURRENT_TIMESTAMP not null,
    updated_at timestamp default CURRENT_TIMESTAMP not null
);

create table inbox
(
    id         int auto_increment
        primary key,
    userId     int                                 not null,
    messageId  int                                 not null,
    status            int                                 not null comment '10 - unread
20 - read, 30 - archived',
    created_at timestamp default CURRENT_TIMESTAMP not null,
    updated_at timestamp default CURRENT_TIMESTAMP not null
);

create table message
(
    id                int auto_increment
        primary key,
    sent_time         timestamp default CURRENT_TIMESTAMP not null,
    content_id        int                                 not null,
    user_id_triggered int                                 not null comment 'The user who triggered this message',
    created_at        timestamp default CURRENT_TIMESTAMP not null,
    updated_at        timestamp default CURRENT_TIMESTAMP not null
);

create index message_sent_time_index
    on message (sent_time);

create index message_status_index
    on message (status);

create index message_user_id_triggered_index
    on message (user_id_triggered);

create table user
(
    id         int auto_increment
        primary key,
    user_name  varchar(256)                        not null,
    created_at timestamp default CURRENT_TIMESTAMP not null,
    update_at  timestamp default CURRENT_TIMESTAMP not null,
    constraint table_name_user_name_uindex
        unique (user_name)
);

