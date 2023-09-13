use anyhow::Result;
use evento::{Event, PgProducer};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use validator::Validate;

use crate::{Created, Feed, FeedEvent};

#[derive(Deserialize, Serialize)]
pub struct CommandMetadata {
    pub request_id: String,
    pub user_id: String,
}

#[derive(Clone)]
pub struct FeedCommand {
    pub producer: PgProducer,
    pub user_id: String,
    pub request_id: String,
}

#[derive(Deserialize, Validate)]
pub struct CreateFeedInput {
    #[validate(length(min = 3, max = 100))]
    pub title: String,
}

impl FeedCommand {
    pub async fn create(&self, input: &CreateFeedInput) -> Result<Vec<Event>> {
        let events = self
            .producer
            .publish::<Feed, _>(
                Ulid::new(),
                vec![Event::new(FeedEvent::Created)
                    .data(Created {
                        title: input.title.to_owned(),
                    })?
                    .metadata(CommandMetadata {
                        user_id: self.user_id.to_owned(),
                        request_id: self.request_id.to_owned(),
                    })?],
                0,
            )
            .await?;

        Ok(events)
    }
}
