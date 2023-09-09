mod config;
mod feed;

use anyhow::Result;
use timada_starter_feed::feed_server::FeedServer;
use tonic::transport::Server;
use tracing::info;

use crate::{config::Config, feed::FeedService};

pub async fn serve() -> Result<()> {
    let config = Config::new()?;
    let addr = config.api.addr.parse()?;

    let feed_service = FeedService;
    let srv = Server::builder().add_service(FeedServer::new(feed_service));

    info!("api listening on http://{}", addr);

    srv.serve(addr).await?;

    Ok(())
}
