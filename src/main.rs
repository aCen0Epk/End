use axum::{
    http::StatusCode, 
    routing::{delete, get, post, put}, 
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
        .route("/api/wx_counter/counters", post(api::counter::add))
        .route("/api/wx_counter/counters/:id", get(api::counter::show))
        .route("/api/wx_counter/counters/:id", put(api::counter::update))
        .route("/api/wx_counter/counters/:id", delete(api::counter::destroy))
        .route("/api/wx_counter/counters/:id/top", post(api::counter::top))
        .route("/api/wx_counter/counter_records", post(api::counter_record::add),)
        .route("/api/wx_counter/counter_records/:counter_id", get(api::counter_record::list),)
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}



