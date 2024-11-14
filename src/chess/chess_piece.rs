use super::*;

pub struct ChessPiece {
    pub piece_type: PieceType,
    pub color: ChessColor,
    pub total_moves: u32,
}

#[derive(Clone, Copy)]
pub enum PieceType {
    KING,
    QUEEN,
    BISHOP,
    KNIGHT,
    ROOK,
    PAWN,
}
pub use PieceType::*;

impl ChessPiece {
    pub const fn new(piece_type: PieceType, color: ChessColor) -> Self {
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
        const WHITE_PIECE_REPR: &str = "KQBNRP";
        const BLACK_PIECE_REPR: &str = "kqbnrp";

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
