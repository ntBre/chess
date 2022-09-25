use crate::board::Board;
use crate::coord::AlgebraicCoord::*;
use crate::piece::Piece;
use crate::piece::PieceType::*;
use crate::piece::Player::*;
use crate::square::Square;
use crate::Move;

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

#[test]
fn king_move() {
    let king = Piece::new(White, King);
    let got = king.moves(E4);
    let want = vec![
        Move::new(king, F3),
        Move::new(king, D3),
        Move::new(king, E3),
        Move::new(king, F4),
        Move::new(king, D5),
        Move::new(king, F5),
        Move::new(king, D4),
        Move::new(king, E5),
    ];
    assert_eq!(got, want, "got {:#?}, wanted {:#?}", got, want);
}
