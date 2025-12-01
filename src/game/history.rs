use crate::board::Board;

pub const MAX_HISTORY: usize = 1024;

pub struct History {
    pub boards: [Board; MAX_HISTORY],
    pub idx: usize,
}

impl Default for History {
    fn default() -> Self {
        Self {
            #[allow(clippy::large_stack_arrays)]
            boards: [Board::EMPTY; MAX_HISTORY],
            idx: 0,
        }
    }
}

impl History {
    pub fn push(&mut self, board: Board) -> Result<(), String> {
        if self.idx >= MAX_HISTORY {
            return Err(format!("Cannot push board to history of size {}", self.idx));
        }

        self.boards[self.idx] = board;
        self.idx += 1;

        Ok(())
    }

    pub fn peek(&self) -> Option<&Board> {
        if self.idx == 0 {
            None
        } else {
            Some(&self.boards[self.idx - 1])
        }
    }

    pub fn pop(&mut self) -> Option<Board> {
        if self.idx == 0 {
            return None;
        }
        self.idx -= 1;
        Some(self.boards[self.idx])
    }

    pub fn clear(&mut self) {
        self.idx = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{Board, bitboard::Bitboard, castling::CastlingRights},
        defs::NrOf,
        game::history::{History, MAX_HISTORY},
    };

    #[test]
    fn push() {
        let mut history = History::default();
        let board = Board {
            bb_colors: [Bitboard::FULL; NrOf::COLORS],
            bb_pieces: [Bitboard::FULL; NrOf::PIECES],
            castling_rights: CastlingRights::NONE,
        };

        history.push(board).expect("Push should succeed");

        assert_eq!(history.peek(), Some(&board));
        assert_eq!(history.idx, 1);
    }

    #[test]
    fn push_at_full() {
        let mut history = History::default();

        let board = Board {
            bb_colors: [Bitboard::FULL; NrOf::COLORS],
            bb_pieces: [Bitboard::FULL; NrOf::PIECES],
            castling_rights: CastlingRights::NONE,
        };

        history.idx = MAX_HISTORY;

        history.push(board).expect_err("Push should not succeed");

        assert_eq!(history.peek(), Some(&Board::EMPTY));
        assert_eq!(history.idx, MAX_HISTORY);
    }
}
