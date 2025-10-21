use axum::{
    http::StatusCode, 
    routing::{get, post}, 
    Error, Json, Router
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::info;

mod db;
mod api;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // initialize tracing
    tracing_subscriber::fmt::init();

    let pool = db::establish_connection().await;

    // build our application with a route
    let app = Router::new()
        .route("/api/wx_counter/login", post(api::user::login))
        .route("/api/wx_counter/counters", get(api::counter::list))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}



