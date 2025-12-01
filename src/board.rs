use crate::{
    board::{bitboard::Bitboard, castling::CastlingRights},
    defs::NrOf,
};

pub mod bitboard;
pub mod castling;
pub mod defs;

pub struct Board {
    pub bb_pieces: [Bitboard; NrOf::PIECES],
    pub bb_colors: [Bitboard; NrOf::COLORS],
    pub castling_rights: CastlingRights,
}
