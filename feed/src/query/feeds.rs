use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use evento::{
    store::Event, Aggregate, ConsumerContext, Query, QueryHandler, QueryOutput, RuleHandler,
};
use evento_query::{Cursor, CursorType, PgQuery, QueryArgs, QueryResult};
use fake::{faker::name::en::Name, Fake};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query::QueryAs, FromRow, PgPool, Postgres};
use uuid::Uuid;

use crate::{Created, Feed, FeedEvent, FeedMetadata};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, FromRow)]
pub struct UserFeed {
    pub id: String,
    pub title: String,
    pub author: String,
    pub content: String,
    pub content_short: String,
    pub total_likes: i32,
    pub tags: Vec<String>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct FeedDetailsHandler;

#[async_trait]
impl RuleHandler for FeedDetailsHandler {
    async fn handle(&self, event: Event, ctx: ConsumerContext) -> Result<()> {
        let db = ctx.extract::<PgPool>();
        let event_name: FeedEvent = event.name.parse()?;
        let Some(metadata) = event.to_metadata::<FeedMetadata>()? else {
            return Ok(());
        };

        match event_name {
            FeedEvent::Created => {
                let data: Created = event.to_data()?;

                let feed = UserFeed {
                    id: Feed::from_aggregate_id(&event.aggregate_id),
                    user_id: metadata.req_user,
                    title: data.title,
                    author: Name().fake(),
                    content_short: data.content.chars().take(250).collect(),
                    content: data.content,
                    total_likes: 0,
                    tags: data.tags,
                    created_at: event.created_at,
                };

                sqlx::query!(
                    r#"
                    INSERT INTO feed_feeds (id, user_id, title, author, content, content_short, tags, created_at)
                    VALUES ( $1, $2, $3, $4, $5, $6, $7, $8 )
                    "#,
                    &feed.id,
                    &feed.user_id,
                    &feed.title,
                    &feed.author,
                    &feed.content,
                    &feed.content_short,
                    &feed.tags,
                    &feed.created_at,
                ).execute(&db)
                .await?;
            }
        };

        Ok(())
    }
}

impl Cursor for UserFeed {
    fn keys() -> Vec<&'static str> {
        vec!["created_at", "id"]
    }

    fn bind<'q, O>(
        self,
        query: QueryAs<Postgres, O, PgArguments>,
    ) -> QueryAs<Postgres, O, PgArguments>
    where
        O: for<'r> FromRow<'r, <sqlx::Postgres as sqlx::Database>::Row>,
        O: 'q + std::marker::Send,
        O: 'q + Unpin,
        O: 'q + Cursor,
    {
        query.bind(self.created_at).bind(self.id)
    }

    fn serialize(&self) -> Vec<String> {
        vec![Self::serialize_utc(self.created_at), self.id.to_owned()]
    }

    fn deserialize(values: Vec<&str>) -> Result<Self, evento_query::QueryError> {
        let mut values = values.iter();
        let created_at = Self::deserialize_as_utc("created_at", values.next())?;
        let id = Self::deserialize_as("id", values.next())?;

        Ok(UserFeed {
            id,
            created_at,
            ..Default::default()
        })
    }
}

#[derive(Deserialize)]
pub struct ListFeedsInput {
    pub first: Option<u16>,
    pub after: Option<CursorType>,
    pub last: Option<u16>,
    pub before: Option<CursorType>,
    pub tag: Option<String>,
}

#[async_trait]
impl QueryHandler for ListFeedsInput {
    type Output = QueryResult<UserFeed>;
    async fn handle(&self, query: &Query) -> QueryOutput<Self::Output> {
        let db: sqlx::Pool<sqlx::Postgres> = query.extract::<PgPool>();
        let query = match &self.tag {
            Some(tag) => {
                PgQuery::<UserFeed>::new("SELECT * FROM feed_feeds WHERE tags @> ARRAY[$1]")
                    .bind(tag)
            }
            None => PgQuery::<UserFeed>::new("SELECT * FROM feed_feeds"),
        };

        Ok(query
            .build_desc(QueryArgs {
                first: self.first.to_owned(),
                after: self.after.to_owned(),
                last: self.last.to_owned(),
                before: self.before.to_owned(),
            })
            .fetch_all(&db)
            .await?)
    }
}

#[derive(Deserialize)]
pub struct GetFeedInput {
    pub id: String,
}

#[async_trait]
impl QueryHandler for GetFeedInput {
    type Output = Option<UserFeed>;
    async fn handle(&self, query: &Query) -> QueryOutput<Self::Output> {
        let db: sqlx::Pool<sqlx::Postgres> = query.extract::<PgPool>();
        Ok(
            sqlx::query_as!(UserFeed, "SELECT * FROM feed_feeds where id = $1", self.id)
                .fetch_optional(&db)
                .await?,
        )
    }
}
