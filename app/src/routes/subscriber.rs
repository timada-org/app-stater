use evento::Subscriber;
use futures::FutureExt;
use leptos::*;
use pikav_client::timada::SimpleEvent;
use timada_starter_feed::{FeedMetadata, FeedProjectionEvent, UserFeed};
use tracing::warn;

use crate::state::WebContext;

use super::component::*;

pub fn subscriber() -> Subscriber {
    Subscriber::new("root")
        .set_from_start(false)
        .filter("feed-feeds/**")
        .handler(|event, ctx| {
            async move {
                let Ok(feed_event) = event.name.parse::<FeedProjectionEvent>() else {
                    warn!(
                        "FeedProjectionEvent.{} not handled by Feed feeds projection",
                        event.name
                    );

                    return Ok(());
                };

                let pikav = ctx.extract::<pikav_client::Client>();
                let metadata = event.to_metadata::<FeedMetadata>()?;
                let web_context: WebContext = (&ctx, metadata.user_lang).into();

                match feed_event {
                    FeedProjectionEvent::Created => {
                        let data: UserFeed = event.to_data()?;

                        let html = web_context.html(move || {
                            view! {
                                <Feed
                                    feed=data
                                    tag=None
                                    cursor=None
                                    attr:_="init remove @disabled from #form-title then call #form-title.focus()"
                                />
                            }
                        });

                        pikav.publish(vec![SimpleEvent {
                            user_id: metadata.req_user.to_string(),
                            topic: "root".to_owned(),
                            event: event.name,
                            data: html.replace("\n", " ")
                        }])
                    }
                    FeedProjectionEvent::Updated => todo!(),
                    FeedProjectionEvent::Deleted => todo!(),
                };

                Ok(())
            }
            .boxed()
        })
}
