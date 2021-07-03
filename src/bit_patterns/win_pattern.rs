//! Helper module for [WinPattern]

use std::fmt::{Binary, Display};
use std::ops::{BitAnd, BitOr, BitXor, Shr};

use super::{Cell, WinPattern};

impl From<WinPattern> for u32 {
    /// Converts a [WinPattern] into a [u32]
    fn from(x: WinPattern) -> Self {
        x as u32
    }
}

impl From<WinPattern> for [Cell; 3] {
    /// Converts a [WinPattern] into a triple of [Cell]s `[Cell; 3]`
    fn from(val: WinPattern) -> Self {
        match val {
            WinPattern::TopRow => [Cell::TopLeft, Cell::TopCentre, Cell::TopRight],
            WinPattern::CentreRow => [Cell::CentreLeft, Cell::Centre, Cell::CentreRight],
            WinPattern::BottomRow => [Cell::BottomLeft, Cell::BottomCentre, Cell::BottomRight],
            WinPattern::LeftColumn => [Cell::TopLeft, Cell::CentreLeft, Cell::BottomLeft],
            WinPattern::CentreColumn => [Cell::TopCentre, Cell::Centre, Cell::BottomCentre],
            WinPattern::RightColumn => [Cell::TopRight, Cell::CentreRight, Cell::BottomRight],
            WinPattern::LeftDiagonal => [Cell::TopLeft, Cell::Centre, Cell::BottomRight],
            WinPattern::RightDiagonal => [Cell::TopRight, Cell::Centre, Cell::BottomLeft],
        }
    }
}

impl Display for WinPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>32b}", *self as u32)
    }
}

impl Binary for WinPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Binary::fmt(&u32::from(*self), f)
    }
}

impl Shr<u32> for WinPattern {
    type Output = u32;

    fn shr(self, rhs: u32) -> Self::Output {
        self as u32 >> rhs
    }
}

impl BitOr<WinPattern> for u32 {
    type Output = u32;

    fn bitor(self, rhs: WinPattern) -> Self::Output {
        self | rhs as u32
    }
}

impl BitOr<u32> for WinPattern {
    type Output = u32;

    fn bitor(self, rhs: u32) -> Self::Output {
        self as u32 | rhs
    }
}

impl BitXor<WinPattern> for u32 {
    type Output = u32;

    fn bitxor(self, rhs: WinPattern) -> Self::Output {
        self ^ rhs as u32
    }
}

impl BitXor<u32> for WinPattern {
    type Output = u32;

    fn bitxor(self, rhs: u32) -> Self::Output {
        self as u32 ^ rhs
    }
}

impl BitAnd<WinPattern> for u32 {
    type Output = u32;

    fn bitand(self, rhs: WinPattern) -> Self::Output {
        self & rhs as u32
    }
}

impl BitAnd<u32> for WinPattern {
    type Output = u32;

    fn bitand(self, rhs: u32) -> Self::Output {
        self as u32 & rhs
    }
}
