mod assets;
mod config;
mod context;
mod i18n;
mod pages;

use anyhow::Result;
use axum::{routing::get, Extension, Router};
use config::Config;
use context::Context;
use evento::PgConsumer;
use evento_axum::{AcceptLanguageSource, QuerySource, UserLanguage};
#[cfg(debug_assertions)]
use pikav_client::timada::SimpleEvent;
use sqlx::PgPool;
use tracing::info;
use twa_jwks::JwksClient;

use crate::assets::static_handler;

pub async fn serve() -> Result<()> {
    let config = Config::new()?;

    let jwks = JwksClient::build(config.jwks_url.to_owned()).await?;
    let db = PgPool::connect(&config.dsn).await?;
    let pikva_client = pikav_client::Client::new(pikav_client::ClientOptions {
        url: config.pikav.url.to_owned(),
        namespace: config.pikav.namespace.to_owned(),
    })?;

    sqlx::migrate!("../migrations")
        .set_locking(false)
        .run(&db)
        .await?;

    let producer = PgConsumer::new(&db)
        .name(&config.region)
        .data(pikva_client.clone())
        .data(config.clone())
        .rules(starter_feed::rules())
        .rules(pages::rules())
        .start(config.evento_delay.unwrap_or(30))
        .await?;

    let command = evento::Command::new(&producer);
    let query = evento::Query::new().data(db.clone());

    let router = pages::create_router();

    let app = match config.base_url.as_ref() {
        Some(base_url) => Router::new().nest(base_url, router),
        _ => router,
    }
    .fallback(get(static_handler))
    .layer(Extension(
        UserLanguage::config()
            .add_source(QuerySource::new("lang"))
            .add_source(AcceptLanguageSource)
            .build(),
    ))
    .layer(Extension(jwks))
    .layer(Extension(Context {
        command,
        query,
        config: config.clone(),
        user_language: None,
        fl_loader: None,
        user_id: None,
    }));

    #[cfg(debug_assertions)]
    pikva_client.publish(vec![SimpleEvent {
        user_id: "*".into(),
        topic: "sys".into(),
        event: "hot-reload".into(),
        data: "App was updated".into(),
    }]);

    info!("app listening on http://{}", &config.addr);

    let listener = tokio::net::TcpListener::bind(config.addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
