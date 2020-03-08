use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::respond_json;
use crate::models::content::{create, find, Content, NewContent};
use crate::validate::validate;
use actix_web::web::{block, Data, Json, Path};
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ContentResponse {
    pub id: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateContentRequest {
    #[validate(length(max = 1024, message = "content length must be less than 1024"))]
    pub content: String,
}

/// Get a content
pub async fn get_content(
    id: Path<String>,
    pool: Data<PoolType>,
) -> Result<Json<ContentResponse>, ApiError> {
    let content = block(move || find(&pool, &id)).await?;
    respond_json(content)
}

/// Create new content
pub async fn create_content(
    pool: Data<PoolType>,
    params: Json<CreateContentRequest>,
) -> Result<Json<ContentResponse>, ApiError> {
    validate(&params)?;
    let new_content: NewContent = NewContent {
        id: Uuid::new_v4().to_string(),
        message_content: params.content.to_owned(),
    };
    let content = block(move || create(&pool, &new_content)).await?;
    respond_json(content)
}

impl From<Content> for ContentResponse {
    fn from(content: Content) -> Self {
        ContentResponse {
            id: content.id,
            content: content.message_content,
        }
    }
}
