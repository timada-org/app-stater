use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};

#[derive(Display, FromStr)]
#[display(style = "kebab-case")]
pub enum FeedProjectionEvent {
    Created,
    Updated,
    Deleted,
}

impl From<FeedProjectionEvent> for String {
    fn from(e: FeedProjectionEvent) -> Self {
        e.to_string()
    }
}

#[derive(Display, FromStr)]
#[display(style = "kebab-case")]
pub enum FeedEvent {
    Created,
}

impl From<FeedEvent> for String {
    fn from(e: FeedEvent) -> Self {
        e.to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Created {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}
