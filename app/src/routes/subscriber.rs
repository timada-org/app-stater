use evento::Subscriber;
use futures::FutureExt;
use leptos::*;
use pikav_client::timada::SimpleEvent;
use starter_feed::{FeedEvent, FeedMetadata, UserFeed};
use tracing::warn;

use crate::state::WebContext;

use super::component::*;

pub fn subscriber() -> Subscriber {
    starter_feed::feeds_subscriber().post_handler(|event, ctx| {
        async move {
            let Ok(feed_event) = event.name.parse::<FeedEvent>() else {
                warn!(
                    "FeedEvent.{} not handled in app/src/routes/subscriber.rs",
                    event.name
                );

                return Ok(());
            };

            let pikav = ctx.extract::<pikav_client::Client>();
            let metadata = event.to_metadata::<FeedMetadata>()?;
            let web_context: WebContext = (&ctx, metadata.user_lang).into();

            match feed_event {
                FeedEvent::Created => {
                    let feed: UserFeed = event.to_data()?;

                    let html = web_context.html(move || {
                        view! {
                            <Feed
                                feed
                                tag=None
                                cursor=None
                                hs="init remove @disabled from #form-title then call #form-title.focus()"
                            />
                        }
                    });

                    pikav.publish(vec![SimpleEvent {
                        user_id: metadata.req_user.to_string(),
                        topic: "root".to_owned(),
                        event: event.name,
                        data: html
                    }])
                }
            };

            Ok(())
        }
        .boxed()
    })
}
