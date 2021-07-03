//! Provides functionality for the [Game] struct

use rand::{thread_rng, Rng};

use crate::{Board, Game, Player};

/// A helper enum for handling the end game state.
pub(crate) enum EndState {
    Replay,
    End,
    Continue,
}

impl Game {
    /// Creates a new [Game]. At game start a player is randomly chosen.
    pub(crate) fn new() -> Game {
        Game {
            board: Board::new(),
            current_player: if thread_rng().gen_bool(0.5) {
                Player::X
            } else {
                Player::O
            },
        }
    }

    /// Resets the game.
    pub(crate) fn reset(&mut self) {
        self.board = Board::new();
        self.current_player = if thread_rng().gen_bool(0.5) {
            Player::X
        } else {
            Player::O
        };
    }

    /// Handles the end state of the game.
    pub(crate) fn handle_end_state(&mut self) -> EndState {
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
            std::io::stdin().read_line(&mut play_again_buffer).unwrap();
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
