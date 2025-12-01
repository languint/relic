#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CastlingRights(u8);
impl CastlingRights {
    pub const WK: CastlingRights = CastlingRights(0b0001);
    pub const WQ: CastlingRights = CastlingRights(0b0010);
    pub const BK: CastlingRights = CastlingRights(0b0100);
    pub const BQ: CastlingRights = CastlingRights(0b1000);

    pub const ALL: CastlingRights = CastlingRights(0b1111);
    pub const NONE: CastlingRights = CastlingRights(0b0000);
}

impl TryFrom<&str> for CastlingRights {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut rights = 0b0000;

        for char in value.chars() {
            match char {
                'K' => rights |= CastlingRights::WK.0,
                'Q' => rights |= CastlingRights::WQ.0,
                'k' => rights |= CastlingRights::BK.0,
                'q' => rights |= CastlingRights::BQ.0,
                _ => return Err(format!("{char} is not a valid castling right")),
            }
        }

        Ok(CastlingRights(rights))
    }
}

#[cfg(test)]
mod tests {
    use crate::board::castling::CastlingRights;

    #[test]
    fn all_from_str() {
        assert_eq!(CastlingRights::try_from("kqKQ"), Ok(CastlingRights::ALL));
    }
}
