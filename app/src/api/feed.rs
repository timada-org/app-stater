use timada_starter_client::{
    feed_server::Feed, CreateFeedRequest, CreateFeedResponse, ListFeedsRequest, ListFeedsResponse,
};
use tonic::{Request, Response, Status};
use starter_feed::add;

#[derive(Default)]
pub struct FeedService;

#[tonic::async_trait]
impl Feed for FeedService {
    async fn create(
        &self,
        _request: Request<CreateFeedRequest>,
    ) -> Result<Response<CreateFeedResponse>, Status> {
        let e = add(1, 2);
        println!("{}", e);
        Ok(Response::new(CreateFeedResponse { success: true }))
    }

    async fn list(
        &self,
        _request: Request<ListFeedsRequest>,
    ) -> Result<Response<ListFeedsResponse>, Status> {
        Ok(Response::new(ListFeedsResponse { success: true }))
    }
}
