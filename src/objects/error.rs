use serde::Deserialize;
use std::fmt;
use std::error::Error as BaseError;

// This standard Notion API error
// See: https://developers.notion.com/reference/errors
#[derive(Deserialize, Debug)]
pub struct Error {
    pub object: String,
    pub status: i32,
    pub code: String,
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Status: {}; code: {}; message: {}", self.status, self.code, self.message)
    }
}

impl BaseError for Error {}