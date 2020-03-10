use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::respond_json;
use crate::models::content::find as find_content;
use crate::models::inbox::{find_by_user, insert_inbox, update_status, Inbox, NewInbox};
use crate::models::message::find as find_message;
use crate::models::message_status::MessageStatus;
use crate::models::user::{find as find_user, find_by_id};
use crate::validate::validate;
use actix_web::web::{block, Data, Json, Path};
use chrono::NaiveDateTime;
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, PartialEq)]
pub struct InboxMessageResponse {
    pub id: String,
    pub username: String,
    pub content: String,
    pub sent_time: NaiveDateTime,
    pub username_triggered: String,
    pub status: MessageStatus,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct InboxMessagesResponse(pub Vec<InboxMessageResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateInboxRequest {
    pub user_id: i32,
    #[validate(length(min = 3, message = "id length must be greater than 3"))]
    pub message_id: String,
    pub status: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct ChangeInboxRequest {
    pub message_id: String,
    pub status: String,
}

pub async fn insert(
    pool: Data<PoolType>,
    params: Json<CreateInboxRequest>,
) -> Result<Json<Inbox>, ApiError> {
    validate(&params)?;
    let new_inbox: NewInbox = NewInbox {
        id: Uuid::new_v4().to_string(),
        user_id: params.user_id,
        message_id: params.message_id.to_owned(),
        status: params.status,
    };
    let inbox = block(move || insert_inbox(&pool, &new_inbox)).await?;
    respond_json(inbox)
}

pub async fn get_inbox_by_user(
    username: Path<String>,
    pool: Data<PoolType>,
) -> Result<Json<InboxMessagesResponse>, ApiError> {
    let inbox_messages = block(move || {
        let user = find_user(&pool, &username)?;
        let inboxs = find_by_user(&pool, user.id)?;
        let mut messages = Vec::with_capacity(inboxs.len());
        for inbox in inboxs {
            let message = find_message(&pool, &inbox.message_id)?;
            let username_triggered = find_by_id(&pool, message.user_id_triggered)?;
            let content = find_content(&pool, &message.content_id)?;

            let message = InboxMessageResponse {
                id: message.id,
                username: user.username.to_owned(),
                content: content.content,
                sent_time: message.sent_time,
                username_triggered: username_triggered.username,
                status: MessageStatus::from_i32(inbox.status)?,
            };

            messages.push(message);
        }
        Ok(InboxMessagesResponse(messages))
    })
    .await?;
    respond_json(inbox_messages)
}

pub async fn change_inbox_status(
    pool: Data<PoolType>,
    params: Json<ChangeInboxRequest>,
) -> Result<Json<Inbox>, ApiError> {
    validate(&params)?;
    let inbox = block(move || update_status(&pool, &params.message_id, &params.status)).await?;
    respond_json(inbox)
}

impl From<Vec<InboxMessageResponse>> for InboxMessagesResponse {
    fn from(inbox_messages: Vec<InboxMessageResponse>) -> Self {
        InboxMessagesResponse(inbox_messages.into_iter().collect())
    }
}
