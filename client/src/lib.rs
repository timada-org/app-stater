pub use tonic::Status;

tonic::include_proto!("starter");

cfg_if::cfg_if! {
    if #[cfg(feature = "client")] {

        use feed_client::FeedClient;
        use tonic::transport::Channel;
        use http::uri::InvalidUri;

        #[derive(Clone)]
        pub struct Client {
            channel: Channel,
        }

        impl Client {
            pub fn new<N: Into<String>>(url: N) -> Result<Self, InvalidUri> {
                let channel = Channel::from_shared(url.into())?.connect_lazy();

                Ok(Self { channel })
            }

            pub async fn create_feed(
                &self,
                message: CreateFeedRequest,
                ) -> Result<tonic::Response<CreateFeedResponse>, Status> {
                let mut client = FeedClient::new(self.channel.clone());

                let request = tonic::Request::new(message);

                client.create(request).await
            }
        }
    }
}
