use axum::Router;
use std::net::SocketAddr;
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
};

pub async fn start_web_server() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let app = Router::new().nest_service("/", ServeDir::new("frontend/dist/"));
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}
