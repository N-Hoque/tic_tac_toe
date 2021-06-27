//! This module contains all the possible bit patterns
//! for Tic-Tac-Toe. This means patterns representing
//! cells that have been set by a player, the player-specific
//! pattern, the general pattern for the board, and patterns for
//! winning the game.

/// These patterns represent which cells have been set by a player.
#[derive(Clone, Copy)]
pub(crate) enum ActiveCell {
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

impl From<PlayerCell> for ActiveCell {
    fn from(x: PlayerCell) -> Self {
        match x {
            PlayerCell::TopLeftO | PlayerCell::TopLeftX => ActiveCell::TopLeft,
            PlayerCell::TopCentreO | PlayerCell::TopCentreX => ActiveCell::TopCentre,
            PlayerCell::TopRightO | PlayerCell::TopRightX => ActiveCell::TopRight,
            PlayerCell::CentreLeftO | PlayerCell::CentreLeftX => ActiveCell::CentreLeft,
            PlayerCell::CentreO | PlayerCell::CentreX => ActiveCell::Centre,
            PlayerCell::CentreRightO | PlayerCell::CentreRightX => ActiveCell::CentreRight,
            PlayerCell::BottomLeftO | PlayerCell::BottomLeftX => ActiveCell::BottomLeft,
            PlayerCell::BottomCentreO | PlayerCell::BottomCentreX => ActiveCell::BottomCentre,
            PlayerCell::BottomRightO | PlayerCell::BottomRightX => ActiveCell::BottomRight,
        }
    }
}

/// These patterns represent all the cells on the board.
/// Note that this is not the same as the active cell set.
///
/// There's actually three states that a cell can be in:
///
/// - O - Player O set this cell
/// - X - Player X set this cell
/// - E - No one has set this cell
///
/// To represent these states correctly, the bit pattern is twice the length of the
/// active cell patterns.
#[derive(Clone, Copy)]
pub(crate) enum BoardCell {
    TopLeft = 0b110000000000000000,
    TopCentre = 0b001100000000000000,
    TopRight = 0b000011000000000000,
    CentreLeft = 0b000000110000000000,
    Centre = 0b000000001100000000,
    CentreRight = 0b000000000011000000,
    BottomLeft = 0b000000000000110000,
    BottomCentre = 0b000000000000001100,
    BottomRight = 0b000000000000000011,
}

impl From<ActiveCell> for BoardCell {
    fn from(x: ActiveCell) -> BoardCell {
        match x {
            ActiveCell::TopLeft => BoardCell::TopLeft,
            ActiveCell::TopCentre => BoardCell::TopCentre,
            ActiveCell::TopRight => BoardCell::TopRight,
            ActiveCell::CentreLeft => BoardCell::CentreLeft,
            ActiveCell::Centre => BoardCell::Centre,
            ActiveCell::CentreRight => BoardCell::CentreRight,
            ActiveCell::BottomLeft => BoardCell::BottomLeft,
            ActiveCell::BottomCentre => BoardCell::BottomCentre,
            ActiveCell::BottomRight => BoardCell::BottomRight,
        }
    }
}

/// These patterns represent the case of flipping a bit on the board cell.
///
/// In theory these patterns aren't needed as these could be computed by
/// whoever the active player is.
#[derive(Clone, Copy)]
pub(crate) enum PlayerCell {
    TopLeftX = 0b010000000000000000,
    TopLeftO = 0b100000000000000000,
    TopCentreX = 0b000100000000000000,
    TopCentreO = 0b001000000000000000,
    TopRightX = 0b000001000000000000,
    TopRightO = 0b000010000000000000,
    CentreLeftX = 0b000000010000000000,
    CentreLeftO = 0b000000100000000000,
    CentreX = 0b000000000100000000,
    CentreO = 0b000000001000000000,
    CentreRightX = 0b000000000001000000,
    CentreRightO = 0b000000000010000000,
    BottomLeftX = 0b000000000000010000,
    BottomLeftO = 0b000000000000100000,
    BottomCentreX = 0b000000000000000100,
    BottomCentreO = 0b000000000000001000,
    BottomRightX = 0b000000000000000001,
    BottomRightO = 0b000000000000000010,
}

/// These patterns represent every possible win state.
///
/// Rows, Columns and Diagonals are accounted for in this.
#[derive(Clone, Copy)]
pub(crate) enum WinPattern {
    TopRow = 0b010101000000000000,
    CentreRow = 0b000000010101000000,
    BottomRow = 0b000000000000010101,
    LeftColumn = 0b010000010000010000,
    CentreColumn = 0b000100000100000100,
    RightColumn = 0b000001000001000001,
    LeftDiagonal = 0b010000000100000001,
    RightDiagonal = 0b000001000100010000,
}
