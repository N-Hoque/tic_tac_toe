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
/// actually set for themselves.
///
/// `set_cells` simply says **if** the cell has been set. It does not say **who** has set it.
struct Board {
    player_cells: u16,
    active_cells: u16,
}

mod board_display_helpers {
    pub(crate) fn draw_top_line() -> &'static str {
        "-------------\n"
    }

    pub(crate) fn draw_cell_line(cell_triple: [&str; 3]) -> String {
        format!(
            "| {} | {} | {} |\n",
            cell_triple[2], cell_triple[1], cell_triple[0]
        )
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output_string = String::new();

        for j in (0..7).rev() {
            if j % 2 == 0 {
                output_string += board_display_helpers::draw_top_line();
            } else {
                let mut cell_triple: [&str; 3] = [""; 3];
                for i in (3 * (j / 2))..3 * (j / 2 + 1) {
                    cell_triple[i % 3] = if self.check_player_has_set_cell(Player::O, i) {
                        "O"
                    } else if self.check_player_has_set_cell(Player::X, i) {
                        "X"
                    } else {
                        "E"
                    };
                }
                output_string += &board_display_helpers::draw_cell_line(cell_triple);
            }
        }

        write!(f, "{}", output_string)
    }
}

impl Board {
    fn check_player_has_set_cell(&self, player: Player, cell_idx: usize) -> bool {
        let cell = 1 << cell_idx;
        let is_cell_active = self.active_cells & cell != 0;
        let player_set_cell = self.player_cells & cell == 0;

        is_cell_active
            && (player == Player::O && player_set_cell || player == Player::X && !player_set_cell)
    }

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
        let cell = self.handle_cell_input();

        self.set_cell(player, cell);
    }

    fn handle_cell_input(&mut self) -> Cell {
        loop {
            println!("Enter a value between 1-9");
            let mut player_input_stream = String::new();
            io::stdin().read_line(&mut player_input_stream).unwrap();
            match Cell::try_from(player_input_stream.trim()) {
                Some(c) if !self.is_cell_set(c) => return c,
                Some(_) => println!("Sorry, this cell has already been set!"),
                None => println!("Sorry, please enter a valid number."),
            }
        }
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
