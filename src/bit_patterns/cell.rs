//! Helper module for [Cell]

use std::{
    convert::TryFrom,
    ops::{BitAnd, BitOrAssign, Shr},
};

use super::Cell;

impl TryFrom<&str> for Cell {
    type Error = String;

    /// Attempts to convert the input to a [Cell]. Returns [None] if the input is not a valid digit.
    fn try_from(x: &str) -> Result<Self, Self::Error> {
        match x {
            "1" => Ok(Self::BottomLeft),
            "2" => Ok(Self::BottomCentre),
            "3" => Ok(Self::BottomRight),
            "4" => Ok(Self::CentreLeft),
            "5" => Ok(Self::Centre),
            "6" => Ok(Self::CentreRight),
            "7" => Ok(Self::TopLeft),
            "8" => Ok(Self::TopCentre),
            "9" => Ok(Self::TopRight),
            _ => Err(format!("{} is not a valid cell number", x)),
        }
    }
}

impl BitOrAssign<Cell> for u32 {
    fn bitor_assign(&mut self, rhs: Cell) {
        *self |= rhs as Self;
    }
}

impl Shr<u32> for Cell {
    type Output = u32;

    fn shr(self, rhs: u32) -> Self::Output {
        self as u32 >> rhs
    }
}

impl BitAnd<Cell> for u32 {
    type Output = Self;

    fn bitand(self, rhs: Cell) -> Self::Output {
        self & rhs as Self
    }
}
