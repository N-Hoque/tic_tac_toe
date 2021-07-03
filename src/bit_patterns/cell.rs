//! Helper module for [Cell]

use std::ops::{BitAnd, BitOrAssign, Shr};

use super::Cell;

impl Cell {
    /// Attempts to convert the input to a [Cell]. Returns [None] if the input is not a valid digit.
    pub(crate) fn try_from(x: &str) -> Option<Cell> {
        match x.into() {
            "1" => Some(Cell::BottomLeft),
            "2" => Some(Cell::BottomCentre),
            "3" => Some(Cell::BottomRight),
            "4" => Some(Cell::CentreLeft),
            "5" => Some(Cell::Centre),
            "6" => Some(Cell::CentreRight),
            "7" => Some(Cell::TopLeft),
            "8" => Some(Cell::TopCentre),
            "9" => Some(Cell::TopRight),
            _ => None,
        }
    }
}

impl BitOrAssign<Cell> for u32 {
    fn bitor_assign(&mut self, rhs: Cell) {
        *self |= rhs as u32
    }
}

impl Shr<u32> for Cell {
    type Output = u32;

    fn shr(self, rhs: u32) -> Self::Output {
        self as u32 >> rhs
    }
}

impl BitAnd<Cell> for u32 {
    type Output = u32;

    fn bitand(self, rhs: Cell) -> Self::Output {
        self & rhs as u32
    }
}
