use crate::board::Board;
use crate::coord::AlgebraicCoord::*;
use crate::square::Square;

#[test]
fn initial_eval() {
    let board = Board::starting_position();
    assert_eq!(board.evaluate(), 0.0);
}

#[test]
fn white_pawn_up() {
    let mut board = Board::starting_position();
    board[E7] = Square::none();
    assert_eq!(board.evaluate(), 1.0);
}

#[test]
fn black_pawn_up() {
    let mut board = Board::starting_position();
    board[E2] = Square::none();
    assert_eq!(board.evaluate(), -1.0);
}
