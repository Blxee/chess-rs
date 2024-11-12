mod chess;
use std::{
    io::{stdin, stdout, Stdin, Write},
    process::exit,
};

use crate::chess::{ChessBoard, ChessVec};

enum ChessInput {
    Move(ChessVec, ChessVec),
    Select(ChessVec),
    Undo,
}

fn get_input() -> ChessInput {
    loop {
        let mut buf = String::new();
        print!("[In]: ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut buf)
            .expect("[Error]: failed to read from stdin");

        let from = match ChessVec::try_from(&mut buf) {
            Ok(from) => from,
            Err(_) => {
                if buf.contains(['u', 'U']) {
                    return ChessInput::Undo;
                } else if buf.contains(['q', 'Q']) {
                    println!("[Info]: quitting now..");
                    exit(0);
                } else {
                    println!("[Error]: no position specified");
                    continue;
                }
            }
        };
        return match ChessVec::try_from(&mut buf) {
            Ok(to) => ChessInput::Move(from, to),
            Err(_) => ChessInput::Select(from),
        };
    }
}

fn main() {
    println!("Hello, world!");

    let mut board = ChessBoard::new();

    loop {
        println!("{board}");
        while let Err(e) = match get_input() {
            ChessInput::Move(from, to) => board.move_piece(from, to),
            ChessInput::Select(pos) => board.select_piece(pos),
            ChessInput::Undo => board.undo_move(),
        } {
            println!("{e}");
        }
    }
}
