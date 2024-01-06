use anyhow::Result;
use async_trait::async_trait;
use evento::{store::Event, ConsumerContext, Query, QueryHandler, QueryOutput, RuleHandler};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{Created, FeedEvent};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TagCount {
    pub tag: String,
    pub total_count: i32,
}

#[derive(Clone)]
pub struct TagsCountHandler;

#[async_trait]
impl RuleHandler for TagsCountHandler {
    async fn handle(&self, event: Event, ctx: ConsumerContext) -> Result<()> {
        let db = ctx.extract::<PgPool>();
        let event_name: FeedEvent = event.name.parse()?;

        match event_name {
            FeedEvent::Created => {
                let data: Created = event.to_data()?;
                let mut query_builder: QueryBuilder<Postgres> =
                    QueryBuilder::new("INSERT INTO feed_tags_count (tag) ");

                query_builder.push_values(data.tags, |mut b, tag| {
                    b.push_bind(tag);
                });

                query_builder.push(" ON CONFLICT (tag) DO UPDATE SET total_count = feed_tags_count.total_count + 1");
                query_builder.build().execute(&db).await?;
            }
        };

        Ok(())
    }
}

pub struct ListPopularTagsInput;

#[async_trait]
impl QueryHandler for ListPopularTagsInput {
    type Output = Vec<TagCount>;
    async fn handle(&self, query: &Query) -> QueryOutput<Self::Output> {
        let db: sqlx::Pool<sqlx::Postgres> = query.extract::<PgPool>();
        Ok(sqlx::query_as!(
            TagCount,
            "SELECT * FROM feed_tags_count ORDER BY total_count DESC LIMIT 5"
        )
        .fetch_all(&db)
        .await?)
    }
}
