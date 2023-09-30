use tonic::Status;

use crate::{feed_client::FeedClient, Client, CreateFeedRequest, CreateFeedResponse};

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
