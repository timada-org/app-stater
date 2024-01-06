use crate::FeedEvent;

use super::event::Created;
use evento::{
    store::{Applier, Event},
    Aggregate,
};
use serde::{Deserialize, Serialize};
use tracing::{error, warn};

#[derive(Default, Serialize, Deserialize, Aggregate)]
pub struct Feed {
    pub title: String,
}

impl Applier for Feed {
    fn apply(&mut self, event: &Event) {
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
}
