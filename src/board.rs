use crate::{
    board::{bitboard::Bitboard, castling::CastlingRights},
    defs::NrOf,
};

pub mod bitboard;
pub mod castling;
pub mod defs;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Board {
    pub bb_pieces: [Bitboard; NrOf::PIECES],
    pub bb_colors: [Bitboard; NrOf::COLORS],
    pub castling_rights: CastlingRights,
}

impl Board {
    pub const EMPTY: Board = Board {
        bb_pieces: [Bitboard::EMPTY; NrOf::PIECES],
        bb_colors: [Bitboard::EMPTY; NrOf::COLORS],
        castling_rights: CastlingRights::NONE,
    };
}
