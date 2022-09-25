use std::fmt::Display;

use crate::piece::Piece;

#[derive(Clone, Copy)]
pub(crate) struct Square(Option<Piece>);

impl Square {
    pub(crate) fn new(p: Piece) -> Self {
        Self(Some(p))
    }

    pub(crate) fn none() -> Self {
        Self(None)
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(p) = self.0 {
            write!(f, "{}", p)
        } else {
            write!(f, "   ")
        }
    }
}
