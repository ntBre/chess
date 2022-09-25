use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Player {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug)]
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
