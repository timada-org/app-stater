mod api;
mod app;

pub use api::FeedService;
pub use app::create_router;

pub fn add(a: usize, b: usize) -> usize {
    b + a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
