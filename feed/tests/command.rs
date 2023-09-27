mod common;

use std::time::Duration;
use timada_starter_feed::{CreateFeedInput, FeedCommand};
use tokio::time::sleep;
use uuid::Uuid;

use crate::common::get_producer;

async fn command() -> FeedCommand {
    FeedCommand {
        producer: get_producer().await.clone(),
        request_id: "".into(),
        user_id: Uuid::new_v4().to_string(),
        user_lang: "en".into(),
    }
}

#[tokio::test]
async fn create() {
    let cmd = command();
    let events = cmd
        .await
        .create(&CreateFeedInput { title: "".into() })
        .await
        .unwrap();

    sleep(Duration::from_millis(300)).await;

    assert!(!events.is_empty());
}
