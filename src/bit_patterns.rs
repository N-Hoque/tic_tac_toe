//! This module contains all the possible bit patterns
//! for Tic-Tac-Toe. This means patterns representing
//! cells that have been set by a player and patterns for
//! winning the game.
//!
//! The patterns themselves are 18-bits long, containing 9 pairs of bits.
//!
//! Each pair (x, y) represents the following:
//! - x: The bit that states if the cell has been set
//! - y: The bit that states which player set the cell
//! Because of this, if x = 0, y != 1. But if x == 1, y = (0, 1).

mod cell;
mod win_pattern;

/// Represents the state of a particular cell.
#[derive(Clone, Copy)]
pub(crate) enum Cell {
    TopLeft = 0b100000000000000000,
    TopCentre = 0b001000000000000000,
    TopRight = 0b000010000000000000,
    CentreLeft = 0b000000100000000000,
    Centre = 0b000000001000000000,
    CentreRight = 0b000000000010000000,
    BottomLeft = 0b000000000000100000,
    BottomCentre = 0b000000000000001000,
    BottomRight = 0b000000000000000010,
}

/// These patterns represent every possible win state.
///
/// Rows, Columns and Diagonals are accounted for in this.
#[derive(Clone, Copy)]
pub(crate) enum WinPattern {
    TopRow = 0b101010000000000000,
    CentreRow = 0b000000101010000000,
    BottomRow = 0b000000000000101010,
    LeftColumn = 0b100000100000100000,
    CentreColumn = 0b001000001000001000,
    RightColumn = 0b000010000010000010,
    LeftDiagonal = 0b100000001000000010,
    RightDiagonal = 0b000010001000100000,
}
