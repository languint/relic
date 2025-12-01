use crate::board::{Board, defs::Square};

/// A move on a chess [`Board`]
/// FFFFTTTTTTSSSSSS
pub struct GameMove(u16);
impl GameMove {
    pub const FLAG_MASK: u16 = 0b1111000000000000;
    pub const TARGET_MASK: u16 = 0b0000111111000000;
    pub const SOURCE_MASK: u16 = 0b0000000000111111;

    pub const FLAG_SHIFT: usize = 12;
    pub const TARGET_SHIFT: usize = 6;
}

pub struct MoveFlags;
impl MoveFlags {
    pub const NONE: u16 = 0b0000;
    pub const EN_PASSANT: u16 = 0b0001;
    pub const CAPTURE: u16 = 0b0010;

    pub const PROMOTE_KNIGHT: u16 = 0b0100;
    pub const PROMOTE_BISHOP: u16 = 0b0101;
    pub const PROMOTE_ROOK: u16 = 0b0110;
    pub const PROMOTE_QUEEN: u16 = 0b0111;
}

impl GameMove {
    pub fn new(source: Square, target: Square, flags: u16) -> GameMove {
        let flag_part = flags << GameMove::FLAG_SHIFT;
        let source_part = source.index() as u16;
        let target_part = (target.index() as u16) << GameMove::TARGET_SHIFT;

        GameMove(flag_part | source_part | target_part)
    }
}

impl GameMove {
    #[inline]
    pub fn flag(&self) -> u16 {
        (self.0 & GameMove::FLAG_MASK) >> GameMove::FLAG_SHIFT
    }

    #[inline]
    pub fn target(&self) -> u16 {
        (self.0 & GameMove::TARGET_MASK) >> GameMove::TARGET_SHIFT
    }

    #[inline]
    pub fn source(&self) -> u16 {
        self.0 & GameMove::SOURCE_MASK
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::defs::Square,
        game::game_move::{GameMove, MoveFlags},
    };

    #[test]
    fn accessors() {
        let game_move = GameMove::new(Square::E2, Square::E5, MoveFlags::NONE);

        assert_eq!(game_move.0, 0b0000_100100_001100);
        assert_eq!(game_move.flag(), MoveFlags::NONE);
        assert_eq!(game_move.source(), Square::E2.index() as u16);
        assert_eq!(game_move.target(), Square::E5.index() as u16);
    }
}
