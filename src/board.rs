//! Provides functionality for the [Board] struct

use crate::bit_patterns::{Cell, WinPattern};
use crate::{Board, Player};

mod board_display;

// Private block
impl Board {
    #[allow(non_snake_case)]
    fn check_player_has_set_cell(&self, player: Player, cell: Cell) -> bool {
        self.cells & cell != 0
            && if player == Player::O {
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
        if player == Player::X {
            self.cells |= cell >> 1
        }
    }

    /// Handles player input for cell selection.
    fn handle_cell_input(&mut self) -> Cell {
        loop {
            println!("Enter a value between 1-9");
            let mut player_input_stream = String::new();
            std::io::stdin()
                .read_line(&mut player_input_stream)
                .unwrap();
            match Cell::try_from(player_input_stream.trim()) {
                Some(c) if !self.is_cell_set(c) => return c,
                Some(_) => println!("Sorry, this cell has already been set!"),
                None => println!("Sorry, please enter a valid number."),
            }
        }
    }
}

// Public-Crate block
impl Board {
    pub(crate) fn new() -> Board {
        Board { cells: 0 }
    }

    /// Checks if every cell has been set.
    pub(crate) fn is_every_cell_set(&self) -> bool {
        !self.cells & 0b101010101010101010 == 0
    }

    /// Lets the active player to select a cell.
    pub(crate) fn select_cell(&mut self, player: Player) {
        if self.is_every_cell_set() {
            return;
        }
        println!("{}: Select a Cell", player);
        let cell = self.handle_cell_input();

        self.set_cell(player, cell);
    }

    /// Applies a [WinPattern] over the cells for a given player.
    ///
    /// This is achieved by converting the [WinPattern] to a triple of Cells,
    /// iterating over each and checking if the player set all of those cells
    pub(crate) fn apply_win_pattern_for_player(
        &self,
        player: Player,
        win_pattern: WinPattern,
    ) -> bool {
        let cells: [Cell; 3] = win_pattern.into();

        cells
            .iter()
            .map(|c| *c)
            .all(|c| self.check_player_has_set_cell(player, c))
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
            self.apply_win_pattern_for_player(Player::O, *p)
                || self.apply_win_pattern_for_player(Player::X, *p)
        })
    }
}

#[cfg(test)]
mod tests;
