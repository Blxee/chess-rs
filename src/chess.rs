use std::{
    error::Error, fmt, mem::{self, replace}, ops::{Add, Index, IndexMut, Sub}, rc::Rc
};

#[macro_export]
macro_rules! cvec {
    ($col:expr, $row:expr) => {
        ChessVec::new($col, $row)
    };
}

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

pub struct ChessBoard {
    grid: [[Option<ChessPiece>; WIDTH]; HEIGHT],
    turn: ChessColor,
    move_stack: Vec<ChessMove>,
    kings_pos: [ChessVec; 2],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChessColor {
    WHITE = 0,
    BLACK = 1,
}
use ChessColor::*;

pub struct ChessPiece {
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
pub struct ChessVec {
    row: i32,
    col: i32,
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
    pub const fn new() -> Self {
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
            kings_pos: [cvec!(4, 7), cvec!(4, 0)],
        }
    }

    fn get_king(&self, color: ChessColor) -> (&ChessPiece, ChessVec) {
        let king_pos = self.kings_pos[color as usize];
        return (self[king_pos].as_ref().unwrap(), king_pos);
    }

    fn swap_turn(&mut self) {
        self.turn = match self.turn {
            WHITE => BLACK,
            BLACK => WHITE,
        };
    }

    pub fn select_piece(&mut self, pos: ChessVec) -> Result<(), &str> {
        todo!()
    }

    pub fn move_piece(&mut self, from: ChessVec, to: ChessVec) -> Result<(), &str> {
        match &self[from] {
            Some(piece) if piece.color != self.turn => {
                return Err("[Error]: this is not your piece to move");
            }
            None => return Err("[Error]: there is no piece to move"),
            _ => (),
        }
        let mut piece = self[from].take().unwrap();

        piece.total_moves += 1;

        if matches!(piece.piece_type, KING) {
            self.kings_pos[piece.color as usize] = to;
        }

        let taken_piece = mem::replace(&mut self[to], Some(piece));

        self.move_stack.push(ChessMove {
            from,
            to,
            taken_piece,
            move_type: MoveType::Normal,
        });

        self.swap_turn();

        Ok(())
    }

    pub fn undo_move(&mut self) -> Result<(), &str> {
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

        self.swap_turn();

        match move_type {
            Normal => {
                self[to] = taken_piece;
            }
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
        &self.grid[row as usize][col as usize]
    }
}

impl IndexMut<ChessVec> for ChessBoard {
    fn index_mut(&mut self, ChessVec { row, col }: ChessVec) -> &mut Self::Output {
        &mut self.grid[row as usize][col as usize]
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

    fn is_move_valid(&self, board: &ChessBoard, from: ChessVec, to: ChessVec) -> bool {
        match self.piece_type {
            KING => self.is_king_move_valid(board, from, to),
            QUEEN => self.is_queen_move_valid(board, from, to),
            BISHOP => self.is_bishop_move_valid(board, from, to),
            KNIGHT => self.is_knight_move_valid(board, from, to),
            ROOK => self.is_rook_move_valid(board, from, to),
            PAWN => self.is_pawn_move_valid(board, from, to),
        }
    }

    fn is_king_move_valid(&self, board: &ChessBoard, from: ChessVec, to: ChessVec) -> bool {
        let diff = (from - to).abs();
        if diff.row > 1 || diff.col > 1 {
            return false;
        }
        true
    }

    fn is_queen_move_valid(&self, board: &ChessBoard, from: ChessVec, to: ChessVec) -> bool {
        todo!()
    }

    fn is_bishop_move_valid(&self, board: &ChessBoard, from: ChessVec, to: ChessVec) -> bool {
        todo!()
    }

    fn is_knight_move_valid(&self, board: &ChessBoard, from: ChessVec, to: ChessVec) -> bool {
        todo!()
    }

    fn is_rook_move_valid(&self, board: &ChessBoard, from: ChessVec, to: ChessVec) -> bool {
        todo!()
    }

    fn is_pawn_move_valid(&self, board: &ChessBoard, from: ChessVec, to: ChessVec) -> bool {
        todo!()
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

impl ChessVec {
    pub const fn new(col: i32, row: i32) -> Self {
        Self { row, col }
    }

    fn abs(mut self) -> Self {
        self.row = self.row.abs();
        self.col = self.col.abs();
        self
    }
}

impl TryFrom<&mut String> for ChessVec {
    type Error = &'static str;

    fn try_from(value: &mut String) -> Result<Self, Self::Error> {
        const ROWS: &'static str = "12345678";
        const COLS: &'static str = "abcdefgh";

        let Some(row_idx) = value.chars().position(|c| ROWS.contains(c)) else {
            return Err("[Warning]: no row number was found");
        };
        let row = ROWS.find(value.remove(row_idx)).unwrap();

        let Some(col_idx) = value.chars().position(|c| COLS.contains(c)) else {
            return Err("[Warning]: no column character was found");
        };
        let col = COLS.find(value.remove(col_idx)).unwrap();

        Ok(Self::new(col.try_into().unwrap(), row.try_into().unwrap()))
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
