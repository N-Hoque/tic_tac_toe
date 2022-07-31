//! Provides functionality for the [Player] enum

use crate::Player;

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::O => write!(f, "Player O"),
            Self::X => write!(f, "Player X"),
        }
    }
}

impl Player {
    /// Swaps the active player
    pub(crate) fn swap(&mut self) {
        *self = match self {
            Self::O => Self::X,
            Self::X => Self::O,
        }
    }
}
