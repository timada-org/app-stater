#[cfg(feature = "feed")]
mod feed;

use http::uri::InvalidUri;
use tonic::transport::Channel;

#[cfg(feature = "feed")]
pub use feed::*;
pub use tonic::Status;

#[derive(Clone)]
pub struct Client {
    #[allow(dead_code)]
    channel: Channel,
}

impl Client {
    pub fn new<N: Into<String>>(url: N) -> Result<Self, InvalidUri> {
        let channel = Channel::from_shared(url.into())?.connect_lazy();

        Ok(Self { channel })
    }
}
