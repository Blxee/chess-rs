use crate::cvec;
use std::mem;
use std::{
    fmt,
    ops::{Index, IndexMut},
};

mod chess_vec;
mod chess_piece;

pub use chess_vec::*;
use chess_piece::*;

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

pub struct ChessBoard {
    grid: [[Option<ChessPiece>; WIDTH]; HEIGHT],
    turn: ChessColor,
    move_stack: Vec<ChessMove>,
    kings_pos: [ChessVec; 2],
    selected_pos: Option<ChessVec>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChessColor {
    WHITE = 0,
    BLACK = 1,
}
use ChessColor::*;

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
            selected_pos: None,
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
        match &self[pos] {
            Some(piece) if piece.color != self.turn => {
                Err("[Warning]: this is not your piece to select")
            }
            None => Err("[Warning]: there is no piece to select"),
            _ => {
                self.selected_pos = Some(pos);
                Ok(())
            }
        }
    }

    pub fn is_piece_selected(&self) -> bool {
        self.selected_pos.is_some()
    }

    pub fn deselect_piece(&mut self) -> Result<(), &str> {
        if self.selected_pos.is_none() {
            Err("[Warning]: no piece was selected")
        } else {
            self.selected_pos = None;
            Ok(())
        }
    }

    pub fn move_piece(&mut self, from: ChessVec, to: ChessVec) -> Result<(), &str> {
        match &self[from] {
            Some(piece) if piece.color != self.turn => {
                return Err("[Warning]: this is not your piece to move");
            }
            None => return Err("[Warning]: there is no piece to move"),
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
        self.deselect_piece()?;

        Ok(())
    }

    pub fn move_selected(&mut self, to: ChessVec) -> Result<(), &str> {
        let Some(selected_pos) = self.selected_pos else {
            return Err("[Warning]: no piece is selected to move");
        };
        self.move_piece(selected_pos, to)
    }

    pub fn undo_move(&mut self) -> Result<(), &str> {
        let Some(ChessMove {
            from,
            to,
            taken_piece,
            move_type,
        }) = self.move_stack.pop()
        else {
            return Err("[Warning]: move stack is empty");
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

        self.deselect_piece()?;

        Ok(())
    }
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "   a b c d e f g h")?;
        writeln!(f, "  +---------------+")?;

        for (y, row) in self.grid.iter().rev().enumerate() {
            write!(f, "{} ", (HEIGHT - y))?;
            for (x, cell) in row.iter().enumerate() {
                let is_selected = self
                    .selected_pos
                    .map_or(false, |pos| pos == cvec!(x as i32, 7 - y as i32));
                write!(
                    f,
                    "{}{}",
                    if is_selected { '+' } else { '|' },
                    cell.as_ref().map_or(" ".to_string(), ChessPiece::to_string)
                )?;
            }
            writeln!(f, "| {}", (HEIGHT - y))?;
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
