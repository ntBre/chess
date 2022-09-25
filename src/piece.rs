use std::fmt::Display;

#[derive(Clone, Copy)]
pub(crate) enum Player {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub(crate) enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy)]
pub(crate) struct Piece {
    #[allow(unused)]
    pub(crate) owner: Player,
    pub(crate) piece: PieceType,
}

impl Piece {
    pub(crate) fn new(owner: Player, piece: PieceType) -> Self {
        Self { owner, piece }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:^3}",
            match self.piece {
                PieceType::King => "K",
                PieceType::Queen => "Q",
                PieceType::Rook => "R",
                PieceType::Bishop => "B",
                PieceType::Knight => "N",
                PieceType::Pawn => "P",
            }
        )
    }
}
