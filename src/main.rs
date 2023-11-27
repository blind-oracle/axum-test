use axum::{
    body::Body,
    http::Request,
    middleware::{self, Next},
    response::IntoResponse,
    Router,
};
use axum_server::{accept::DefaultAcceptor, Server};
use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;

pub async fn mw(_r: Request<Body>, _n: Next<Body>) -> impl IntoResponse {
    "bar".repeat(10)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", axum::routing::get(handler)).layer(
        ServiceBuilder::new()
            .layer(CompressionLayer::new())
            .layer(middleware::from_fn(mw)),
    );

    // let app = Router::new()
    //     .route("/", axum::routing::get(handler))
    //     .layer(middleware::from_fn(mw))
    //     .layer(CompressionLayer::new());

    let svc = app.clone().into_make_service();

    Server::bind(SocketAddr::new(
        IpAddr::from_str("127.0.0.1").unwrap(),
        3000,
    ))
    .acceptor(DefaultAcceptor)
    .serve(svc)
    .await
    .unwrap();
}

async fn handler() -> impl IntoResponse {
    "foo".repeat(10)
}
