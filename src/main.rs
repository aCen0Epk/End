use std::sync::{atomic::{AtomicU64, Ordering}, Arc};

use axum::{
    http::Request,
    routing::{delete, get, post, put}, 
    Router
};
use dotenvy::dotenv;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{RequestId, MakeRequestId},
    trace::{DefaultOnResponse, TraceLayer}, 
    ServiceBuilderExt};

use tracing::{info, info_span, Level};

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
        .layer(ServiceBuilder::new()
        .set_x_request_id(MyMakeRequestId::default())
        .layer(
            TraceLayer::new_for_http()
               .make_span_with(|request: &Request<_>| {
                    let reqid = request
                    .headers()
                    .get("x-request-id")
                    .map(|v| v.to_str().unwrap_or(" "))
                    .unwrap_or(" ");

                info_span!(
                    "request",
                    method = %request.method(),
                    uri = %request.uri(),
                    reqid = ?reqid,
                )
                })
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .propagate_x_request_id(),
        )
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


#[derive(Clone, Default)]

struct MyMakeRequestId {
    counter: Arc<AtomicU64>,
}

impl MakeRequestId for MyMakeRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let request_id = self.counter
            .fetch_add(1, Ordering::SeqCst)
            .to_string()
            .parse()
            .unwrap();

        Some(RequestId::new(request_id))
    }
    
}


