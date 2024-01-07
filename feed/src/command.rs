use async_trait::async_trait;
use evento::{Command, CommandHandler, CommandOutput};
use fake::{
    faker::company::en::Buzzword,
    faker::lorem::en::{Paragraph, Sentence},
    Fake,
};
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, str::FromStr};
use ulid::Ulid;
use uuid::Uuid;
use validator::Validate;

use crate::{Created, Feed};

#[derive(Deserialize, Serialize)]
pub struct FeedMetadata {
    pub req_id: String,
    pub req_user: Uuid,
}

#[derive(Deserialize, Validate)]
pub struct CreateFeedInput {
    #[validate(length(min = 3, max = 100))]
    pub title: String,
    pub user_id: String,
    pub request_id: Option<String>,
}

#[async_trait]
impl CommandHandler for CreateFeedInput {
    async fn handle(&self, cmd: &Command) -> CommandOutput {
        let tags: Vec<String> = [
            Buzzword().fake(),
            Buzzword().fake(),
            Buzzword().fake(),
            Buzzword().fake(),
            Buzzword().fake(),
        ]
        .choose_multiple(&mut rand::thread_rng(), rand::thread_rng().gen_range(1..6))
        .cloned()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

        let events = cmd
            .write(Ulid::new())
            .metadata(FeedMetadata {
                req_user: Uuid::from_str(self.user_id.as_str())?,
                req_id: self
                    .request_id
                    .to_owned()
                    .unwrap_or(Uuid::new_v4().to_string()),
            })?
            .event(Created {
                title: Sentence(5..10).fake(),
                content: Paragraph(50..100).fake(),
                tags,
            })?
            .commit::<Feed>()
            .await?;

        Ok(events)
    }
}
