use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::any,
    Json, Router,
};
use serde_json::json;
use std::{
    cell::Cell, net::SocketAddr, sync::{Arc, RwLock}, time::Duration
};
use tokio::{sync::Notify, time::sleep};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::chess::{ChessBoard, ChessColor, ChessVec};
use crate::cvec;

/// Stores ongoing matches
#[derive(Clone)]
struct AppState {
    chess_game: Arc<(RwLock<ChessBoard>, Notify)>,
    is_game_empty: Arc<RwLock<bool>>,
}

pub async fn start_web_server() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let app_state = AppState {
        chess_game: Arc::new((RwLock::new(ChessBoard::new()), Notify::new())),
        is_game_empty: Arc::new(RwLock::new(true)),
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
    State(AppState { chess_game: pair, is_game_empty }): State<AppState>,
) -> impl IntoResponse {
    // let color = {
    //     let mut board = pair.0.write().unwrap();
    //     let turn = board.get_turn();
    //     board.swap_turn();
    //     turn
    // };
    let mut is_game_empty = is_game_empty.write().unwrap();
    let color = if *is_game_empty {
        *is_game_empty = false;
        ChessColor::WHITE
    } else {
        ChessColor::BLACK
    };
    ws.on_upgrade(move |socket| handle_socket(socket, Arc::clone(&pair), color))
}

async fn handle_socket(
    mut socket: WebSocket,
    pair: Arc<(RwLock<ChessBoard>, Notify)>,
    color: ChessColor,
) {
    println!("{} connectef", ["white", "black"][color as usize]);
    let (lock, notice) = &*pair;

    loop {
        let (fen, turn) = {
            let board = lock.read().unwrap();
            println!("{board}");
            (board.to_fen(), board.get_turn())
        };
        socket
            .send(
                json!({"result": "success", "message": fen})
                    .to_string()
                    .into(),
            )
            .await
            .unwrap();

        if turn != color {
            println!("{} waiting", ["white", "black"][color as usize]);
            notice.notified().await;
        }

        if let Some(Ok(msg)) = socket.recv().await {
            let mut msg = msg.to_text().unwrap().to_owned();

            println!("{msg}");

            let move_result = {
                let mut board = lock.write().unwrap();

                if board.is_piece_selected() {
                    board.move_selected(ChessVec::try_from(&mut msg).unwrap())
                } else {
                    board.select_piece(ChessVec::try_from(&mut msg).unwrap())
                }
            };

            match move_result {
                Ok(_) => {
                    println!("{} took action", ["white", "black"][color as usize]);
                    notice.notify_waiters()
                }
                Err(e) => {
                    println!("{e}");
                    socket
                        .send(json!({"result": "error", "message": e}).to_string().into())
                        .await;
                }
            }
        }
    }
}
