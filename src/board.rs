use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::{piece::Piece, piece::PieceType, piece::Player, square::Square};

use crate::coord::AlgebraicCoord;

pub(crate) struct Board([[Square; 8]; 8]);

impl Board {
    pub(crate) fn starting_position() -> Self {
        use PieceType::*;
        use Player::*;
        Self([
            [
                Square::new(Piece::new(Black, Rook)),
                Square::new(Piece::new(Black, Knight)),
                Square::new(Piece::new(Black, Bishop)),
                Square::new(Piece::new(Black, Queen)),
                Square::new(Piece::new(Black, King)),
                Square::new(Piece::new(Black, Bishop)),
                Square::new(Piece::new(Black, Knight)),
                Square::new(Piece::new(Black, Rook)),
            ],
            [Square::new(Piece::new(Black, Pawn)); 8],
            [Square::none(); 8],
            [Square::none(); 8],
            [Square::none(); 8],
            [Square::none(); 8],
            [Square::new(Piece::new(White, Pawn)); 8],
            [
                Square::new(Piece::new(White, Rook)),
                Square::new(Piece::new(White, Knight)),
                Square::new(Piece::new(White, Bishop)),
                Square::new(Piece::new(White, Queen)),
                Square::new(Piece::new(White, King)),
                Square::new(Piece::new(White, Bishop)),
                Square::new(Piece::new(White, Knight)),
                Square::new(Piece::new(White, Rook)),
            ],
        ])
    }

    /// evaluate the position represented by `self`
    pub(crate) fn evaluate(&self) -> f64 {
        self.pieces().iter().map(|p| p.value()).sum()
    }

    /// return an iterator over the pieces in `self`
    pub(crate) fn pieces(&self) -> Vec<Piece> {
        let mut v = Vec::with_capacity(64);
        for row in self.0 {
            v.extend(row.iter().filter_map(|s| s.piece));
        }
        v
    }
}

impl Index<AlgebraicCoord> for Board {
    type Output = Square;

    fn index(&self, index: AlgebraicCoord) -> &Self::Output {
        let (i, j) = index.into();
        &self.0[i][j]
    }
}

impl IndexMut<AlgebraicCoord> for Board {
    fn index_mut(&mut self, index: AlgebraicCoord) -> &mut Self::Output {
        let (i, j) = index.into();
        &mut self.0[i][j]
    }
}

impl Display for Board {
    /// display a [Board] by looping over the component [Square]s and
    /// deferring to their `Display` implementation
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bar = ['\u{2500}'; 31];
        let s: String = bar.into_iter().collect();
        writeln!(f, "\u{250c}{}\u{2510}", s)?;
        for (row, rank) in self.0.into_iter().enumerate() {
            write!(f, "\u{2502}")?;
            for file in rank {
                write!(f, "{}\u{2502}", file)?;
            }
            writeln!(f)?;
            if row < 7 {
                writeln!(f, "\u{2502}{}\u{2502}", s)?;
            }
        }
        writeln!(f, "\u{2514}{}\u{2518}", s)?;
        Ok(())
    }
}
