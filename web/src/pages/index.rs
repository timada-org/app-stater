use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

pub async fn index() -> Result<IndexTemplate, crate::pages::Error> {
    Ok(IndexTemplate)
}
