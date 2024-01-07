use askama::Template;
use askama_axum::Response;
use axum::extract::Path;
use i18n_embed_fl::fl;
use starter_feed::{GetFeedInput, UserFeed};

use crate::context::UserContext;

pub struct IndexTemplateHomeLinkFl {
    title: String,
}

pub struct IndexTemplateFl {
    home_link: IndexTemplateHomeLinkFl,
}

#[derive(Template)]
#[template(path = "feed/index.html")]
pub struct IndexTemplate {
    ctx: UserContext,
    feed: UserFeed,
    fl: IndexTemplateFl,
}

pub async fn index(
    ctx: UserContext,
    Path((id,)): Path<(String,)>,
) -> Result<IndexTemplate, Response> {
    let feed = ctx.query(GetFeedInput { id }).await?;

    Ok(IndexTemplate {
        fl: IndexTemplateFl {
            home_link: IndexTemplateHomeLinkFl {
                title: fl!(
                    ctx.fl_loader(),
                    "pages_feed_index-IndexTemplate_HomeLink_title"
                ),
            },
        },
        ctx,
        feed,
    })
}
