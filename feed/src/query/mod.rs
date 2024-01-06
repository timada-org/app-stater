mod feeds;
mod tags_count;

use evento::Rule;
pub use feeds::*;
use parse_display::{Display, FromStr};
pub use tags_count::*;

#[derive(Display, FromStr)]
#[display(style = "kebab-case")]
pub enum ProductRule {
    ProductDetails,
    ProductTask,
}

impl From<ProductRule> for String {
    fn from(value: ProductRule) -> Self {
        value.to_string()
    }
}
pub fn rules() -> Vec<Rule> {
    vec![
        Rule::new(ProductRule::ProductDetails).handler("feed/**", TagsCountHandler),
        Rule::new(ProductRule::ProductTask).handler("feed/**", FeedDetailsHandler),
    ]
}
