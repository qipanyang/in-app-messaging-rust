use std::str::FromStr;
use crate::errors::ApiError;
use core::fmt;
use serde::export::Formatter;
use serde::{Serialize, Serializer};

#[derive(Copy, Clone, PartialEq)]
pub enum MessageStatus {
    Unread = 10,
    Read = 20,
    Archived = 30,
    Saved = 40,
}

impl MessageStatus {
    pub fn from_i32(n: i32) -> Result<MessageStatus, ApiError> {
        match n {
            10 => Ok(MessageStatus::Unread),
            20 => Ok(MessageStatus::Read),
            30 => Ok(MessageStatus::Archived),
            40 => Ok(MessageStatus::Saved),
            _ => Err(ApiError::ParseError(format!("Unknown Message Status: {}", n))),
        }
    }
}

impl Serialize for MessageStatus {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str( &format!("{:?}", self))
    }
}

impl fmt::Debug for MessageStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let s = match self {
            MessageStatus::Unread => "unread",
            MessageStatus::Read => "read",
            MessageStatus::Archived => "archived",
            MessageStatus::Saved => "saved",
        };
        write!(f, "{}", s)
    }
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