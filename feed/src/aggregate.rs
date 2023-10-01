use crate::FeedEvent;

use super::event::Created;
use evento::Aggregate;
use serde::{Deserialize, Serialize};
use tracing::{error, warn};

#[derive(Default, Serialize, Deserialize)]
pub struct Feed {
    pub title: String,
}

impl Aggregate for Feed {
    fn apply(&mut self, event: &evento::Event) {
        let Ok(feed_event) = event.name.parse() else {
            warn!("FeedEvent.{} not handled by Feed aggregate", event.name);
            return;
        };

        match feed_event {
            FeedEvent::Created => {
                let data = match event.to_data::<Created>() {
                    Ok(data) => data,
                    Err(e) => {
                        error!("Feed.apply {} {}", event.name, e);
                        return;
                    }
                };

                self.title = data.title;
            }
        }
    }

    fn aggregate_type<'a>() -> &'a str {
        "feed"
    }
}
