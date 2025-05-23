mod chess;
mod cli;
mod web;

#[tokio::main]
async fn main() {
    // web::start_web_server().await;
    cli::start_cli_game();
}
