use std::fmt::{Display, Formatter};
use crate::game::history::GameHistory;
use crate::game::player::{DealtPlayer, DealtPlayerVisible};
use crate::rules::Card;

#[derive(Debug)]
pub struct Environment {
    pub table_cards: Vec<Card>,
    pub current_player: DealtPlayer,
    pub player_states: Vec<DealtPlayerVisible>,
    pub game_history: Vec<GameHistory>
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Environment\
        Current Player:
        ")
    }
}
