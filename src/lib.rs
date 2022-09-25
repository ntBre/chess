use coord::AlgebraicCoord;

use crate::piece::Piece;

pub mod board;
mod coord;
mod piece;
mod square;

#[cfg(test)]
mod tests;

/// a [Piece] and an [AlgebraicCoord] it can legally move to
#[derive(PartialEq, Debug)]
pub(crate) struct Move {
    piece: Piece,
    coord: AlgebraicCoord,
}

impl Move {
    pub(crate) fn new<T: Into<AlgebraicCoord>>(piece: Piece, coord: T) -> Self {
        Self {
            piece,
            coord: coord.into(),
        }
    }
}
