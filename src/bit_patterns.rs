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
pub enum Cell {
    TopLeft = 0b10_0000_0000_0000_0000,
    TopCentre = 0b00_1000_0000_0000_0000,
    TopRight = 0b00_0010_0000_0000_0000,
    CentreLeft = 0b00_0000_1000_0000_0000,
    Centre = 0b00_0000_0010_0000_0000,
    CentreRight = 0b00_0000_0000_1000_0000,
    BottomLeft = 0b00_0000_0000_0010_0000,
    BottomCentre = 0b00_0000_0000_0000_1000,
    BottomRight = 0b00_0000_0000_0000_0010,
}

/// These patterns represent every possible win state.
///
/// Rows, Columns and Diagonals are accounted for in this.
#[derive(Clone, Copy)]
pub enum WinPattern {
    TopRow = 0b10_1010_0000_0000_0000,
    CentreRow = 0b00_0000_1010_1000_0000,
    BottomRow = 0b00_0000_0000_0010_1010,
    LeftColumn = 0b10_0000_1000_0010_0000,
    CentreColumn = 0b00_1000_0010_0000_1000,
    RightColumn = 0b00_0010_0000_1000_0010,
    LeftDiagonal = 0b10_0000_0010_0000_0010,
    RightDiagonal = 0b00_0010_0010_0010_0000,
}
