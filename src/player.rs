//! Provides functionality for the [Player] enum

use crate::Player;

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::O => write!(f, "Player O"),
            Player::X => write!(f, "Player X"),
        }
    }
}

impl Player {
    /// Swaps the active player
    pub(crate) fn swap(&mut self) {
        *self = match self {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }
}
