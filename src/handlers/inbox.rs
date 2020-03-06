use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::respond_json;
use crate::models::inbox::{insert_inbox, find_by_user, Inbox, NewInbox};
use crate::validate::validate;
use actix_web::web::{block, Data, Json, Path};
use rayon::prelude::*;
use serde::Serialize;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct InboxResponse {
    pub id: i32,
    pub user_id: i32,
    pub message_id: String,
    pub status: i32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct InboxsResponse(pub Vec<InboxResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateInboxRequest {
    pub user_id: i32,
    #[validate(length(min = 3, message = "id length must be greater than 3"))]
    pub message_id: String,
    pub status: i32,
}

pub async fn insert(
    pool: Data<PoolType>,
    params: Json<CreateInboxRequest>,
) -> Result<Json<InboxResponse>, ApiError> {
    validate(&params)?;
    let new_inbox: NewInbox = NewInbox {
        user_id: params.user_id,
        message_id: params.message_id.to_owned(),
        status:params.status,
    };
    let inbox = block(move || insert_inbox(&pool, &new_inbox)).await?;
    respond_json(inbox)
}

pub async fn get_inbox_by_user(
    user_id: Path<i32>,
    pool: Data<PoolType>,
) -> Result<Json<InboxsResponse>, ApiError> {
    let inboxs = block(move || find_by_user(&pool, user_id.to_owned())).await?;
    respond_json(inboxs)
}


impl From<Inbox> for InboxResponse {
    fn from(inbox: Inbox) -> Self {
        InboxResponse {
            id: inbox.id,
            user_id: inbox.user_id,
            message_id: inbox.message_id,
            status: inbox.status,
        }
    }
}

impl From<Vec<Inbox>> for InboxsResponse {
    fn from(inboxs: Vec<Inbox>) -> Self {
        InboxsResponse(inboxs.into_par_iter().map(|inbox| inbox.into()).collect())
    }
}