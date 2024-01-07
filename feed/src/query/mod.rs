mod feeds;
mod tags_count;

use evento::Rule;
pub use feeds::*;
use parse_display::{Display, FromStr};
pub use tags_count::*;

#[derive(Display, FromStr)]
#[display(style = "kebab-case")]
pub enum FeedRule {
    TagsCount,
    FeedDetails,
}

impl From<FeedRule> for String {
    fn from(value: FeedRule) -> Self {
        value.to_string()
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule::new(FeedRule::TagsCount).handler("feed/**", TagsCountHandler),
        Rule::new(FeedRule::FeedDetails).handler("feed/**", FeedDetailsHandler),
    ]
}
