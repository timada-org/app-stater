use askama::Template;
use i18n_embed_fl::fl;

use crate::context::Context;

pub struct NotFoundPageHomeLinkFl {
    title: String,
}

pub struct NotFoundPageFl {
    title: String,
    content: String,
    home_link: NotFoundPageHomeLinkFl,
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundPage {
    ctx: Context,
    fl: NotFoundPageFl,
}

impl NotFoundPage {
    pub fn new(ctx: Context) -> Self {
        Self {
            fl: NotFoundPageFl {
                title: fl!(ctx.fl_loader(), "pages_error-NotFoundPage_title"),
                content: fl!(ctx.fl_loader(), "pages_error-NotFoundPage_content"),
                home_link: NotFoundPageHomeLinkFl {
                    title: fl!(ctx.fl_loader(), "pages_error-NotFoundPage_HomeLink_title"),
                },
            },
            ctx,
        }
    }
}

pub struct InternalServerErrorPageHomeLinkFl {
    title: String,
}

pub struct InternalServerErrorPageFl {
    title: String,
    content: String,
    home_link: InternalServerErrorPageHomeLinkFl,
}

#[derive(Template)]
#[template(path = "500.html")]
pub struct InternalServerErrorPage {
    ctx: Context,
    fl: InternalServerErrorPageFl,
}

impl InternalServerErrorPage {
    pub fn new(ctx: Context) -> Self {
        Self {
            fl: InternalServerErrorPageFl {
                title: fl!(ctx.fl_loader(), "pages_error-InternalServerErrorPage_title"),
                content: fl!(ctx.fl_loader(), "pages_error-InternalServerErrorPage_content"),
                home_link: InternalServerErrorPageHomeLinkFl {
                    title: fl!(ctx.fl_loader(), "pages_error-InternalServerErrorPage_HomeLink_title"),
                },
            },
            ctx,
        }
    }
}
