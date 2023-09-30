use timada_starter_client::{
    feed_server::Feed, CreateFeedRequest, CreateFeedResponse, ListFeedsRequest, ListFeedsResponse,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct FeedService;

#[tonic::async_trait]
impl Feed for FeedService {
    async fn create(
        &self,
        _request: Request<CreateFeedRequest>,
    ) -> Result<Response<CreateFeedResponse>, Status> {
        println!("FeedService.create");
        Ok(Response::new(CreateFeedResponse { success: true }))
    }

    async fn list(
        &self,
        _request: Request<ListFeedsRequest>,
    ) -> Result<Response<ListFeedsResponse>, Status> {
        println!("FeedService.list");
        Ok(Response::new(ListFeedsResponse { success: true }))
    }
}
