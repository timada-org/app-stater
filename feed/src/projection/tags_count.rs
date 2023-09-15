use anyhow::Result;
use evento::Subscriber;
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, QueryBuilder};
use tracing::warn;

use crate::{Created, FeedEvent, FeedQuery};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TagCount {
    pub tag: String,
    pub total_count: i32,
}

pub fn tags_count_subscriber() -> Subscriber {
    Subscriber::new("tags-count")
        .filter("feed/**")
        .handler(|event, ctx| {
            async move {
                let Ok(feed_event) = event.name.parse::<FeedEvent>() else {
                    warn!(
                        "FeedEvent.{} not handled by Feed tags_count projection",
                        event.name
                    );

                    return Ok(());
                };

                let db = ctx.0.read().extract::<PgPool>().clone();

                match feed_event {
                    FeedEvent::Created => {
                        let data: Created = event.to_data()?;
                        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                            "INSERT INTO feed_tags_count (tag) "
                        );

                        query_builder.push_values(data.tags, |mut b, tag| {
                            b.push_bind(tag);
                        });

                        query_builder.push(" ON CONFLICT (tag) DO UPDATE SET total_count = feed_tags_count.total_count + 1");
                        query_builder.build().execute(&db).await?;
                    }
                };

                Ok(())
            }
            .boxed()
        })
}

impl FeedQuery {
    pub async fn list_popular_tags(&self) -> Result<Vec<TagCount>> {
        Ok(sqlx::query_as!(
            TagCount,
            "SELECT * FROM feed_tags_count ORDER BY total_count DESC LIMIT 5"
        )
        .fetch_all(&self.db)
        .await?)
    }
}
