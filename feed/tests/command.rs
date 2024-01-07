mod common;

use evento::Command;
use starter_feed::CreateFeedInput;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

use crate::common::get_producer;

async fn command() -> Command {
    Command::new(&get_producer().await.clone())
}

#[tokio::test]
async fn create() {
    let cmd = command().await;
    let events = cmd
        .execute(
            "en".to_owned(),
            &CreateFeedInput {
                title: "aze".into(),
                user_id: Uuid::new_v4().to_string(),
                request_id: None,
            },
        )
        .await
        .unwrap();

    sleep(Duration::from_millis(300)).await;

    assert!(!events.is_empty());
}
