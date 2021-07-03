//! This module contains all the possible bit patterns
//! for Tic-Tac-Toe. This means patterns representing
//! cells that have been set by a player and patterns for
//! winning the game.

/// Represents the state of a particular cell.
#[derive(Clone, Copy)]
pub(crate) enum Cell {
    TopLeft = 0b100000000,
    TopCentre = 0b010000000,
    TopRight = 0b001000000,
    CentreLeft = 0b000100000,
    Centre = 0b000010000,
    CentreRight = 0b000001000,
    BottomLeft = 0b000000100,
    BottomCentre = 0b000000010,
    BottomRight = 0b000000001,
}

impl Cell {
    pub(crate) fn try_from(x: &str) -> Option<Cell> {
        match x {
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

/// These patterns represent every possible win state.
///
/// Rows, Columns and Diagonals are accounted for in this.
#[derive(Clone, Copy)]
pub(crate) enum WinPattern {
    TopRow = 0b111000000,
    CentreRow = 0b000111000,
    BottomRow = 0b000000111,
    LeftColumn = 0b100100100,
    CentreColumn = 0b010010010,
    RightColumn = 0b001001001,
    LeftDiagonal = 0b100010001,
    RightDiagonal = 0b001010100,
}
