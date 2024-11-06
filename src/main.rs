use std::{fmt, io::stdin};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

struct ChessBoard {
    grid: [[ChessCell; WIDTH]; HEIGHT],
}

enum ChessColor {
    WHITE,
    BLACK,
}
use ChessColor::*;

enum ChessCell {
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
    const fn new() -> Self {
        let mut grid = [const { [const { ChessCell::Empty }; WIDTH] }; HEIGHT];

        grid[7][0] = Filled(ChessPiece::new(ROOK, BLACK));
        grid[7][1] = Filled(ChessPiece::new(KNIGHT, BLACK));
        grid[7][2] = Filled(ChessPiece::new(BISHOP, BLACK));
        grid[7][3] = Filled(ChessPiece::new(QUEEN, BLACK));
        grid[7][4] = Filled(ChessPiece::new(KING, BLACK));
        grid[7][5] = Filled(ChessPiece::new(BISHOP, BLACK));
        grid[7][6] = Filled(ChessPiece::new(KNIGHT, BLACK));
        grid[7][7] = Filled(ChessPiece::new(ROOK, BLACK));
        grid[6] = [const { Filled(ChessPiece::new(PAWN, BLACK)) }; 8];

        grid[1] = [const { Filled(ChessPiece::new(PAWN, WHITE)) }; 8];
        grid[0][0] = Filled(ChessPiece::new(ROOK, WHITE));
        grid[0][1] = Filled(ChessPiece::new(KNIGHT, WHITE));
        grid[0][2] = Filled(ChessPiece::new(BISHOP, WHITE));
        grid[0][3] = Filled(ChessPiece::new(QUEEN, WHITE));
        grid[0][4] = Filled(ChessPiece::new(KING, WHITE));
        grid[0][5] = Filled(ChessPiece::new(BISHOP, WHITE));
        grid[0][6] = Filled(ChessPiece::new(KNIGHT, WHITE));
        grid[0][7] = Filled(ChessPiece::new(ROOK, WHITE));

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
        write!(f, "   a b c d e f g h")
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

impl ChessPiece {
    const fn new(piece_type: PieceType, color: ChessColor) -> Self {
        Self { piece_type, color }
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
                WHITE => WHITE_PIECE_REPR.chars().nth(self.piece_type as usize),
                BLACK => BLACK_PIECE_REPR.chars().nth(self.piece_type as usize),
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
