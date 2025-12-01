use crate::board::defs::Square;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Bitboard(u64);
impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u64::MAX);
}

impl Bitboard {
    #[inline]
    pub fn square_mask(square: Square) -> Bitboard {
        Bitboard::from(1u64 << square.index())
    }

    #[inline]
    pub fn get_square(self, square: Square) -> Bitboard {
        Self::square_mask(square) & self
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAnd for &Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOr for &Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::defs::Square;

    #[test]
    fn square_mask_produces_correct_bit() {
        let a1 = Square::A1;
        let h8 = Square::H8;

        assert_eq!(Bitboard::square_mask(a1).0, 1u64 << a1.index());
        assert_eq!(Bitboard::square_mask(h8).0, 1u64 << h8.index());
    }

    #[test]
    fn get_square_returns_masked_value() {
        let mut bb = Bitboard::from(0);
        bb = bb | Bitboard::square_mask(Square::A1);
        bb = bb | Bitboard::square_mask(Square::C3);

        assert_eq!(bb.get_square(Square::A1).0, 1u64 << Square::A1.index());
        assert_eq!(bb.get_square(Square::C3).0, 1u64 << Square::C3.index());
        assert_eq!(bb.get_square(Square::H8).0, 0);
    }

    #[test]
    fn bitand_works() {
        let a1 = Bitboard::square_mask(Square::A1);
        let b1 = Bitboard::square_mask(Square::B1);

        let both = &a1 | &b1;

        assert_eq!((&both & &a1).0, a1.0);
        assert_eq!((&both & &b1).0, b1.0);
        assert_eq!((a1 & b1).0, 0);
    }

    #[test]
    fn bitor_works() {
        let a1 = Bitboard::square_mask(Square::A1);
        let b1 = Bitboard::square_mask(Square::B1);

        let combined = &a1 | &b1;
        assert_eq!(combined.0, &a1.0 | b1.0);
    }

    #[test]
    fn from_u64_initializes_correctly() {
        let bb = Bitboard::from(0xF0F0_F0F0_F0F0_F0F0);
        assert_eq!(bb.0, 0xF0F0_F0F0_F0F0_F0F0);
    }
}
