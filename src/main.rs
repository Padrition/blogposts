mod db;
mod schema;
mod models;
mod handlers;

use axum::Router;
use db::run_migrations;
use handlers::blog_post_routers;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = db::setup_connection_pool();

    run_migrations(&pool).await;

    let app = Router::new()
        .merge(blog_post_routers(pool));
    

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("->> SERVER STARTED\n");
    axum::serve(listener, app).await.unwrap();
}