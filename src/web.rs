use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::any,
    Router,
};
use serde_json::json;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::chess::{ChessBoard, ChessVec};
use crate::cvec;

/// Stores ongoing matches
#[derive(Clone)]
struct AppState {
    chess_game: Arc<Mutex<ChessBoard>>,
}

pub async fn start_web_server() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let app_state = AppState {
        chess_game: Arc::new(Mutex::new(ChessBoard::new())),
    };

    let app = Router::new()
        .route("/ws", any(ws_handler))
        .nest_service("/", ServeDir::new("frontend/dist/"))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(AppState { chess_game }): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, Arc::clone(&chess_game)))
}

async fn handle_socket(mut socket: WebSocket, board: Arc<Mutex<ChessBoard>>) {
    println!("connected");

    loop {
        let mut board = board.lock().await;

        socket.send(board.to_fen().into()).await.unwrap();
        if let Some(Ok(msg)) = socket.recv().await {
            let mut msg = msg.to_text().unwrap().to_owned();

            println!("{msg}");

            if board.is_piece_selected() {
                board.move_selected(ChessVec::try_from(&mut msg).unwrap()).map_err(|e| println!("{e}"));
            } else {
                board.select_piece(ChessVec::try_from(&mut msg).unwrap()).map_err(|e| println!("{e}"));
            }
        }
    }
}
