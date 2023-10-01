use evento::PgEvento;
use futures::FutureExt;
use starter_feed::{FeedEvent, FeedMetadata, UserFeed};
use tracing::warn;

use crate::{routes, state::WebContext};

pub enum SubscribeEvent<D, M> {
    Created(D, M),
    #[allow(dead_code)]
    Updated(D, M),
    #[allow(dead_code)]
    Deleted(D, M),
}

pub fn subscribe(evento: PgEvento) -> PgEvento {
    evento
        .subscribe(starter_feed::feeds_subscriber().post_handler(|event, ctx| {
            async move {
                let Ok(feed_event) = event.name.parse::<FeedEvent>() else {
                    warn!(
                        "FeedEvent.{} not handled in app/src/subscriber.rs",
                        event.name
                    );

                    return Ok(());
                };

                let pikav = ctx.extract::<pikav_client::Client>();
                let feed: UserFeed = event.to_data()?;
                let metadata = event.to_metadata::<FeedMetadata>()?;
                let web_context: WebContext = (&ctx, metadata.user_lang.to_owned()).into();

                let event = match feed_event {
                    FeedEvent::Created => SubscribeEvent::Created(feed, metadata),
                };

                routes::subscribe(web_context, pikav, event).await?;
                Ok(())
            }
            .boxed()
        }))
        .subscribe(starter_feed::tags_count_subscriber())
}
