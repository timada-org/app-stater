mod feeds;

pub use feeds::*;

use sqlx::PgPool;

#[derive(Clone)]
pub struct FeedQuery {
    pub db: PgPool,
    pub user_id: String,
}
