use std::fmt::{Display, Formatter};
use itertools::Itertools;
use crate::game::history::GameHistory;
use crate::game::player::{DealtPlayer, DealtPlayerVisible};
use crate::rules::Card;

#[derive(Debug)]
pub struct Environment {
    pub table_cards: Vec<Card>,
    pub current_player: DealtPlayer,
    pub player_states: Vec<DealtPlayerVisible>,
    pub game_history: Vec<GameHistory>,
    pub pot: usize
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let table = self.table_cards.iter().map(|x| format!("{}", x)).join("|");
        write!(f, "\"{}\"'s Environment\n\tTable: |{}|\n\tPot: {}\n\tCards: |{}|{}|\n\tRem. Balance/Bet: {}/{}\n\tOther Players: {:?}",
               self.current_player.player_id,
               table,
               self.pot,
               self.current_player.hand[0], self.current_player.hand[1],
               self.current_player.balance.0, self.current_player.balance.1,
               self.player_states.iter().map(|x| x.balance).collect_vec(),
        )
    }
}
