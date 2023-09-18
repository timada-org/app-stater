use std::{collections::HashSet, str::FromStr};

use anyhow::Result;
use evento::{Event, PgProducer};
use fake::{
    faker::company::en::Buzzword,
    faker::lorem::en::{Paragraph, Sentence},
    Fake,
};
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use uuid::Uuid;
use validator::Validate;

use crate::{Created, Feed, FeedEvent};

#[derive(Deserialize, Serialize)]
pub struct FeedMetadata {
    pub req_id: String,
    pub req_user: Uuid,
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
    pub async fn create(&self, _input: &CreateFeedInput) -> Result<Vec<Event>> {
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

        let events = self
            .producer
            .publish::<Feed, _>(
                Ulid::new(),
                vec![Event::new(FeedEvent::Created)
                    .data(Created {
                        title: Sentence(5..10).fake(),
                        content: Paragraph(50..100).fake(),
                        tags,
                    })?
                    .metadata(FeedMetadata {
                        req_user: Uuid::from_str(self.user_id.as_str())?,
                        req_id: self.request_id.to_owned(),
                    })?],
                0,
            )
            .await?;

        Ok(events)
    }
}
