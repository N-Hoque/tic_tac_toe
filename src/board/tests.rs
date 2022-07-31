#![allow(non_snake_case)]

#[cfg(test)]
mod set_cell_tests {
    use crate::bit_patterns::Cell;
    use crate::{Board, Player};

    const fn get_test_cell(player: Player, cell_idx: usize) -> u32 {
        (if let Player::O = player { 0b10 } else { 0b11 }) << (cell_idx * 2)
    }

    fn cell_test(player: Player, cell: Cell, idx: usize) {
        let mut board = Board::new();
        board.set_cell(player, cell);
        let expected = get_test_cell(player, idx);
        assert_eq!(
            board.cells, expected,
            "Got {:0b} expected {:0b}",
            board.cells, expected
        );
        assert!(board.has_player_set_cell(player, cell));
    }

    #[test]
    fn top_left_cell_for_player_X_is_correct() {
        cell_test(Player::X, Cell::TopLeft, 8);
    }

    #[test]
    fn top_left_cell_for_player_O_is_correct() {
        cell_test(Player::O, Cell::TopLeft, 8);
    }

    #[test]
    fn top_centre_cell_for_player_X_is_correct() {
        cell_test(Player::X, Cell::TopCentre, 7);
    }

    #[test]
    fn top_centre_cell_for_player_O_is_correct() {
        cell_test(Player::O, Cell::TopCentre, 7);
    }

    #[test]
    fn top_right_cell_for_player_X_is_correct() {
        cell_test(Player::X, Cell::TopRight, 6);
    }

    #[test]
    fn top_right_cell_for_player_O_is_correct() {
        cell_test(Player::O, Cell::TopRight, 6);
    }

    #[test]
    fn centre_left_cell_for_player_X_is_correct() {
        cell_test(Player::X, Cell::CentreLeft, 5);
    }

    #[test]
    fn centre_left_cell_for_player_O_is_correct() {
        cell_test(Player::O, Cell::CentreLeft, 5);
    }

    #[test]
    fn centre_cell_for_player_X_is_correct() {
        cell_test(Player::X, Cell::Centre, 4);
    }

    #[test]
    fn centre_cell_for_player_O_is_correct() {
        cell_test(Player::O, Cell::Centre, 4);
    }

    #[test]
    fn centre_right_cell_for_player_X_is_correct() {
        cell_test(Player::X, Cell::CentreRight, 3);
    }

    #[test]
    fn centre_right_cell_for_player_O_is_correct() {
        cell_test(Player::O, Cell::CentreRight, 3);
    }

    #[test]
    fn bottom_left_cell_for_player_X_is_correct() {
        cell_test(Player::X, Cell::BottomLeft, 2);
    }

    #[test]
    fn bottom_left_cell_for_player_O_is_correct() {
        cell_test(Player::O, Cell::BottomLeft, 2);
    }

    #[test]
    fn bottom_centre_cell_for_player_X_is_correct() {
        cell_test(Player::X, Cell::BottomCentre, 1);
    }

    #[test]
    fn bottom_centre_cell_for_player_O_is_correct() {
        cell_test(Player::O, Cell::BottomCentre, 1);
    }

    #[test]
    fn bottom_right_cell_for_player_X_is_correct() {
        cell_test(Player::X, Cell::BottomRight, 0);
    }

    #[test]
    fn bottom_right_cell_for_player_O_is_correct() {
        cell_test(Player::O, Cell::BottomRight, 0);
    }

    #[test]
    fn top_row_mixed_cells_are_set_correctly() {
        let mut board = Board::new();
        board.set_cell(Player::O, Cell::TopLeft);
        assert!(board.has_player_set_cell(Player::O, Cell::TopLeft));
        board.set_cell(Player::X, Cell::TopCentre);
        assert!(board.has_player_set_cell(Player::X, Cell::TopCentre));
        board.set_cell(Player::O, Cell::TopRight);
        assert!(board.has_player_set_cell(Player::O, Cell::TopRight));
        assert_eq!(0b10_1110_0000_0000_0000, board.cells);
    }

    #[test]
    fn every_cell_is_set_returns_true_for_check() {
        let mut board = Board::new();
        board.set_cell(Player::O, Cell::TopLeft);
        board.set_cell(Player::X, Cell::CentreLeft);
        board.set_cell(Player::O, Cell::BottomLeft);

        board.set_cell(Player::X, Cell::TopCentre);
        board.set_cell(Player::O, Cell::Centre);
        board.set_cell(Player::X, Cell::BottomCentre);

        board.set_cell(Player::O, Cell::TopRight);
        board.set_cell(Player::X, Cell::CentreRight);
        board.set_cell(Player::O, Cell::BottomRight);

        assert_eq!(0, !board.cells & 0b10_1010_1010_1010_1010);
        assert!(board.is_every_cell_set());
    }

    #[test]
    fn every_cell_but_one_is_set_returns_false_for_check() {
        let mut board = Board::new();
        board.set_cell(Player::O, Cell::TopLeft);
        board.set_cell(Player::X, Cell::CentreLeft);
        board.set_cell(Player::O, Cell::BottomLeft);

        board.set_cell(Player::X, Cell::TopCentre);
        board.set_cell(Player::O, Cell::Centre);
        board.set_cell(Player::X, Cell::BottomCentre);

        board.set_cell(Player::O, Cell::TopRight);
        board.set_cell(Player::X, Cell::CentreRight);

        assert_ne!(0, !board.cells & 0b10_1010_1010_1010_1010);
        assert!(!board.is_every_cell_set());
    }

    #[test]
    fn check_cell_correctly_says_O_set_cell() {
        let mut board = Board::new();
        board.set_cell(Player::O, Cell::TopLeft);
        assert!(board.has_player_set_cell(Player::O, Cell::TopLeft));
    }

    #[test]
    fn check_cell_correctly_says_X_did_not_set_cell() {
        let mut board = Board::new();
        board.set_cell(Player::O, Cell::TopLeft);
        assert!(!board.has_player_set_cell(Player::X, Cell::TopLeft));
    }

    #[test]
    fn check_cell_correctly_says_X_set_cell() {
        let mut board = Board::new();
        board.set_cell(Player::X, Cell::TopLeft);
        assert!(board.has_player_set_cell(Player::X, Cell::TopLeft));
    }

    #[test]
    fn check_cell_correctly_says_O_did_not_set_cell() {
        let mut board = Board::new();
        board.set_cell(Player::X, Cell::TopLeft);
        assert!(!board.has_player_set_cell(Player::O, Cell::TopLeft));
    }
}

#[cfg(test)]
mod win_tests {
    use crate::bit_patterns::{Cell, WinPattern};
    use crate::{Board, Player};

    type Cells = [Cell; 3];

    const fn switch_player(player: Player) -> Player {
        match player {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }

    fn win_test(player: Player, win_pattern: WinPattern) {
        let mut board = Board::new();
        let cells: Cells = win_pattern.into();
        board.set_cell(player, cells[0]);
        board.set_cell(player, cells[1]);
        board.set_cell(player, cells[2]);
        assert!(
            board.check_player_has_won(player, win_pattern),
            "For these cells {:>018b} and win pattern {:>018b}, the player should win",
            board.cells,
            win_pattern
        );
    }

    fn lose_test(player: Player, win_pattern: WinPattern) {
        let mut board = Board::new();
        let cells: Cells = win_pattern.into();
        let other_player = switch_player(player);
        board.set_cell(other_player, cells[0]);
        board.set_cell(other_player, cells[1]);
        board.set_cell(other_player, cells[2]);
        assert!(
            !board.check_player_has_won(player, win_pattern),
            "For these cells {:>018b} and win pattern {:>018b}, the player should lose",
            board.cells,
            win_pattern
        );
    }

    #[test]
    fn player_O_won_with_top_row() {
        win_test(Player::O, WinPattern::TopRow);
    }

    #[test]
    fn player_X_won_with_top_row() {
        win_test(Player::X, WinPattern::TopRow);
    }

    #[test]
    fn player_O_won_with_centre_row() {
        win_test(Player::O, WinPattern::CentreRow);
    }

    #[test]
    fn player_X_won_with_centre_row() {
        win_test(Player::X, WinPattern::CentreRow);
    }

    #[test]
    fn player_O_won_with_bottom_row() {
        win_test(Player::O, WinPattern::BottomRow);
    }

    #[test]
    fn player_X_won_with_bottom_row() {
        win_test(Player::X, WinPattern::CentreRow);
    }

    #[test]
    fn player_O_won_with_left_column() {
        win_test(Player::O, WinPattern::LeftColumn);
    }

    #[test]
    fn player_X_won_with_left_column() {
        win_test(Player::X, WinPattern::LeftColumn);
    }

    #[test]
    fn player_O_won_with_centre_column() {
        win_test(Player::O, WinPattern::CentreColumn);
    }

    #[test]
    fn player_X_won_with_centre_column() {
        win_test(Player::X, WinPattern::CentreColumn);
    }

    #[test]
    fn player_O_won_with_right_column() {
        win_test(Player::O, WinPattern::RightColumn);
    }

    #[test]
    fn player_X_won_with_right_column() {
        win_test(Player::X, WinPattern::RightColumn);
    }

    #[test]
    fn player_O_won_with_left_diagonal() {
        win_test(Player::O, WinPattern::LeftDiagonal);
    }

    #[test]
    fn player_X_won_with_left_diagonal() {
        win_test(Player::X, WinPattern::LeftDiagonal);
    }

    #[test]
    fn player_O_won_with_right_diagonal() {
        win_test(Player::O, WinPattern::RightDiagonal);
    }

    #[test]
    fn player_X_won_with_right_diagonal() {
        win_test(Player::X, WinPattern::RightDiagonal);
    }

    #[test]
    fn player_O_lost_with_top_row() {
        lose_test(Player::O, WinPattern::TopRow);
    }

    #[test]
    fn player_X_lost_with_top_row() {
        lose_test(Player::X, WinPattern::TopRow);
    }

    #[test]
    fn player_O_lost_with_centre_row() {
        lose_test(Player::O, WinPattern::CentreRow);
    }

    #[test]
    fn player_X_lost_with_centre_row() {
        lose_test(Player::X, WinPattern::CentreRow);
    }

    #[test]
    fn player_O_lost_with_bottom_row() {
        lose_test(Player::O, WinPattern::BottomRow);
    }

    #[test]
    fn player_X_lost_with_bottom_row() {
        lose_test(Player::X, WinPattern::CentreRow);
    }

    #[test]
    fn player_O_lost_with_left_column() {
        lose_test(Player::O, WinPattern::LeftColumn);
    }

    #[test]
    fn player_X_lost_with_left_column() {
        lose_test(Player::X, WinPattern::LeftColumn);
    }

    #[test]
    fn player_O_lost_with_centre_column() {
        lose_test(Player::O, WinPattern::CentreColumn);
    }

    #[test]
    fn player_X_lost_with_centre_column() {
        lose_test(Player::X, WinPattern::CentreColumn);
    }

    #[test]
    fn player_O_lost_with_right_column() {
        lose_test(Player::O, WinPattern::RightColumn);
    }

    #[test]
    fn player_X_lost_with_right_column() {
        lose_test(Player::X, WinPattern::RightColumn);
    }

    #[test]
    fn player_O_lost_with_left_diagonal() {
        lose_test(Player::O, WinPattern::LeftDiagonal);
    }

    #[test]
    fn player_X_lost_with_left_diagonal() {
        lose_test(Player::X, WinPattern::LeftDiagonal);
    }

    #[test]
    fn player_O_lost_with_right_diagonal() {
        lose_test(Player::O, WinPattern::RightDiagonal);
    }

    #[test]
    fn player_X_lost_with_right_diagonal() {
        lose_test(Player::X, WinPattern::RightDiagonal);
    }

    #[test]
    fn check_mixed_row_returns_false() {
        let mut board = Board::new();
        let cells: Cells = WinPattern::TopRow.into();
        let mut player = Player::O;
        board.set_cell(player, cells[0]);
        player.swap();
        board.set_cell(player, cells[1]);
        player.swap();
        board.set_cell(player, cells[2]);
        assert!(
            !board.check_player_has_won(Player::O, WinPattern::TopRow),
            "Unfortunately Player O still won.",
        );
        assert!(
            !board.check_player_has_won(Player::X, WinPattern::TopRow),
            "Unfortunately Player X still won.",
        );
        assert!(!board.has_player_won());
    }

    #[test]
    fn check_full_mixed_board_returns_false() {
        let mut board = Board::new();
        board.set_cell(Player::O, Cell::Centre);
        board.set_cell(Player::X, Cell::TopLeft);
        board.set_cell(Player::O, Cell::TopCentre);
        board.set_cell(Player::X, Cell::BottomCentre);
        board.set_cell(Player::O, Cell::CentreRight);
        board.set_cell(Player::X, Cell::CentreLeft);
        board.set_cell(Player::O, Cell::BottomLeft);
        board.set_cell(Player::X, Cell::TopRight);
        board.set_cell(Player::O, Cell::BottomRight);
        println!("{}", board);
        assert!(
            !board.check_player_has_won(Player::O, WinPattern::TopRow),
            "Unfortunately Player O still won.",
        );
        assert!(
            !board.check_player_has_won(Player::X, WinPattern::TopRow),
            "Unfortunately Player X still won.",
        );
        assert!(!board.has_player_won());
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
        let winner_message = if board.has_player_won() {
            format!("{} Has Won!", winner)
        } else {
            String::new()
        };
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
        let winner_message = if board.has_player_won() {
            format!("{} Has Won!", winner)
        } else {
            String::new()
        };
        assert_eq!("Player X Has Won!", &winner_message);
    }
}

#[cfg(test)]
mod board_display_tests {
    use crate::bit_patterns::Cell;
    use crate::{Board, Player};

    #[test]
    fn board_displays_cells_correctly() {
        let mut board = Board::new();
        let p1 = Player::X;
        let p2 = Player::O;
        board.set_cell(p1, Cell::Centre);
        board.set_cell(p2, Cell::TopLeft);
        board.set_cell(p1, Cell::BottomRight);
        assert_eq!("-------------\n| O | E | E |\n-------------\n| E | X | E |\n-------------\n| E | E | X |\n-------------\n", &board.to_string());
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
        assert_eq!("-------------\n| O | O | O |\n-------------\n| O | X | X |\n-------------\n| X | X | X |\n-------------\n", &board.to_string());
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
        assert_eq!("-------------\n| O | X | O |\n-------------\n| X | O | X |\n-------------\n| O | X | O |\n-------------\n", &board.to_string());
    }
}
