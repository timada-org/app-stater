use serde::Deserialize;
use timada::{starter_client::StarterClient, CreateFeedReply};
use tonic::transport::Channel;

pub use timada::CreateFeedRequest;
pub use tonic::Status;

pub mod timada {
    tonic::include_proto!("timada");
}

#[derive(Clone)]
pub struct Client {
    channel: Channel,
}

impl Client {
    pub fn new<N: Into<String>>(url: N) -> Result<Self, ClientError> {
        let channel = Channel::from_shared(url.into())?.connect_lazy();

        Ok(Self { channel })
    }

    pub async fn create_feed(
        &self,
        message: CreateFeedRequest,
    ) -> Result<tonic::Response<CreateFeedReply>, Status> {
        let mut client = StarterClient::new(self.channel.clone());

        let request = tonic::Request::new(message);

        client.create_feed(request).await
    }
}
