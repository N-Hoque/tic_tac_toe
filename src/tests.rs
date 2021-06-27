#![allow(non_snake_case)]

#[cfg(test)]
mod set_cell_tests {
    use crate::{bit_patterns::PlayerCell, Board};

    #[test]
    fn top_left_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopLeftO);
        assert_eq!(0b011111111111111111, board.cells);
    }

    #[test]
    fn top_left_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopLeftX);
        assert_eq!(0b101111111111111111, board.cells);
    }

    #[test]
    fn top_centre_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopCentreO);
        assert_eq!(0b110111111111111111, board.cells);
    }

    #[test]
    fn top_centre_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopCentreX);
        assert_eq!(0b111011111111111111, board.cells);
    }

    #[test]
    fn top_right_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopRightO);
        assert_eq!(0b111101111111111111, board.cells);
    }

    #[test]
    fn top_right_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopRightX);
        assert_eq!(0b111110111111111111, board.cells);
    }

    #[test]
    fn centre_left_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::CentreLeftO);
        assert_eq!(0b111111011111111111, board.cells);
    }

    #[test]
    fn centre_left_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::CentreLeftX);
        assert_eq!(0b111111101111111111, board.cells);
    }

    #[test]
    fn centre_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::CentreO);
        assert_eq!(0b111111110111111111, board.cells);
    }

    #[test]
    fn centre_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::CentreX);
        assert_eq!(0b111111111011111111, board.cells);
    }

    #[test]
    fn centre_right_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::CentreRightO);
        assert_eq!(0b111111111101111111, board.cells);
    }

    #[test]
    fn centre_right_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::CentreRightX);
        assert_eq!(0b111111111110111111, board.cells);
    }

    #[test]
    fn bottom_left_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::BottomLeftO);
        assert_eq!(0b111111111111011111, board.cells);
    }

    #[test]
    fn bottom_left_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::BottomLeftX);
        assert_eq!(0b111111111111101111, board.cells);
    }

    #[test]
    fn bottom_centre_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::BottomCentreO);
        assert_eq!(0b111111111111110111, board.cells);
    }

    #[test]
    fn bottom_centre_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::BottomCentreX);
        assert_eq!(0b111111111111111011, board.cells);
    }

    #[test]
    fn bottom_right_cell_for_player_O_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::BottomRightO);
        assert_eq!(0b111111111111111101, board.cells);
    }

    #[test]
    fn bottom_right_cell_for_player_X_is_correct() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::BottomRightX);
        assert_eq!(0b111111111111111110, board.cells);
    }
}

#[cfg(test)]
mod win_tests {
    use crate::{
        bit_patterns::{PlayerCell, WinPattern},
        Board,
    };

    #[test]
    fn player_O_won_with_top_row() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopLeftO);
        board.set_cell(PlayerCell::TopCentreO);
        board.set_cell(PlayerCell::TopRightO);
        assert_eq!(0, board.cells & ((WinPattern::TopRow as u32) << 1))
    }

    #[test]
    fn player_X_won_with_top_row() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopLeftX);
        board.set_cell(PlayerCell::TopCentreX);
        board.set_cell(PlayerCell::TopRightX);
        assert_eq!(0, board.cells & (WinPattern::TopRow as u32))
    }

    #[test]
    fn player_O_won_with_centre_row() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::CentreLeftO);
        board.set_cell(PlayerCell::CentreO);
        board.set_cell(PlayerCell::CentreRightO);
        assert_eq!(0, board.cells & ((WinPattern::CentreRow as u32) << 1))
    }

    #[test]
    fn player_X_won_with_centre_row() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::CentreLeftX);
        board.set_cell(PlayerCell::CentreX);
        board.set_cell(PlayerCell::CentreRightX);
        assert_eq!(0, board.cells & (WinPattern::CentreRow as u32))
    }

    #[test]
    fn player_O_won_with_bottom_row() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::BottomLeftO);
        board.set_cell(PlayerCell::BottomCentreO);
        board.set_cell(PlayerCell::BottomRightO);
        assert_eq!(0, board.cells & ((WinPattern::BottomRow as u32) << 1))
    }

    #[test]
    fn player_X_won_with_bottom_row() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::BottomLeftX);
        board.set_cell(PlayerCell::BottomCentreX);
        board.set_cell(PlayerCell::BottomRightX);
        assert_eq!(0, board.cells & (WinPattern::BottomRow as u32))
    }

    #[test]
    fn player_O_won_with_left_column() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopLeftO);
        board.set_cell(PlayerCell::CentreLeftO);
        board.set_cell(PlayerCell::BottomLeftO);
        assert_eq!(0, board.cells & ((WinPattern::LeftColumn as u32) << 1))
    }

    #[test]
    fn player_X_won_with_left_column() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopLeftX);
        board.set_cell(PlayerCell::CentreLeftX);
        board.set_cell(PlayerCell::BottomLeftX);
        assert_eq!(0, board.cells & (WinPattern::LeftColumn as u32))
    }

    #[test]
    fn player_O_won_with_centre_column() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopCentreO);
        board.set_cell(PlayerCell::CentreO);
        board.set_cell(PlayerCell::BottomCentreO);
        assert_eq!(0, board.cells & ((WinPattern::CentreColumn as u32) << 1))
    }

    #[test]
    fn player_X_won_with_centre_column() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopCentreX);
        board.set_cell(PlayerCell::CentreX);
        board.set_cell(PlayerCell::BottomCentreX);
        assert_eq!(0, board.cells & (WinPattern::CentreColumn as u32))
    }

    #[test]
    fn player_O_won_with_right_column() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopRightO);
        board.set_cell(PlayerCell::CentreRightO);
        board.set_cell(PlayerCell::BottomRightO);
        assert_eq!(0, board.cells & ((WinPattern::RightColumn as u32) << 1))
    }

    #[test]
    fn player_X_won_with_right_column() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopRightX);
        board.set_cell(PlayerCell::CentreRightX);
        board.set_cell(PlayerCell::BottomRightX);
        assert_eq!(0, board.cells & (WinPattern::RightColumn as u32))
    }

    #[test]
    fn player_O_won_with_left_diagonal() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopLeftO);
        board.set_cell(PlayerCell::CentreO);
        board.set_cell(PlayerCell::BottomRightO);
        assert_eq!(0, board.cells & ((WinPattern::LeftDiagonal as u32) << 1))
    }

    #[test]
    fn player_X_won_with_left_diagonal() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopLeftX);
        board.set_cell(PlayerCell::CentreX);
        board.set_cell(PlayerCell::BottomRightX);
        assert_eq!(0, board.cells & (WinPattern::LeftDiagonal as u32))
    }

    #[test]
    fn player_O_won_with_right_diagonal() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopRightO);
        board.set_cell(PlayerCell::CentreO);
        board.set_cell(PlayerCell::BottomLeftO);
        assert_eq!(0, board.cells & ((WinPattern::RightDiagonal as u32) << 1))
    }

    #[test]
    fn player_X_won_with_right_diagonal() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::TopRightX);
        board.set_cell(PlayerCell::CentreX);
        board.set_cell(PlayerCell::BottomLeftX);
        assert_eq!(0, board.cells & (WinPattern::RightDiagonal as u32))
    }
}

#[cfg(test)]
mod board_tests {
    use crate::{bit_patterns::PlayerCell, Board, Player};

    #[test]
    fn new_board_has_empty_cells() {
        let board = Board::new();
        assert_eq!(0b111111111111111111, board.cells);
    }

    #[test]
    fn board_correctly_states_winner() {
        let mut board = Board::new();
        let winner = Player::O;
        board.set_cell(PlayerCell::TopLeftO);
        board.set_cell(PlayerCell::BottomCentreO);
        board.set_cell(PlayerCell::TopRightO);
        board.set_cell(PlayerCell::BottomRightO);
        board.set_cell(PlayerCell::TopCentreO);
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
    fn board_displays_cells_correctly() {
        let mut board = Board::new();
        board.set_cell(PlayerCell::CentreX);
        board.set_cell(PlayerCell::TopLeftO);
        board.set_cell(PlayerCell::BottomRightX);
        assert_eq!("OEE\nEXE\nEEX\n", &board.to_string());
    }
}
