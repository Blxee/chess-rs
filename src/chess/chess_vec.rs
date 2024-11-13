use std::ops::{Add, Sub};


#[macro_export]
macro_rules! cvec {
    ($col:expr, $row:expr) => {
        ChessVec::new($col, $row)
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChessVec {
    pub row: i32,
    pub col: i32,
}

impl ChessVec {
    pub const fn new(col: i32, row: i32) -> Self {
        Self { row, col }
    }

    pub fn abs(mut self) -> Self {
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
