use anyhow::Result;
use chrono::{DateTime, Utc};
use evento::{Aggregate, Subscriber};
use evento_query::{Cursor, CursorError, Query, QueryArgs, QueryResult};
use fake::{faker::name::en::Name, Fake};
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query::QueryAs, FromRow, PgPool, Postgres};
use tracing::warn;
use uuid::Uuid;

use crate::{CommandMetadata, Created, Feed, FeedEvent, FeedQuery};

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

    fn deserialize(values: Vec<&str>) -> Result<Self, CursorError> {
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

pub fn feeds_subscriber() -> Subscriber {
    Subscriber::new("feeds")
        .filter("feed/**")
        .handler(|event, ctx| {
            async move {
                let Ok(feed_event) = event.name.parse::<FeedEvent>() else {
                    warn!(
                        "FeedEvent.{} not handled by Feed feeds projection",
                        event.name
                    );

                    return Ok(());
                };

                let metadata = event.to_metadata::<CommandMetadata>()?;
                let db = ctx.extract::<PgPool>();

                match feed_event {
                    FeedEvent::Created => {
                        let data: Created = event.to_data()?;

                        let feed = UserFeed {
                            id: Feed::to_id(event.aggregate_id),
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
                            feed.id,
                            feed.user_id,
                            feed.title,
                            feed.author,
                            feed.content,
                            feed.content_short,
                            &feed.tags,
                            feed.created_at,
                        ).execute(&db)
                        .await?;
                    }
                };
                Ok(())
            }
            .boxed()
        })
}

#[derive(Deserialize)]
pub struct ListFeedsInput {
    pub args: QueryArgs,
    pub tag: Option<String>,
}

impl FeedQuery {
    pub async fn list_feeds(&self, input: ListFeedsInput) -> Result<QueryResult<UserFeed>> {
        let query = match input.tag {
            Some(tag) => {
                Query::<UserFeed>::new("SELECT * FROM feed_feeds WHERE tags @> ARRAY[$1]").bind(tag)
            }
            None => Query::<UserFeed>::new("SELECT * FROM feed_feeds"),
        };

        Ok(query.build(input.args).fetch_all(&self.db).await?)
    }

    pub async fn get_feed(&self, id: String) -> Result<Option<UserFeed>> {
        Ok(
            sqlx::query_as!(UserFeed, "SELECT * FROM feed_feeds where id = $1", id)
                .fetch_optional(&self.db)
                .await?,
        )
    }
}
