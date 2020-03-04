use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::content::ContentResponse;
use crate::schema::contents;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::RunQueryDsl;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct Content {
    pub id: String,
    pub message_content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Insertable)]
#[table_name = "contents"]
pub struct NewContent {
    pub id: String,
    pub message_content: String,
}

pub fn find(pool: &PoolType, id: String) -> Result<ContentResponse, ApiError> {
    use crate::schema::contents::dsl::{contents, id as id_pred};
    let not_found = format!("Content {} not found", id);
    let conn = pool.get()?;
    let content = contents
        .filter(id_pred.eq(id))
        .first::<Content>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;
    Ok(content.into())
}

/// Create new content
pub fn create(pool: &PoolType, new_content: &NewContent) -> Result<ContentResponse, ApiError> {
    use crate::schema::contents::dsl::contents;
    let conn = pool.get()?;
    let id = new_content.id.to_owned();
    diesel::insert_into(contents)
        .values(new_content)
        .execute(&conn)?;
    find(pool, id)
}
