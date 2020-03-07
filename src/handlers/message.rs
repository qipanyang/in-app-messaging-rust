use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::content::{create as createContent, NewContent};
use crate::models::message::{create, find, Message, NewMessage};
use crate::models::user::find as findUser;
use crate::validate::validate;
use actix_web::web::{block, Data, Json, Path};
use chrono::{NaiveDateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;
use actix_web::HttpResponse;
use crate::models::inbox::{NewInbox, insert_inbox};
use crate::models::message_status::MessageStatus;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MessageResponse {
    pub id: String,
    pub content_id: String,
    pub sent_time: NaiveDateTime,
    pub user_id_triggered: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateMessageRequest {
    pub content: String,
    pub usernames: Vec<String>,
}

/// Get a message
pub async fn get_message(
    id: Path<String>,
    pool: Data<PoolType>,
) -> Result<Json<MessageResponse>, ApiError> {
    let message = block(move || find(&pool, id.to_owned())).await?;
    respond_json(message)
}

///// Create new message
//pub async fn create_message(
//    pool: Data<PoolType>,
//    params: Json<CreateMessageRequest>,
//) -> Result<Json<MessageResponse>, ApiError> {
//    validate(&params)?;
//    let new_message: NewMessage = NewMessage {
//        id: Uuid::new_v4().to_string(),
//        sent_time: Utc::now().naive_utc(),
//        content_id: params.content_id.to_owned(),
//        user_id_triggered: params.user_id_triggered,
//    };
//    let message = block(move || create(&pool, &new_message)).await?;
//    respond_json(message)
//}

pub async fn send_message(
    pool: Data<PoolType>,
    username: Path<String>,
    params: Json<CreateMessageRequest>,
) -> Result<HttpResponse, ApiError> {
    validate(&params)?;
    block(move || {
        // find the user id who send the message
        let sent_user_id = findUser(&pool, &username)?.id;

        // find the user id array who the message is sent to
        let mut sent_to_users = Vec::new();
        // Not Efficient
        for username in params.usernames.iter() {
            let sent_to_user = findUser(&pool, &username)?;
            sent_to_users.push(sent_to_user.id);
        }

        // create content
        let new_content = NewContent {
            id: Uuid::new_v4().to_string(),
            message_content: params.content.to_owned(),
        };

        let content = createContent(&pool, &new_content)?;
        let sent_time = Utc::now().naive_utc();

        // not efficient should be run in parallel
        for sent_to_user_id in sent_to_users {
            let new_message: NewMessage = NewMessage {
                id: Uuid::new_v4().to_string(),
                sent_time: sent_time.to_owned(),
                content_id: content.id.to_owned(),
                user_id_triggered: sent_user_id,
            };
            create(&pool, &new_message)?;
            let new_inbox = NewInbox {
                user_id: sent_to_user_id.to_owned(),
                message_id: new_message.id,
                status: MessageStatus::Unread as i32,
            };
            insert_inbox(&pool, &new_inbox)?;
        }
        Ok(())
    })
    .await?;

    respond_ok()
}

impl From<Message> for MessageResponse {
    fn from(message: Message) -> Self {
        MessageResponse {
            id: message.id,
            content_id: message.content_id,
            sent_time: message.sent_time,
            user_id_triggered: message.user_id_triggered,
        }
    }
}
