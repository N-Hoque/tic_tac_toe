#![allow(non_snake_case)]

#[cfg(test)]
mod set_cell_tests {
    use crate::bit_patterns::Cell;
    use crate::{Board, Player};

    #[test]
    fn top_left_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::TopLeft);
        assert!(board.active_cells == 0b100000000 && board.player_cells == 0b100000000);
    }

    #[test]
    fn top_left_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::TopLeft);
        assert!(board.active_cells == 0b100000000 && board.player_cells == 0);
    }

    #[test]
    fn top_centre_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::TopCentre);
        assert!(board.active_cells == 0b010000000 && board.player_cells == 0b010000000);
    }

    #[test]
    fn top_centre_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::TopCentre);
        assert!(board.active_cells == 0b010000000 && board.player_cells == 0);
    }

    #[test]
    fn top_right_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::TopRight);
        assert!(board.active_cells == 0b001000000 && board.player_cells == 0b001000000);
    }

    #[test]
    fn top_right_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::TopRight);
        assert!(board.active_cells == 0b001000000 && board.player_cells == 0);
    }

    #[test]
    fn centre_left_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::CentreLeft);
        assert!(board.active_cells == 0b000100000 && board.player_cells == 0b000100000);
    }

    #[test]
    fn centre_left_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::CentreLeft);
        assert!(board.active_cells == 0b000100000 && board.player_cells == 0);
    }

    #[test]
    fn centre_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::Centre);
        assert!(board.active_cells == 0b000010000 && board.player_cells == 0b000010000);
    }

    #[test]
    fn centre_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::Centre);
        assert!(board.active_cells == 0b000010000 && board.player_cells == 0);
    }

    #[test]
    fn centre_right_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::CentreRight);
        assert!(board.active_cells == 0b000001000 && board.player_cells == 0b000001000);
    }

    #[test]
    fn centre_right_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::CentreRight);
        assert!(board.active_cells == 0b000001000 && board.player_cells == 0);
    }

    #[test]
    fn bottom_left_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::BottomLeft);
        assert!(board.active_cells == 0b000000100 && board.player_cells == 0b000000100);
    }

    #[test]
    fn bottom_left_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::BottomLeft);
        assert!(board.active_cells == 0b000000100 && board.player_cells == 0);
    }

    #[test]
    fn bottom_centre_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::BottomCentre);
        assert!(board.active_cells == 0b000000010 && board.player_cells == 0b000000010);
    }

    #[test]
    fn bottom_centre_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::BottomCentre);
        assert!(board.active_cells == 0b000000010 && board.player_cells == 0);
    }

    #[test]
    fn bottom_right_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::BottomRight);
        assert!(board.active_cells == 0b000000001 && board.player_cells == 0b000000001);
    }

    #[test]
    fn bottom_right_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::BottomRight);
        assert!(board.active_cells == 0b000000001 && board.player_cells == 0);
    }
}

#[cfg(test)]
mod win_tests {
    use crate::bit_patterns::Cell;
    use crate::{Board, Player};

    #[test]
    fn player_O_won_with_top_row() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::TopLeft);
        board.set_cell(player, Cell::TopCentre);
        board.set_cell(player, Cell::TopRight);
        assert!(board.has_player_won())
    }

    #[test]
    fn player_X_won_with_top_row() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::TopLeft);
        board.set_cell(player, Cell::TopCentre);
        board.set_cell(player, Cell::TopRight);
        assert!(board.has_player_won())
    }

    #[test]
    fn player_O_won_with_centre_row() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::CentreLeft);
        board.set_cell(player, Cell::Centre);
        board.set_cell(player, Cell::CentreRight);
        assert!(board.has_player_won())
    }

    #[test]
    fn player_X_won_with_centre_row() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::CentreLeft);
        board.set_cell(player, Cell::Centre);
        board.set_cell(player, Cell::CentreRight);
        assert!(board.has_player_won())
    }

    #[test]
    fn player_O_won_with_bottom_row() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::BottomLeft);
        board.set_cell(player, Cell::BottomCentre);
        board.set_cell(player, Cell::BottomRight);
        assert!(board.has_player_won())
    }

    #[test]
    fn player_X_won_with_bottom_row() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::BottomLeft);
        board.set_cell(player, Cell::BottomCentre);
        board.set_cell(player, Cell::BottomRight);
        assert!(board.has_player_won())
    }

    #[test]
    fn player_O_won_with_left_column() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::TopLeft);
        board.set_cell(player, Cell::CentreLeft);
        board.set_cell(player, Cell::BottomLeft);
        assert!(board.has_player_won())
    }

    #[test]
    fn player_X_won_with_left_column() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::TopLeft);
        board.set_cell(player, Cell::CentreLeft);
        board.set_cell(player, Cell::BottomLeft);
        assert!(board.has_player_won())
    }

    #[test]
    fn player_O_won_with_centre_column() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::TopCentre);
        board.set_cell(player, Cell::Centre);
        board.set_cell(player, Cell::BottomCentre);
        assert!(board.has_player_won())
    }

    #[test]
    fn player_X_won_with_centre_column() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::TopCentre);
        board.set_cell(player, Cell::Centre);
        board.set_cell(player, Cell::BottomCentre);
        assert!(board.has_player_won())
    }

    #[test]
    fn player_O_won_with_right_column() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::TopRight);
        board.set_cell(player, Cell::CentreRight);
        board.set_cell(player, Cell::BottomRight);
        assert!(board.has_player_won())
    }

    #[test]
    fn player_X_won_with_right_column() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::TopRight);
        board.set_cell(player, Cell::CentreRight);
        board.set_cell(player, Cell::BottomRight);
        assert!(board.has_player_won())
    }

    #[test]
    fn player_O_won_with_left_diagonal() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::TopLeft);
        board.set_cell(player, Cell::Centre);
        board.set_cell(player, Cell::BottomRight);
        assert!(board.has_player_won())
    }

    #[test]
    fn player_X_won_with_left_diagonal() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::TopLeft);
        board.set_cell(player, Cell::Centre);
        board.set_cell(player, Cell::BottomRight);
        assert!(board.has_player_won())
    }

    #[test]
    fn player_O_won_with_right_diagonal() {
        let mut board = Board::new();
        let player = Player::O;
        board.set_cell(player, Cell::TopRight);
        board.set_cell(player, Cell::Centre);
        board.set_cell(player, Cell::BottomLeft);
        assert!(board.has_player_won())
    }

    #[test]
    fn player_X_won_with_right_diagonal() {
        let mut board = Board::new();
        let player = Player::X;
        board.set_cell(player, Cell::TopRight);
        board.set_cell(player, Cell::Centre);
        board.set_cell(player, Cell::BottomLeft);
        assert!(board.has_player_won())
    }
}

#[cfg(test)]
mod board_tests {
    use crate::bit_patterns::Cell;
    use crate::{Board, Player};

    #[test]
    fn new_board_has_empty_cells() {
        let board = Board::new();
        assert!(board.player_cells == 0 && board.active_cells == 0);
    }

    #[test]
    fn board_correctly_states_winner() {
        let mut board = Board::new();
        let winner = Player::O;
        let loser = Player::X;
        board.set_cell(winner, Cell::TopLeft);
        board.set_cell(loser, Cell::BottomCentre);
        board.set_cell(winner, Cell::TopRight);
        board.set_cell(loser, Cell::BottomRight);
        board.set_cell(winner, Cell::TopCentre);
        let mut winner_message = String::new();
        if board.has_player_won() {
            if winner == Player::O {
                winner_message = String::from("Player O Has Won!");
            } else {
                winner_message = String::from("Player X Has Won!");
            }
        }
        assert_eq!("Player O Has Won!", &winner_message);
    }

    #[test]
    fn board_correctly_states_winner_2() {
        let mut board = Board::new();
        let winner = Player::X;
        let loser = Player::O;
        board.set_cell(winner, Cell::TopLeft);
        board.set_cell(loser, Cell::BottomCentre);
        board.set_cell(winner, Cell::Centre);
        board.set_cell(loser, Cell::BottomLeft);
        board.set_cell(winner, Cell::BottomRight);
        let mut winner_message = String::new();
        if board.has_player_won() {
            if winner == Player::O {
                winner_message = String::from("Player O Has Won!");
            } else {
                winner_message = String::from("Player X Has Won!");
            }
        }
        assert_eq!("Player X Has Won!", &winner_message);
    }

    #[test]
    fn board_displays_cells_correctly() {
        let mut board = Board::new();
        let p1 = Player::X;
        let p2 = Player::O;
        board.set_cell(p1, Cell::Centre);
        board.set_cell(p2, Cell::TopLeft);
        board.set_cell(p1, Cell::BottomRight);
        assert_eq!("OEE\nEXE\nEEX\n", &board.to_string());
    }

    #[test]
    fn board_displays_cells_correctly_2() {
        let mut board = Board::new();
        let p1 = Player::X;
        let p2 = Player::O;
        board.set_cell(p1, Cell::Centre);
        board.set_cell(p2, Cell::TopLeft);
        board.set_cell(p1, Cell::BottomRight);
        board.set_cell(p2, Cell::TopRight);
        board.set_cell(p1, Cell::BottomLeft);
        board.set_cell(p2, Cell::TopCentre);
        board.set_cell(p1, Cell::BottomCentre);
        board.set_cell(p2, Cell::CentreLeft);
        board.set_cell(p1, Cell::CentreRight);
        assert_eq!("OOO\nOXX\nXXX\n", &board.to_string());
    }

    #[test]
    fn board_displays_cells_correctly_3() {
        let mut board = Board::new();
        let p1 = Player::O;
        let p2 = Player::X;
        board.set_cell(p1, Cell::TopLeft);
        board.set_cell(p2, Cell::TopCentre);
        board.set_cell(p1, Cell::TopRight);
        board.set_cell(p2, Cell::CentreLeft);
        board.set_cell(p1, Cell::Centre);
        board.set_cell(p2, Cell::CentreRight);
        board.set_cell(p1, Cell::BottomLeft);
        board.set_cell(p2, Cell::BottomCentre);
        board.set_cell(p1, Cell::BottomRight);
        assert_eq!("OXO\nXOX\nOXO\n", &board.to_string());
    }
}
