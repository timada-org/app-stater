cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        mod api;
        mod config;
    }
}

#[tokio::main]
async fn main() {
    use axum::{routing::post, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use starter_app::app::*;
    use starter_app::fileserv::file_and_error_handler;
    use timada_starter_client::feed_server::FeedServer;
    use tonic::transport::Server;

    use crate::api::FeedService;
    use crate::config::StarterConfig;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let starter_config = match StarterConfig::new() {
        Ok(config) => config,
        Err(e) => {
            error!("Config::new -> {}", e.to_string());
            std::process::exit(1);
        }
    };

    let api_addr = starter_config.api_addr;
    tokio::spawn(async move {
        let addr = match api_addr.parse() {
            Ok(addr) => addr,
            Err(e) => {
                error!("{}", e);
                std::process::exit(1);
            }
        };

        let feed_service = FeedService;
        let srv = Server::builder().add_service(FeedServer::new(feed_service));

        log!("api listening on http://{}", addr);

        if let Err(e) = srv.serve(addr).await {
            error!("srv.serve -> {}", e.to_string());
            std::process::exit(1);
        }
    });

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;

    let addr = match starter_config.app_addr.parse() {
        Ok(addr) => addr,
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    };

    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, |cx| view! { cx, <App/> })
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("app listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
