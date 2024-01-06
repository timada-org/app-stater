use evento::PublisherEvent;
use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};

#[derive(Display, FromStr, PublisherEvent)]
#[display(style = "kebab-case")]
pub enum FeedEvent {
    Created,
}

#[derive(Serialize, Deserialize)]
pub struct Created {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}
