use crate::Client;
use starter_feed::{feed_client::FeedClient, CreateFeedRequest, CreateFeedResponse};
use tonic::Status;

impl Client {
    pub async fn create_feed(
        &self,
        message: CreateFeedRequest,
    ) -> Result<tonic::Response<CreateFeedResponse>, Status> {
        let mut client = FeedClient::new(self.channel.clone());

        let request = tonic::Request::new(message);

        client.create(request).await
    }
}
