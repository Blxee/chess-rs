use axum::{
    extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::any,
    Router,
};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::chess::ChessBoard;
use crate::cvec;

pub async fn start_web_server() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let app = Router::new()
        .route("/ws", any(ws_handler))
        .nest_service("/", ServeDir::new("frontend/dist/"))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(Mutex::new(ChessBoard::new())));

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, State(board): State<Arc<Mutex<ChessBoard>>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, Arc::clone(&board)))
}

async fn handle_socket(mut socket: WebSocket, board: Arc<Mutex<ChessBoard>>) {
    println!("connected");

    loop {
        let mut board = board.lock().await;

        socket.send(board.to_fen().into()).await.unwrap();
        if let Some(Ok(msg)) = socket.recv().await {
            println!("{}", msg.to_text().unwrap());
            board.move_piece(cvec!(0, 0), cvec!(0, 0));
        }
    }
}
