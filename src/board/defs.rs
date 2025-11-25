pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl TryFrom<char> for Piece {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let value = value.to_ascii_lowercase();

        match value {
            'p' => Ok(Piece::Pawn),
            'n' => Ok(Piece::Knight),
            'b' => Ok(Piece::Bishop),
            'r' => Ok(Piece::Rook),
            'q' => Ok(Piece::Queen),
            'k' => Ok(Piece::King),
            _ => Err(format!("'{value}' is not a valid piece!'")),
        }
    }
}

pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl<T> From<T> for Rank
where
    T: Into<u8>,
{
    fn from(value: T) -> Self {
        match value.into() / 8 {
            0 => Rank::R1,
            1 => Rank::R2,
            2 => Rank::R3,
            3 => Rank::R4,
            4 => Rank::R5,
            5 => Rank::R6,
            6 => Rank::R7,
            7 => Rank::R8,
            _ => unreachable!(),
        }
    }
}

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

impl<T> From<T> for File
where
    T: Into<u8>,
{
    fn from(value: T) -> Self {
        match value.into() % 8 {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        }
    }
}

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
        unsafe { Square::try_from(index).unwrap_unchecked() }
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
