#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl TryFrom<char> for Piece {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        const MAP: [Option<Piece>; 26] = {
            let mut m = [None; 26];
            m['p' as usize - 97] = Some(Piece::Pawn);
            m['n' as usize - 97] = Some(Piece::Knight);
            m['b' as usize - 97] = Some(Piece::Bishop);
            m['r' as usize - 97] = Some(Piece::Rook);
            m['q' as usize - 97] = Some(Piece::Queen);
            m['k' as usize - 97] = Some(Piece::King);
            m
        };

        let c = value.to_ascii_lowercase();
        let idx = (c as u8).wrapping_sub(b'a');
        if idx < 26 {
            MAP[idx as usize].ok_or("invalid piece character")
        } else {
            Err("invalid piece character")
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Rank { R1, R2, R3, R4, R5, R6, R7, R8 }

impl From<u8> for Rank {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value / 8) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl From<u8> for File {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value % 8) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
#[repr(u8)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub fn new(file: File, rank: Rank) -> Self {
        let index = (rank as u8) * 8 + (file as u8);
        unsafe { std::mem::transmute::<u8, Square>(index) }
    }

    pub fn file(self) -> File {
        File::from(u8::from(self))
    }

    pub fn rank(self) -> Rank {
        Rank::from(u8::from(self))
    }
}

impl From<Square> for u8 {
    fn from(value: Square) -> u8 {
        value as u8
    }
}

impl TryFrom<u8> for Square {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 63 {
            Ok(unsafe { std::mem::transmute::<u8, Square>(value) })
        } else {
            Err(format!("'{value}' is not a valid square index"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::defs::{File, Piece, Rank, Square};

    #[test]
    fn test_piece_try_from_valid() {
        assert_eq!(Piece::try_from('p').unwrap(), Piece::Pawn);
        assert_eq!(Piece::try_from('N').unwrap(), Piece::Knight);
        assert_eq!(Piece::try_from('b').unwrap(), Piece::Bishop);
        assert_eq!(Piece::try_from('R').unwrap(), Piece::Rook);
        assert_eq!(Piece::try_from('q').unwrap(), Piece::Queen);
        assert_eq!(Piece::try_from('K').unwrap(), Piece::King);
    }

    #[test]
    fn test_piece_try_from_invalid() {
        assert!(Piece::try_from('x').is_err());
        assert!(Piece::try_from('1').is_err());
        assert!(Piece::try_from(' ').is_err());
    }

    #[test]
    fn test_file_roundtrip() {
        for i in 0..64 {
            let file = File::from(i);
            assert_eq!(file as u8, i % 8);
        }
    }

    #[test]
    fn test_rank_roundtrip() {
        for i in 0..64 {
            let rank = Rank::from(i);
            assert_eq!(rank as u8, i / 8);
        }
    }

    #[test]
    fn test_square_try_from_valid() {
        for i in 0u8..=63 {
            let sq = Square::try_from(i).unwrap();
            assert_eq!(u8::from(sq), i);
        }
    }

    #[test]
    fn test_square_try_from_invalid() {
        assert!(Square::try_from(64u8).is_err());
        assert!(Square::try_from(100u8).is_err());
    }
}
