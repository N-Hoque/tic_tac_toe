//! Provides functionality for the [Game] struct

use crate::{Board, Game, Player};

/// A helper enum for handling the end game state.
pub(crate) enum EndState {
    Replay,
    End,
    Continue,
}

impl Game {
    /// Allows the user to decide who starts the game.
    fn select_start_player() -> Player {
        println!("Who would like to start?");
        println!("Press O or 1 for Player O");
        println!("Press X or 2 for Player X");
        loop {
            let mut play_again_buffer = String::new();
            std::io::stdin().read_line(&mut play_again_buffer).unwrap();
            match play_again_buffer.trim() {
                "O" | "o" | "1" => return Player::O,
                "X" | "x" | "2" => return Player::X,
                _ => {
                    println!("Sorry, please provide a valid selection.");
                    continue;
                }
            }
        }
    }
}

impl Game {
    /// Creates a new [Game]. At game start a player is randomly chosen.
    pub(crate) fn new() -> Game {
        Game {
            board: Board::new(),
            current_player: Game::select_start_player(),
        }
    }

    /// Resets the game.
    pub(crate) fn reset(&mut self) {
        self.board = Board::new();
        self.current_player = Game::select_start_player();
    }

    /// Handles the end state of the game.
    pub(crate) fn on_end(&mut self) -> EndState {
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
