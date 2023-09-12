use anyhow::Result;
use evento::{Event, PgProducer};
use serde::Deserialize;
use ulid::Ulid;
use validator::Validate;

use crate::{Created, Feed, FeedEvent};

#[derive(Deserialize, Validate)]
pub struct CreateFeedInput {
    #[validate(length(min = 3, max = 100))]
    pub title: String,
}

pub async fn create_feed(producer: &PgProducer, input: &CreateFeedInput) -> Result<Vec<Event>> {
    let events = producer
        .publish::<Feed, _>(
            Ulid::new(),
            vec![Event::new(FeedEvent::Created).data(Created { title: input.title.to_owned() })?],
            0,
        )
        .await?;

    Ok(events)
}
