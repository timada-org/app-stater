mod feeds;
mod tags_count;

pub use feeds::*;
pub use tags_count::*;

use sqlx::PgPool;

#[derive(Clone)]
pub struct FeedQuery {
    pub db: PgPool,
    pub user_id: String,
}
