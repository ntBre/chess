use std::fmt::Display;

use crate::{coord::AlgebraicCoord, Move};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Player {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Piece {
    pub(crate) player: Player,
    pub(crate) piece: PieceType,
}

impl Piece {
    pub(crate) fn new(player: Player, piece: PieceType) -> Self {
        Self { player, piece }
    }

    pub(crate) fn value(&self) -> f64 {
        let fac = match self.player {
            Player::White => 1.0,
            Player::Black => -1.0,
        };
        let value = match self.piece {
            PieceType::King => 0.0,
            PieceType::Queen => 9.0,
            PieceType::Rook => 5.0,
            PieceType::Bishop => 3.0,
            PieceType::Knight => 3.0,
            PieceType::Pawn => 1.0,
        };
        fac * value
    }

    /// return all of the legal moves for a piece, disregarding the board state
    pub(crate) fn moves(&self, coord: AlgebraicCoord) -> Vec<Move> {
        let mut ret = Vec::new();
        match self.piece {
            PieceType::King => {
                let (i, j) = coord.into();
                if i < 7 && j < 7 {
                    ret.push(Move::new(*self, (i + 1, j + 1)));
                }
                if i < 7 && j > 0 {
                    ret.push(Move::new(*self, (i + 1, j - 1)));
                }
                if i < 7 {
                    ret.push(Move::new(*self, (i + 1, j)));
                }
                if j < 7 {
                    ret.push(Move::new(*self, (i, j + 1)));
                }
                if i > 0 && j > 0 {
                    ret.push(Move::new(*self, (i - 1, j - 1)));
                }
                if i > 0 && j < 7 {
                    ret.push(Move::new(*self, (i - 1, j + 1)));
                }
                if j > 0 {
                    ret.push(Move::new(*self, (i, j - 1)));
                }
                if i > 0 {
                    ret.push(Move::new(*self, (i - 1, j)));
                }
            }
            PieceType::Queen => todo!(),
            PieceType::Rook => todo!(),
            PieceType::Bishop => todo!(),
            PieceType::Knight => todo!(),
            PieceType::Pawn => todo!(),
        }
        ret
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " {}{}",
            match self.piece {
                PieceType::King => "K",
                PieceType::Queen => "Q",
                PieceType::Rook => "R",
                PieceType::Bishop => "B",
                PieceType::Knight => "N",
                PieceType::Pawn => "P",
            },
            match self.player {
                Player::White => " ",
                Player::Black => "*",
            }
        )
    }
}
