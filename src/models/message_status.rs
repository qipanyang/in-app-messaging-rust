use std::str::FromStr;
use crate::errors::ApiError;

#[derive(Copy, Clone)]
pub enum MessageStatus {
    Unread = 10,
    Read = 20,
    Archived = 30,
    Saved = 40,
}

impl FromStr for MessageStatus {
    type Err = ApiError;
    fn from_str(s: &str) -> Result<MessageStatus, ApiError> {
        let s = s.to_lowercase();
        match s.as_str() {
            "unread" => Ok(MessageStatus::Unread),
            "read" => Ok(MessageStatus::Read),
            "archived" => Ok(MessageStatus::Archived),
            "saved" => Ok(MessageStatus::Saved),
            _ => Err(ApiError::ParseError(format!("Unknown Message Status: {}", s))),
        }
    }
}