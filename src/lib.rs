//! Welcome to Tic-Tac-Toe!
//!
//! This crate implements the schoolyard game using only bit patterns.
//!
//! An explanation of how this works is stated below:
//!
//!
//! # Explanation
//! - Each cell can be encoded as a two-bit pair.
//! - The top-left cell will contain the MSBs
//! - The bottom-right cell contains the LSBs
//! - We read from left-to-right, top-to-bottom
//!
//! When a player makes a selection, a bit is flipped
//! in the cell. Player O flips the left bit, player X
//! flips the right bit.
mod bit_patterns;

use bit_patterns::*;

use std::fmt::Display;
use std::io;

/// This enum indicates who the active player is.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Player {
    O,
    X,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::O => write!(f, "Player O"),
            Player::X => write!(f, "Player X"),
        }
    }
}

/// This represent the state of the game board.
#[derive(Debug)]
struct Board {
    /// The cells the players will set
    cells: u32,
    /// The state of the cells above.
    set_cells: u32,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output_string = String::new();
        let mut counter = 0;
        let pattern_o = 0b10;
        let pattern_x = 0b01;

        for i in (0..9).rev() {
            output_string += if self.cells & (pattern_o << (2 * i)) == 0 {
                "O"
            } else if self.cells & (pattern_x << (2 * i)) == 0 {
                "X"
            } else {
                "E"
            };
            counter += 1;
            if counter % 3 == 0 {
                output_string += "\n";
            }
        }

        write!(f, "{}", output_string)
    }
}

impl Board {
    /// Creates a new board.
    ///
    /// The state of the set cells is zero as no cells have been set.
    /// But the state of the player cells is a series of 1s.
    ///
    /// This is due to how the patterns are set up.
    fn new() -> Board {
        Board {
            cells: 0b111111111111111111,
            set_cells: 0,
        }
    }

    /// Checks if every cell has been set.
    fn is_every_cell_set(&self) -> bool {
        self.set_cells ^ 0b111111111 == 0
    }

    /// Checks if a given cell has been set.
    fn is_cell_set(&self, cell_to_check: ActiveCell) -> bool {
        self.set_cells & (cell_to_check as u32) != 0
    }

    /// Sets the cell for a given player.
    fn set_cell(&mut self, player_cell: PlayerCell) {
        self.cells ^= player_cell as u32;
        self.set_cells |= ActiveCell::from(player_cell) as u32;
    }

    /// Checks if the active player has won the game.
    fn has_player_won(&self) -> bool {
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
        .map(|p| *p as u32)
        .any(|p| self.cells & (p << 1) == 0 || self.cells & p == 0)
    }

    /// Enable the active player to select a cell.
    fn select_cell(&mut self, player: Player) {
        if self.is_every_cell_set() {
            return;
        }
        println!("{}: Select a Cell", player);
        let mut input: ActiveCell;
        loop {
            let mut player_input_stream = String::new();
            println!("Enter a value between 1-9");
            io::stdin().read_line(&mut player_input_stream).unwrap();
            input = match player_input_stream.trim() {
                "1" => ActiveCell::TopLeft,
                "2" => ActiveCell::TopCentre,
                "3" => ActiveCell::TopRight,
                "4" => ActiveCell::CentreLeft,
                "5" => ActiveCell::Centre,
                "6" => ActiveCell::CentreRight,
                "7" => ActiveCell::BottomLeft,
                "8" => ActiveCell::BottomCentre,
                "9" => ActiveCell::BottomRight,
                _ => {
                    println!("Invalid input.");
                    continue;
                }
            };
            if self.is_cell_set(input) {
                println!("Sorry, this cell has already been set!");
                continue;
            }
            break;
        }

        let player_cell = get_player_cell(player, BoardCell::from(input));
        self.set_cell(player_cell);
    }
}

/// Represents the game state
pub struct Game {
    board: Board,
    current_player: Player,
}

/// A helper enum for handling the end game state.
enum EndState {
    Replay,
    End,
    Continue,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            current_player: Player::O,
        }
    }

    fn reset(&mut self) {
        self.board = Board::new();
        self.current_player = Player::O;
    }

    pub fn play(&mut self) {
        println!("Welcome to Tic-Tac-Toe!");
        println!("Player O Begins.");

        loop {
            self.board.select_cell(self.current_player);
            println!("{}", self.board);
            match self.handle_end_state() {
                EndState::End => break,
                EndState::Replay => continue,
                EndState::Continue => (),
            }
            self.current_player = if self.current_player == Player::O {
                Player::X
            } else {
                Player::O
            };
        }
        println!("Thank you for playing Tic-Tac-Toe!");
    }

    /// Handles the end state of the game.
    fn handle_end_state(&mut self) -> EndState {
        if !self.board.has_player_won() && !self.board.is_every_cell_set() {
            return EndState::Continue;
        }

        if self.board.has_player_won() {
            println!("{} has won!", self.current_player);
        } else {
            println!("Draw!");
        }
        println!("Would you like to play again?");
        println!("Press Y for Yes, N for No.");

        loop {
            let mut play_again_buffer = String::new();
            io::stdin().read_line(&mut play_again_buffer).unwrap();
            match play_again_buffer.trim() {
                "n" | "N" => return EndState::End,
                "y" | "Y" => {
                    self.reset();
                    return EndState::Replay;
                }
                _ => {
                    println!("Please enter a valid argument.");
                    continue;
                }
            }
        }
    }
}

/// Gets a player cell for a given board cell.
fn get_player_cell(player: Player, cell: BoardCell) -> PlayerCell {
    match cell {
        BoardCell::TopLeft => {
            if player == Player::O {
                PlayerCell::TopLeftO
            } else {
                PlayerCell::TopLeftX
            }
        }
        BoardCell::TopCentre => {
            if player == Player::O {
                PlayerCell::TopCentreO
            } else {
                PlayerCell::TopCentreX
            }
        }
        BoardCell::TopRight => {
            if player == Player::O {
                PlayerCell::TopRightO
            } else {
                PlayerCell::TopRightX
            }
        }
        BoardCell::CentreLeft => {
            if player == Player::O {
                PlayerCell::CentreLeftO
            } else {
                PlayerCell::CentreLeftX
            }
        }
        BoardCell::Centre => {
            if player == Player::O {
                PlayerCell::CentreO
            } else {
                PlayerCell::CentreX
            }
        }
        BoardCell::CentreRight => {
            if player == Player::O {
                PlayerCell::CentreRightO
            } else {
                PlayerCell::CentreRightX
            }
        }
        BoardCell::BottomLeft => {
            if player == Player::O {
                PlayerCell::BottomLeftO
            } else {
                PlayerCell::BottomLeftX
            }
        }
        BoardCell::BottomCentre => {
            if player == Player::O {
                PlayerCell::BottomCentreO
            } else {
                PlayerCell::BottomCentreX
            }
        }
        BoardCell::BottomRight => {
            if player == Player::O {
                PlayerCell::BottomRightO
            } else {
                PlayerCell::BottomRightX
            }
        }
    }
}

#[cfg(test)]
mod tests;
