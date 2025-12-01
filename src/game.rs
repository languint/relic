pub mod history;

use crate::{board::Board, game::history::History};

pub struct Game {
    pub board: Board,
    pub history: History,
}
