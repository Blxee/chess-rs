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
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::{
    sync::{Notify, RwLock},
    time::sleep,
};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::chess::{ChessBoard, ChessColor, ChessVec};
use crate::cvec;

/// Stores ongoing matches
#[derive(Clone)]
struct AppState {
    games: Vec<Arc<ChessGame>>,
}

/// 
struct ChessGame {
    state: Cell<GameState>,
    board: RwLock<ChessBoard>,
    notify: Notify,
}

/// Represets the current game state
///
/// **Starting**: when one player is waiting for another opponent.
/// **Ongoing**: the game is currently being played.
/// **Halted**: game has started but both players are not connected.
/// **Finished**: the game has already finished.
///
enum GameState {
    Starting,
    Ongoing,
    Halted,
    Finished,
}

pub async fn start_web_server() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let app_state = AppState { games: Vec::new() };

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
    State(AppState { games }): State<AppState>,
) -> impl IntoResponse {
    let color = {
        let mut board = pair.0.write().unwrap();
        board.swap_turn();
        board.get_turn()
    };

    for game in games.iter_mut() {
    }
    ws.on_upgrade(move |socket| handle_socket(socket, Arc::clone(&pair), color))
}

async fn handle_socket(
    mut socket: WebSocket,
    game,
    color: ChessColor,
) {
    println!("connected");
    let (lock, notice) = &*pair;

    loop {
        let (fen, turn) = {
            let board = lock.read().unwrap();
            (board.to_fen(), board.get_turn())
        };
        socket.send(fen.into()).await.unwrap();

        if turn != color {
            notice.notified().await;
        }
        if let Some(Ok(msg)) = socket.recv().await {
            let mut msg = msg.to_text().unwrap().to_owned();

            println!("{msg}");

            let mut board = lock.write().unwrap();

            if board.is_piece_selected() {
                board
                    .move_selected(ChessVec::try_from(&mut msg).unwrap())
                    .map_err(|e| println!("{e}"));
            } else {
                board
                    .select_piece(ChessVec::try_from(&mut msg).unwrap())
                    .map_err(|e| println!("{e}"));
            }

            if turn != color {
                notice.notify_one();
            }
        }
    }
}
