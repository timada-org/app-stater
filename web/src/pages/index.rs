use std::collections::HashMap;

use askama::Template;
use askama_axum::Response;
use axum::{async_trait, extract::Query, Form};
use evento::{store::Event, Aggregate, ConsumerContext, RuleHandler};
use evento_query::{Cursor, CursorType, Edge, QueryResult};
use pikav_client::timada::SimpleEvent;
use serde::Deserialize;
use starter_feed::{
    Created, Feed, FeedEvent, FeedMetadata, ListFeedsInput, ListPopularTagsInput, TagCount,
    UserFeed,
};
use validator::Validate;

use crate::{
    config::Config,
    context::{Context, UserContext},
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    ctx: Context,
    tag: Option<String>,
    prev_tag: Option<String>,
    feeds: QueryResult<UserFeed>,
    popular_tags: Vec<TagCount>,
    errors: HashMap<String, Vec<String>>,
    global_link: String,
}

impl IndexTemplate {
    fn end_cursor(&self, feed: &Edge<UserFeed>) -> Option<CursorType> {
        self.feeds
            .page_info
            .end_cursor
            .to_owned()
            .and_then(|cursor| {
                if cursor == feed.cursor && self.feeds.page_info.has_next_page {
                    Some(cursor)
                } else {
                    None
                }
            })
    }

    fn query_tag(&self) -> String {
        self.tag
            .as_ref()
            .map(|tag| format!("&tag={tag}"))
            .unwrap_or("".to_owned())
    }

    fn sse_tag_suffix(&self) -> String {
        self.tag
            .as_ref()
            .map(|tag| format!("-{tag}"))
            .unwrap_or_default()
    }
}

#[derive(Deserialize)]
pub struct IndexQuery {
    tag: Option<String>,
    prev_tag: Option<String>,
}

pub async fn index(
    ctx: Context,
    Query(input): Query<IndexQuery>,
    Query(list_feeds_input): Query<ListFeedsInput>,
) -> Result<IndexTemplate, Response> {
    let (feeds, popular_tags) =
        tokio::try_join!(ctx.query(list_feeds_input), ctx.query(ListPopularTagsInput))?;

    let global_link = input
        .tag
        .as_ref()
        .map(|tag| format!("?prev_tag={tag}"))
        .unwrap_or_default();

    Ok(IndexTemplate {
        ctx,
        feeds,
        popular_tags,
        global_link,
        tag: input.tag,
        prev_tag: input.prev_tag,
        errors: Default::default(),
    })
}

#[derive(Template)]
#[template(path = "feeds_list.html")]
pub struct FeedsListTemplate {
    ctx: UserContext,
    feeds: QueryResult<UserFeed>,
    tag: Option<String>,
}

impl FeedsListTemplate {
    fn end_cursor(&self, feed: &Edge<UserFeed>) -> Option<CursorType> {
        self.feeds
            .page_info
            .end_cursor
            .to_owned()
            .and_then(|cursor| {
                if cursor == feed.cursor && self.feeds.page_info.has_next_page {
                    Some(cursor)
                } else {
                    None
                }
            })
    }

    fn query_tag(&self) -> String {
        self.tag
            .as_ref()
            .map(|tag| format!("&tag={tag}"))
            .unwrap_or("".to_owned())
    }
}

pub async fn load_more(
    ctx: UserContext,
    Query(input): Query<ListFeedsInput>,
) -> Result<FeedsListTemplate, Response> {
    let tag = input.tag.to_owned();
    let feeds = ctx.query(input).await?;

    Ok(FeedsListTemplate { ctx, feeds, tag })
}

#[derive(Template)]
#[template(path = "create_feed_form.html")]
pub struct CreateFeedFormTemplate {
    ctx: UserContext,
    errors: HashMap<String, Vec<String>>,
}

#[derive(Deserialize, Validate)]
pub struct CreateFeedInput {
    #[validate(length(min = 3, max = 100))]
    pub title: String,
}

pub async fn create_feed(
    ctx: UserContext,
    Form(input): Form<CreateFeedInput>,
) -> Result<CreateFeedFormTemplate, Response> {
    let errors = ctx
        .execute(starter_feed::CreateFeedInput {
            title: input.title,
            user_id: ctx.user_id.to_owned(),
            request_id: None,
        })
        .await?;

    Ok(CreateFeedFormTemplate {
        ctx,
        errors: errors.unwrap_or_default(),
    })
}

#[derive(Template)]
#[template(path = "feed_item.html")]
pub struct FeedItemTemplate {
    ctx: UserContext,
    feed: Edge<UserFeed>,
    end_cursor: Option<CursorType>,
}

impl FeedItemTemplate {
    fn query_tag(&self) -> String {
        "".into()
    }
}

pub async fn feed(
    ctx: UserContext,
    Query(input): Query<starter_feed::GetFeedInput>,
) -> Result<FeedItemTemplate, Response> {
    let feed = ctx.query(input).await?;

    Ok(FeedItemTemplate {
        ctx,
        feed: Edge {
            cursor: feed.to_cursor(),
            node: feed,
        },
        end_cursor: None,
    })
}

#[derive(Clone)]
pub struct IndexFeedHandler;

#[async_trait]
impl RuleHandler for IndexFeedHandler {
    async fn handle(&self, event: Event, ctx: ConsumerContext) -> anyhow::Result<()> {
        let id = Feed::from_aggregate_id(&event.aggregate_id);
        let pikav = ctx.extract::<pikav_client::Client>();
        let config = ctx.extract::<Config>();
        let Some(metadata) = event.to_metadata::<FeedMetadata>()? else {
            return Ok(());
        };

        match event.name.parse()? {
            FeedEvent::Created => {
                let data: Created = event.to_data()?;
                let html = format!(
                    r#"<div hx-get="{}?id={id}" hx-swap="outerHTML" hx-trigger="load"></div>"#,
                    config.create_url("/_feed")
                );

                for tag in data.tags {
                    pikav.publish(vec![SimpleEvent {
                        user_id: metadata.req_user.to_string(),
                        topic: "index".into(),
                        event: format!("created-{tag}"),
                        data: html.to_owned(),
                    }]);
                }

                pikav.publish(vec![SimpleEvent {
                    user_id: metadata.req_user.to_string(),
                    topic: "index".into(),
                    event: "created".into(),
                    data: html,
                }]);
            }
        };

        Ok(())
    }
}
