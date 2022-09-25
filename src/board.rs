use std::fmt::Display;

use crate::{
    piece::Piece, piece::PieceType, piece::Player, square::Square,
};

pub(crate) struct Board([[Square; 8]; 8]);

impl Board {
    pub(crate) fn starting_position() -> Self {
        use PieceType::*;
        use Player::*;
        Self([
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
            [Square::new(Piece::new(White, Pawn)); 8],
            [Square::none(); 8],
            [Square::none(); 8],
            [Square::none(); 8],
            [Square::none(); 8],
            [Square::new(Piece::new(Black, Pawn)); 8],
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
        ])
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
