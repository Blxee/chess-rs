use std::{array, fmt};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

struct ChessBoard {
    grid: [[ChessCell; WIDTH]; HEIGHT],
}

enum ChessColor {
    White,
    Black,
}
use ChessColor::*;

#[derive(Default)]
enum ChessCell {
    #[default]
    Empty,
    Filled(ChessPiece),
}
use ChessCell::*;

struct ChessPiece {
    piece_type: PieceType,
    color: ChessColor,
}

#[derive(Clone, Copy)]
enum PieceType {
    KING,
    QUEEN,
    BISHOP,
    KNIGHT,
    ROOK,
    PAWN,
}
use PieceType::*;

impl ChessBoard {
    fn new() -> Self {
        let mut grid = [const { [const { ChessCell::Empty }; WIDTH] }; HEIGHT];
        grid[6] = [const {
            Filled(ChessPiece {
                piece_type: PAWN,
                color: Black,
            })
        }; 8];
        grid[1] = [const {
            Filled(ChessPiece {
                piece_type: PAWN,
                color: White,
            })
        }; 8];
        Self { grid }
    }
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "   a b c d e f g h")?;
        writeln!(f, "  +---------------+")?;
        for (i, row) in self.grid.iter().rev().enumerate() {
            write!(f, "{} ", (HEIGHT - i))?;
            for cell in row {
                write!(f, "|{cell}")?;
            }
            writeln!(f, "| {}", (HEIGHT - i))?;
        }
        writeln!(f, "  +---------------+")?;
        writeln!(f, "   a b c d e f g h")
    }
}

impl fmt::Display for ChessCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, " "),
            Self::Filled(piece) => write!(f, "{piece}"),
        }
    }
}

impl fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const WHITE_PIECE_REPR: &str = "KQBHRP";
        const BLACK_PIECE_REPR: &str = "kqbhrp";
        write!(
            f,
            "{}",
            match self.color {
                White => WHITE_PIECE_REPR.chars().nth(self.piece_type as usize),
                Black => BLACK_PIECE_REPR.chars().nth(self.piece_type as usize),
            }
            .unwrap()
        )
    }
}

fn main() {
    println!("Hello, world!");

    let board = ChessBoard::new();

    println!("{board}");
}
