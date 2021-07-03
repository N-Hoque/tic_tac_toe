//! Handles how the board is displayed to the players.

use crate::bit_patterns::Cell;
use crate::{Board, Player};

fn draw_top_line() -> &'static str {
    "-------------\n"
}

fn draw_cell_line(cell_triple: [char; 3]) -> String {
    format!(
        "| {} | {} | {} |\n",
        cell_triple[2], cell_triple[1], cell_triple[0]
    )
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output_string = String::new();

        for j in (0..7).rev() {
            if j % 2 == 0 {
                output_string += draw_top_line();
            } else {
                let mut cell_triple: [char; 3] = ['E'; 3];
                for (i, c) in ((3 * (j / 2))..3 * (j / 2 + 1))
                    .map(|i| (i + 1).to_string())
                    .filter_map(|i| Cell::try_from(i.as_str()))
                    .enumerate()
                {
                    if self.has_player_set_cell(Player::X, c) {
                        cell_triple[2 - (i % 3)] = 'X';
                    } else if self.has_player_set_cell(Player::O, c) {
                        cell_triple[2 - (i % 3)] = 'O';
                    };
                }
                output_string += &draw_cell_line(cell_triple);
            }
        }

        write!(f, "{}", output_string)
    }
}
