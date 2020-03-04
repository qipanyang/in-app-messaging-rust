use crate::schema::messages;
use chrono::NaiveDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct Message {
    pub id: String,
    pub sent_time: NaiveDateTime,
    pub content_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
