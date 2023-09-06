mod config;

use anyhow::Result;
use timada_starter_client::feed_server::FeedServer;
use tonic::transport::Server;
use tracing::info;

use crate::config::StarterConfig;
use starter_feed::FeedService;

pub async fn serve() -> Result<()> {
    let config = StarterConfig::new()?;
    let addr = config.api.addr.parse()?;

    let feed_service = FeedService;
    let srv = Server::builder().add_service(FeedServer::new(feed_service));

    info!("api listening on http://{}", addr);

    srv.serve(addr).await?;

    Ok(())
}
