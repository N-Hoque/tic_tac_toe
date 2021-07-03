//! Welcome to Tic-Tac-Toe!
//!
//! This crate implements the schoolyard game using only bit patterns.
mod bit_patterns;

use bit_patterns::*;

use std::fmt::Display;
use std::io;

/// This enum indicates who the active player is.
#[derive(PartialEq, Eq, Clone, Copy)]
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

/// A helper enum for handling the end game state.
enum EndState {
    Replay,
    End,
    Continue,
}

/// This represent the state of the game board.
///
/// There is a subtlety between these two fields.
///
/// `cells` refers to player-settable cells. These are what the players
/// actually sets for themselves.
///
/// `set_cells` simply says **if** the cell has been set. It does not say **who** has set it.
#[derive(Debug)]
struct Board {
    player_cells: u16,
    active_cells: u16,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output_string = String::new();
        let mut counter = 0;

        for i in (0..9).rev() {
            output_string +=
                if self.active_cells & (1 << i) != 0 && (self.player_cells & (1 << i)) == 0 {
                    "O"
                } else if self.active_cells & (1 << i) != 0 && self.player_cells & (1 << i) != 0 {
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
    fn new() -> Board {
        Board {
            player_cells: 0,
            active_cells: 0,
        }
    }

    /// Checks if every cell has been set.
    fn is_every_cell_set(&self) -> bool {
        self.active_cells ^ 0b111111111 == 0
    }

    /// Checks if a given cell has been set.
    fn is_cell_set(&self, cell_to_check: Cell) -> bool {
        self.active_cells & (cell_to_check as u16) != 0
    }

    /// Sets the cell for a given player.
    fn set_cell(&mut self, player: Player, cell: Cell) {
        if player == Player::X {
            self.player_cells |= cell as u16;
        }
        self.active_cells |= cell as u16;
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
        .map(|p| *p as u16)
        .any(|p| {
            !self.active_cells & p == 0
                && (self.player_cells & p == 0 || !self.player_cells & p == 0)
        })
    }

    /// Lets the active player to select a cell.
    fn select_cell(&mut self, player: Player) {
        if self.is_every_cell_set() {
            return;
        }
        println!("{}: Select a Cell", player);
        let mut cell: Cell;
        loop {
            let mut player_input_stream = String::new();
            println!("Enter a value between 1-9");
            io::stdin().read_line(&mut player_input_stream).unwrap();
            cell = match player_input_stream.trim() {
                "1" => Cell::TopLeft,
                "2" => Cell::TopCentre,
                "3" => Cell::TopRight,
                "4" => Cell::CentreLeft,
                "5" => Cell::Centre,
                "6" => Cell::CentreRight,
                "7" => Cell::BottomLeft,
                "8" => Cell::BottomCentre,
                "9" => Cell::BottomRight,
                _ => {
                    println!("Invalid input.");
                    continue;
                }
            };
            if self.is_cell_set(cell) {
                println!("Sorry, this cell has already been set!");
                continue;
            }
            break;
        }

        self.set_cell(player, cell);
    }
}

/// Represents the game state
pub struct Game {
    board: Board,
    current_player: Player,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            current_player: Player::O,
        }
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

    fn reset(&mut self) {
        self.board = Board::new();
        self.current_player = Player::O;
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

#[cfg(test)]
mod tests;
