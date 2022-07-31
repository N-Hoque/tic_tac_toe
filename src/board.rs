//! Provides functionality for the [Board] struct

use std::convert::TryFrom;

use crate::bit_patterns::{Cell, WinPattern};
use crate::{Board, Player};

mod board_display;

// Private block
impl Board {
    #[allow(non_snake_case)]
    fn has_player_set_cell(&self, player: Player, cell: Cell) -> bool {
        self.cells & cell != 0
            && if let Player::O = player {
                self.cells & cell >> 1 == 0
            } else {
                self.cells & cell >> 1 != 0
            }
    }

    /// Checks if a given cell has been set.
    fn is_cell_set(&self, cell: Cell) -> bool {
        self.cells & cell != 0
    }

    /// Sets the cell for a given player.
    fn set_cell(&mut self, player: Player, cell: Cell) {
        self.cells |= cell;
        if let Player::X = player {
            self.cells |= cell >> 1;
        }
    }

    /// Handles player input for cell selection.
    fn on_user_input(&mut self) -> Cell {
        loop {
            println!("Enter a value between 1-9");
            let mut player_input_stream = String::new();
            std::io::stdin()
                .read_line(&mut player_input_stream)
                .expect("read user input");
            match Cell::try_from(player_input_stream.trim()) {
                Ok(c) if !self.is_cell_set(c) => return c,
                Ok(_) => println!("Sorry, this cell has already been set!"),
                Err(e) => println!("Sorry, {}.", e),
            }
        }
    }
}

// Public-Crate block
impl Board {
    pub(crate) const fn new() -> Self {
        Self { cells: 0 }
    }

    /// Checks if every cell has been set.
    pub(crate) const fn is_every_cell_set(&self) -> bool {
        !self.cells & 0b10_1010_1010_1010_1010 == 0
    }

    /// Lets the active player to select a cell.
    pub(crate) fn select_cell(&mut self, player: Player) {
        if self.is_every_cell_set() {
            return;
        }
        println!("{}: Select a Cell", player);
        let cell = self.on_user_input();

        self.set_cell(player, cell);
    }

    /// Applies a [`WinPattern`] over the cells for a given player.
    ///
    /// This is achieved by converting the [`WinPattern`] to a triple of Cells,
    /// iterating over each and checking if the player set all of those cells
    pub(crate) fn check_player_has_won(&self, player: Player, win_pattern: WinPattern) -> bool {
        let cells: [Cell; 3] = win_pattern.into();

        cells.iter().all(|c| self.has_player_set_cell(player, *c))
    }

    /// Checks if the active player has won the game.
    pub(crate) fn has_player_won(&self) -> bool {
        [
            WinPattern::TopRow,
            WinPattern::CentreRow,
            WinPattern::BottomRow,
            WinPattern::LeftColumn,
            WinPattern::CentreColumn,
            WinPattern::RightColumn,
            WinPattern::LeftDiagonal,
            WinPattern::RightDiagonal,
        ]
        .iter()
        .any(|p| {
            self.check_player_has_won(Player::O, *p) || self.check_player_has_won(Player::X, *p)
        })
    }
}

#[cfg(test)]
mod tests;
