use crate::database::PoolType;
use crate::errors::ApiError;
//use crate::handlers::user::{UserResponse, UsersResponse};
use crate::schema::inboxs;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::models::message_status::MessageStatus;
use std::str::FromStr;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct Inbox {
    pub id: String,
    pub user_id: i32,
    pub message_id: String,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Insertable)]
#[table_name = "inboxs"]
pub struct NewInbox {
    pub id: String,
    pub user_id: i32,
    pub message_id: String,
    pub status: i32,
}

pub fn insert_inbox(pool: &PoolType, new_inbox: &NewInbox) -> Result<Inbox, ApiError> {
    use crate::schema::inboxs::dsl::inboxs;
    let conn = pool.get()?;
    let user_id = new_inbox.user_id;
    let message_id = new_inbox.message_id.to_owned();
    diesel::insert_into(inboxs)
        .values(new_inbox)
        .execute(&conn)?;
    find(pool, user_id, &message_id)
}

pub fn find(pool: &PoolType, user_id: i32, message_id: &str) -> Result<Inbox, ApiError> {
    use crate::schema::inboxs::dsl::{
        inboxs, message_id as message_id_pred, user_id as user_id_pred,
    };
    let not_found = format!("inbox item {:?} and {:?} not found", user_id, message_id);
    let conn = pool.get()?;
    let inbox = inboxs
        .filter(user_id_pred.eq(user_id).and(message_id_pred.eq(message_id)))
        .first::<Inbox>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(inbox)
}

pub fn find_by_user(pool: &PoolType, user_id: i32) -> Result<Vec<Inbox>, ApiError> {
    use crate::schema::inboxs::dsl::{inboxs, user_id as user_id_pred};
    let not_found = format!("inbox item {:?} not found", user_id);
    let conn = pool.get()?;
    let inbox_user = inboxs
        .filter(user_id_pred.eq(user_id))
        .load::<Inbox>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(inbox_user)
}

pub fn update_status(pool: &PoolType, message_id: &str, new_status: &str) -> Result<Inbox, ApiError> {
    let new_status = MessageStatus::from_str(new_status)? as i32;
    use crate::schema::inboxs::dsl::{message_id as message_id_pred, inboxs, status};
    let conn = pool.get()?;
    diesel::update(inboxs.filter(message_id_pred.eq(message_id)))
        .set(status.eq(new_status))
        .execute(&conn)?;
    let not_found = format!("inbox item {:?} not found", message_id);
    let inbox = inboxs
        .filter(message_id_pred.eq(message_id))
        .first::<Inbox>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;
    Ok(inbox)
}
