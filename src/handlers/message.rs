use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::respond_json;
use crate::models::message::{create, find, Message, NewMessage};
use crate::validate::validate;
use actix_web::web::{block, Data, Json, Path};
use chrono::{NaiveDateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MessageResponse {
    pub id: String,
    pub content_id: String,
    pub sent_time: NaiveDateTime,
    pub user_id_triggered: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateMessageRequest {
    pub content_id: String,
    pub user_id_triggered: i32,
}

/// Get a message
pub async fn get_message(
    id: Path<String>,
    pool: Data<PoolType>,
) -> Result<Json<MessageResponse>, ApiError> {
    let message = block(move || find(&pool, id.to_owned())).await?;
    respond_json(message)
}

/// Create new message
pub async fn create_message(
    pool: Data<PoolType>,
    params: Json<CreateMessageRequest>,
) -> Result<Json<MessageResponse>, ApiError> {
    validate(&params)?;
    let new_message: NewMessage = NewMessage {
        id: Uuid::new_v4().to_string(),
        sent_time: Utc::now().naive_utc(),
        content_id: params.content_id.to_owned(),
        user_id_triggered: params.user_id_triggered,
    };
    let message = block(move || create(&pool, &new_message)).await?;
    respond_json(message)
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
