use std::fmt::Display;

use crate::piece::Piece;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Square {
    pub(crate) piece: Option<Piece>,
}

impl Square {
    pub(crate) fn new(p: Piece) -> Self {
        Self { piece: Some(p) }
    }

    pub(crate) fn none() -> Self {
        Self { piece: None }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(p) = self.piece {
            write!(f, "{}", p)
        } else {
            write!(f, "   ")
        }
    }
}
