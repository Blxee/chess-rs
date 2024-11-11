use std::{
    fmt,
    mem::{self, replace},
    ops::{Add, Index, IndexMut, Sub},
};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

struct ChessBoard {
    grid: [[Option<ChessPiece>; WIDTH]; HEIGHT],
    turn: ChessColor,
    move_stack: Vec<ChessMove>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ChessColor {
    WHITE,
    BLACK,
}
use ChessColor::*;

struct ChessPiece {
    piece_type: PieceType,
    color: ChessColor,
    total_moves: u32,
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

#[derive(Clone, Copy)]
struct ChessVec {
    row: usize,
    col: usize,
}

struct ChessMove {
    from: ChessVec,
    to: ChessVec,
    taken_piece: Option<ChessPiece>,
    move_type: MoveType,
}

enum MoveType {
    Normal,
    EnPassant(ChessVec),
    Casteling(Box<ChessMove>),
    Promoting(PieceType),
}
use MoveType::*;

impl ChessBoard {
    const fn new() -> Self {
        let mut grid = [const { [const { None }; WIDTH] }; HEIGHT];

        grid[7][0] = Some(ChessPiece::new(ROOK, BLACK));
        grid[7][1] = Some(ChessPiece::new(KNIGHT, BLACK));
        grid[7][2] = Some(ChessPiece::new(BISHOP, BLACK));
        grid[7][3] = Some(ChessPiece::new(QUEEN, BLACK));
        grid[7][4] = Some(ChessPiece::new(KING, BLACK));
        grid[7][5] = Some(ChessPiece::new(BISHOP, BLACK));
        grid[7][6] = Some(ChessPiece::new(KNIGHT, BLACK));
        grid[7][7] = Some(ChessPiece::new(ROOK, BLACK));
        grid[6] = [const { Some(ChessPiece::new(PAWN, BLACK)) }; 8];

        grid[1] = [const { Some(ChessPiece::new(PAWN, WHITE)) }; 8];
        grid[0][0] = Some(ChessPiece::new(ROOK, WHITE));
        grid[0][1] = Some(ChessPiece::new(KNIGHT, WHITE));
        grid[0][2] = Some(ChessPiece::new(BISHOP, WHITE));
        grid[0][3] = Some(ChessPiece::new(QUEEN, WHITE));
        grid[0][4] = Some(ChessPiece::new(KING, WHITE));
        grid[0][5] = Some(ChessPiece::new(BISHOP, WHITE));
        grid[0][6] = Some(ChessPiece::new(KNIGHT, WHITE));
        grid[0][7] = Some(ChessPiece::new(ROOK, WHITE));

        Self {
            grid,
            turn: ChessColor::WHITE,
            move_stack: Vec::new(),
        }
    }

    fn move_piece(&mut self, from: ChessVec, to: ChessVec) -> Result<(), &str> {
        match &self[from] {
            Some(piece) if piece.color != self.turn => {
                return Err("[Error]: this is not your piece to move");
            }
            None => return Err("[Error]: there is no piece to move"),
            _ => (),
        }
        let mut piece = self[from].take().unwrap();

        piece.total_moves += 1;

        let taken_piece = mem::replace(&mut self[to], Some(piece));

        self.move_stack.push(ChessMove {
            from,
            to,
            taken_piece,
            move_type: MoveType::Normal,
        });

        self.turn = match self.turn {
            WHITE => BLACK,
            BLACK => WHITE,
        };

        Ok(())
    }

    fn undo_move(&mut self) -> Result<(), &str> {
        let Some(ChessMove {
            from,
            to,
            taken_piece,
            move_type,
        }) = self.move_stack.pop()
        else {
            return Err("[Error]: move stack is empty");
        };

        let mut piece = self[to].take();
        piece.as_mut().map(|piece| piece.total_moves -= 1);
        self[from] = piece;

        match move_type {
            Normal => {
                self[to] = taken_piece;
            },
            EnPassant(target) => todo!(),
            Casteling(rook_move) => todo!(),
            Promoting(to_type) => todo!(),
        }
        Ok(())
    }
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "   a b c d e f g h")?;
        writeln!(f, "  +---------------+")?;

        for (i, row) in self.grid.iter().rev().enumerate() {
            write!(f, "{} ", (HEIGHT - i))?;
            for cell in row {
                write!(
                    f,
                    "|{}",
                    cell.as_ref().map_or(" ".to_string(), ChessPiece::to_string)
                )?;
            }
            writeln!(f, "| {}", (HEIGHT - i))?;
        }

        writeln!(f, "  +---------------+")?;
        write!(f, "   a b c d e f g h")
    }
}

impl Index<ChessVec> for ChessBoard {
    type Output = Option<ChessPiece>;

    fn index(&self, ChessVec { row, col }: ChessVec) -> &Self::Output {
        &self.grid[row][col]
    }
}

impl IndexMut<ChessVec> for ChessBoard {
    fn index_mut(&mut self, ChessVec { row, col }: ChessVec) -> &mut Self::Output {
        &mut self.grid[row][col]
    }
}

impl ChessPiece {
    const fn new(piece_type: PieceType, color: ChessColor) -> Self {
        Self {
            piece_type,
            color,
            total_moves: 0,
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
                WHITE => WHITE_PIECE_REPR.chars().nth(self.piece_type as usize),
                BLACK => BLACK_PIECE_REPR.chars().nth(self.piece_type as usize),
            }
            .unwrap()
        )
    }
}

macro_rules! cvec {
    ($col:expr, $row:expr) => {
        ChessVec::new($col, $row)
    };
}

impl ChessVec {
    const fn new(col: usize, row: usize) -> Self {
        Self { row, col }
    }
}

impl TryFrom<&str> for ChessVec {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        const ROWS: &'static str = "12345678";
        const COLS: &'static str = "abcdefgh";

        let Some(row) = value.chars().find_map(|c| ROWS.find(c)) else {
            return Err("[Warning]: no row number was found");
        };

        let Some(col) = value.chars().find_map(|c| COLS.find(c)) else {
            return Err("[Warning]: no column character was found");
        };

        Ok(Self { row, col })
    }
}

impl Add for ChessVec {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Sub for ChessVec {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut board = ChessBoard::new();

    board.move_piece(cvec!(0, 0), cvec!(3, 3)).unwrap();
    board.undo_move();
    println!("{board}");
}
