#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum AlgebraicCoord {
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,

    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,

    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,

    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,

    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,

    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,

    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
}

impl From<&AlgebraicCoord> for (usize, usize) {
    fn from(a: &AlgebraicCoord) -> Self {
        (*a).into()
    }
}

impl From<AlgebraicCoord> for (usize, usize) {
    fn from(a: AlgebraicCoord) -> Self {
        match a {
            AlgebraicCoord::A1 => (7, 0),
            AlgebraicCoord::A2 => (6, 0),
            AlgebraicCoord::A3 => (5, 0),
            AlgebraicCoord::A4 => (4, 0),
            AlgebraicCoord::A5 => (3, 0),
            AlgebraicCoord::A6 => (2, 0),
            AlgebraicCoord::A7 => (1, 0),
            AlgebraicCoord::A8 => (0, 0),

            AlgebraicCoord::B1 => (7, 1),
            AlgebraicCoord::B2 => (6, 1),
            AlgebraicCoord::B3 => (5, 1),
            AlgebraicCoord::B4 => (4, 1),
            AlgebraicCoord::B5 => (3, 1),
            AlgebraicCoord::B6 => (2, 1),
            AlgebraicCoord::B7 => (1, 1),
            AlgebraicCoord::B8 => (0, 1),

            AlgebraicCoord::C1 => (7, 2),
            AlgebraicCoord::C2 => (6, 2),
            AlgebraicCoord::C3 => (5, 2),
            AlgebraicCoord::C4 => (4, 2),
            AlgebraicCoord::C5 => (3, 2),
            AlgebraicCoord::C6 => (2, 2),
            AlgebraicCoord::C7 => (1, 2),
            AlgebraicCoord::C8 => (0, 2),

            AlgebraicCoord::D1 => (7, 3),
            AlgebraicCoord::D2 => (6, 3),
            AlgebraicCoord::D3 => (5, 3),
            AlgebraicCoord::D4 => (4, 3),
            AlgebraicCoord::D5 => (3, 3),
            AlgebraicCoord::D6 => (2, 3),
            AlgebraicCoord::D7 => (1, 3),
            AlgebraicCoord::D8 => (0, 3),

            AlgebraicCoord::E1 => (7, 4),
            AlgebraicCoord::E2 => (6, 4),
            AlgebraicCoord::E3 => (5, 4),
            AlgebraicCoord::E4 => (4, 4),
            AlgebraicCoord::E5 => (3, 4),
            AlgebraicCoord::E6 => (2, 4),
            AlgebraicCoord::E7 => (1, 4),
            AlgebraicCoord::E8 => (0, 4),

            AlgebraicCoord::F1 => (7, 5),
            AlgebraicCoord::F2 => (6, 5),
            AlgebraicCoord::F3 => (5, 5),
            AlgebraicCoord::F4 => (4, 5),
            AlgebraicCoord::F5 => (3, 5),
            AlgebraicCoord::F6 => (2, 5),
            AlgebraicCoord::F7 => (1, 5),
            AlgebraicCoord::F8 => (0, 5),

            AlgebraicCoord::G1 => (7, 6),
            AlgebraicCoord::G2 => (6, 6),
            AlgebraicCoord::G3 => (5, 6),
            AlgebraicCoord::G4 => (4, 6),
            AlgebraicCoord::G5 => (3, 6),
            AlgebraicCoord::G6 => (2, 6),
            AlgebraicCoord::G7 => (1, 6),
            AlgebraicCoord::G8 => (0, 6),

            AlgebraicCoord::H1 => (7, 7),
            AlgebraicCoord::H2 => (6, 7),
            AlgebraicCoord::H3 => (5, 7),
            AlgebraicCoord::H4 => (4, 7),
            AlgebraicCoord::H5 => (3, 7),
            AlgebraicCoord::H6 => (2, 7),
            AlgebraicCoord::H7 => (1, 7),
            AlgebraicCoord::H8 => (0, 7),
        }
    }
}

impl From<(usize, usize)> for AlgebraicCoord {
    fn from(a: (usize, usize)) -> Self {
        match a {
            (7, 0) => AlgebraicCoord::A1,
            (6, 0) => AlgebraicCoord::A2,
            (5, 0) => AlgebraicCoord::A3,
            (4, 0) => AlgebraicCoord::A4,
            (3, 0) => AlgebraicCoord::A5,
            (2, 0) => AlgebraicCoord::A6,
            (1, 0) => AlgebraicCoord::A7,
            (0, 0) => AlgebraicCoord::A8,

            (7, 1) => AlgebraicCoord::B1,
            (6, 1) => AlgebraicCoord::B2,
            (5, 1) => AlgebraicCoord::B3,
            (4, 1) => AlgebraicCoord::B4,
            (3, 1) => AlgebraicCoord::B5,
            (2, 1) => AlgebraicCoord::B6,
            (1, 1) => AlgebraicCoord::B7,
            (0, 1) => AlgebraicCoord::B8,

            (7, 2) => AlgebraicCoord::C1,
            (6, 2) => AlgebraicCoord::C2,
            (5, 2) => AlgebraicCoord::C3,
            (4, 2) => AlgebraicCoord::C4,
            (3, 2) => AlgebraicCoord::C5,
            (2, 2) => AlgebraicCoord::C6,
            (1, 2) => AlgebraicCoord::C7,
            (0, 2) => AlgebraicCoord::C8,

            (7, 3) => AlgebraicCoord::D1,
            (6, 3) => AlgebraicCoord::D2,
            (5, 3) => AlgebraicCoord::D3,
            (4, 3) => AlgebraicCoord::D4,
            (3, 3) => AlgebraicCoord::D5,
            (2, 3) => AlgebraicCoord::D6,
            (1, 3) => AlgebraicCoord::D7,
            (0, 3) => AlgebraicCoord::D8,

            (7, 4) => AlgebraicCoord::E1,
            (6, 4) => AlgebraicCoord::E2,
            (5, 4) => AlgebraicCoord::E3,
            (4, 4) => AlgebraicCoord::E4,
            (3, 4) => AlgebraicCoord::E5,
            (2, 4) => AlgebraicCoord::E6,
            (1, 4) => AlgebraicCoord::E7,
            (0, 4) => AlgebraicCoord::E8,

            (7, 5) => AlgebraicCoord::F1,
            (6, 5) => AlgebraicCoord::F2,
            (5, 5) => AlgebraicCoord::F3,
            (4, 5) => AlgebraicCoord::F4,
            (3, 5) => AlgebraicCoord::F5,
            (2, 5) => AlgebraicCoord::F6,
            (1, 5) => AlgebraicCoord::F7,
            (0, 5) => AlgebraicCoord::F8,

            (7, 6) => AlgebraicCoord::G1,
            (6, 6) => AlgebraicCoord::G2,
            (5, 6) => AlgebraicCoord::G3,
            (4, 6) => AlgebraicCoord::G4,
            (3, 6) => AlgebraicCoord::G5,
            (2, 6) => AlgebraicCoord::G6,
            (1, 6) => AlgebraicCoord::G7,
            (0, 6) => AlgebraicCoord::G8,

            (7, 7) => AlgebraicCoord::H1,
            (6, 7) => AlgebraicCoord::H2,
            (5, 7) => AlgebraicCoord::H3,
            (4, 7) => AlgebraicCoord::H4,
            (3, 7) => AlgebraicCoord::H5,
            (2, 7) => AlgebraicCoord::H6,
            (1, 7) => AlgebraicCoord::H7,
            (0, 7) => AlgebraicCoord::H8,

            _ => panic!("can't convert {a:?} into AlgebraicCoord"),
        }
    }
}
