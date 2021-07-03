//! Welcome to Tic-Tac-Toe!
//!
//! This crate implements the schoolyard game using only bit patterns.
mod bit_patterns;
mod board;
pub mod game;
mod player;

use crate::game::EndState;

/// Indicates who the active player is.
#[derive(PartialEq, Eq, Clone, Copy)]
enum Player {
    O,
    X,
}

/// Represents the state of the game board.
struct Board {
    cells: u32,
}

/// Represents the total game state
struct Game {
    board: Board,
    current_player: Player,
}

/// Runs the game loop
pub fn play() {
    let mut game = Game::new();
    println!("Welcome to Tic-Tac-Toe!");
    println!("{} Begins.", game.current_player);

    loop {
        game.board.select_cell(game.current_player);
        println!("{}", game.board);
        match game.handle_end_state() {
            EndState::End => break,
            EndState::Replay => continue,
            EndState::Continue => (),
        }
        game.current_player.swap();
    }
    println!("Thank you for playing Tic-Tac-Toe!");
}
