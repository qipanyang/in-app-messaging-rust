use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::message::MessageResponse;
use crate::schema::messages;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::RunQueryDsl;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct Message {
    pub id: String,
    pub sent_time: NaiveDateTime,
    pub content_id: String,
    pub user_id_triggered: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Insertable)]
#[table_name = "messages"]
pub struct NewMessage {
    pub id: String,
    pub sent_time: NaiveDateTime,
    pub content_id: String,
    pub user_id_triggered: i32,
}

pub fn find(pool: &PoolType, id: &str) -> Result<MessageResponse, ApiError> {
    use crate::schema::messages::dsl::{id as id_pred, messages};
    let not_found = format!("Message {} not found", id);
    let conn = pool.get()?;
    let message = messages
        .filter(id_pred.eq(id))
        .first::<Message>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;
    Ok(message.into())
}

/// Create new message
pub fn create(pool: &PoolType, new_message: &NewMessage) -> Result<MessageResponse, ApiError> {
    use crate::schema::messages::dsl::messages;
    let conn = pool.get()?;
    let id = new_message.id.to_owned();
    diesel::insert_into(messages)
        .values(new_message)
        .execute(&conn)?;
    find(pool, &id)
}
