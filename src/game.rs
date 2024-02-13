use std::array::IntoIter;
use std::cmp::min;
use itertools::Itertools;
use rand::Rng;
use playlist::Playlist;
use crate::rules::{calculate_best_hand, Card, Deck, Hand};

pub use player::*;
pub use environment::*;
pub use history::*;
pub use action::*;

mod player;
mod environment;
mod playlist;
mod history;
mod action;

#[derive(Clone)]
pub enum BettingRound<R: Rng + Sized> {
    PreFlop {
        deck: Deck<R>,
        play_list: Playlist<DealtPlayer>,
        bet: (usize, usize, usize),
        history: GameHistory
    },
    Flop {
        deck: Deck<R>,
        play_list: Playlist<DealtPlayer>,
        bet: (usize, usize, usize),
        table: [Card; 3],
        history: [GameHistory; 2]
    },
    Turn {
        deck: Deck<R>,
        play_list: Playlist<DealtPlayer>,
        bet: (usize, usize, usize),
        table: [Card; 4],
        history: [GameHistory; 3]
    },
    River {
        deck: Deck<R>,
        play_list: Playlist<DealtPlayer>,
        bet: (usize, usize, usize),
        table: [Card; 5],
        history: [GameHistory; 4]
    }
}

impl <R: Rng + Sized> BettingRound<R> {

    pub fn update_state(mut self, next_player_action: Action) -> GameState<R> {
        let (BettingRound::PreFlop { play_list, bet: (pot, expected_bet, minimum_bet), history, .. }
        | BettingRound::Flop { play_list, bet: (pot, expected_bet, minimum_bet), history: [.., history], .. }
        | BettingRound::Turn { play_list, bet: (pot, expected_bet, minimum_bet), history: [.., history], .. }
        | BettingRound::River { play_list, bet: (pot, expected_bet, minimum_bet), history: [.., history], .. }) = &mut self;

        // Perform the players action
        match next_player_action {
            Action::Raise(raise_amount) => {
                let should_reset = play_list.next(move |current_player| {
                    let (ref  mut player_remaining_balance, ref mut player_bet) = current_player.balance;
                    // Player cannot raise more than they have.
                    // Players who wish to stay in should call if they cannot afford in order to win the side pot
                    // Also force folds on inputs smaller than the minimum bet
                    // Raise gives a guarantee this is not the last player
                    if raise_amount + *expected_bet > *player_remaining_balance + *player_bet
                        || raise_amount < *minimum_bet
                    {
                        history.push(ActionHistory(current_player.player_id.clone(), Action::Fold));
                        false
                    } else {
                        *expected_bet += raise_amount;
                        let balance_change = *expected_bet - *player_bet;
                        *player_remaining_balance -= balance_change;
                        *player_bet = *expected_bet;
                        *pot += balance_change;
                        current_player.expectation = *pot;
                        history.push(ActionHistory(current_player.player_id.clone(), next_player_action));

                        true
                    }
                });

                if should_reset {
                    play_list.restart()
                }
            }
            Action::Call => {
                play_list.next(move |current_player| {
                    let (player_remaining_balance, player_bet) = &mut current_player.balance;
                    let raise_delta = *expected_bet - *player_bet;

                    let actual_raise = min(raise_delta, *player_remaining_balance);
                    if raise_delta > 0 {
                        current_player.expectation = *pot;
                    }
                    *player_remaining_balance -= actual_raise;
                    *player_bet += actual_raise;
                    *pot += actual_raise;

                    history.push(ActionHistory(current_player.player_id.clone(), next_player_action));

                    true
                });
            }
            Action::Fold => {
                play_list.next(|current_player| {
                    history.push(ActionHistory(current_player.player_id.clone(), next_player_action));
                    false
                });
            }
        }

        // Then check that the next player isn't the only player
        if play_list.len() == 1 {
            match self {
                BettingRound::PreFlop{ play_list, bet: (pot, expected_bet, ..), history, .. } => {
                    let (actives, folded) = play_list.into_lists();
                    GameState::Finished(
                        Showdown {
                            players: (distribute_pot(pot, &Vec::with_capacity(0), actives), folded.into_iter().map(|x| x.into()).collect()),
                            table: Vec::with_capacity(0),
                            bet: (pot, expected_bet),
                            history: vec![history]
                        }
                    )
                }
                BettingRound::Flop{ play_list, bet: (pot, expected_bet, ..), table, history, .. } => {
                    let (actives, folded) = play_list.into_lists();
                    GameState::Finished(
                        Showdown {
                            players: (distribute_pot(pot, &Vec::from(table), actives), folded.into_iter().map(|x| x.into()).collect()),
                            table: Vec::from(table),
                            bet: (pot, expected_bet),
                            history: Vec::from(history)
                        }
                    )
                }
                BettingRound::Turn{ play_list, bet: (pot, expected_bet, ..), table, history, .. } => {
                    let (actives, folded) = play_list.into_lists();
                    GameState::Finished(
                        Showdown {
                            players: (distribute_pot(pot, &Vec::from(table), actives), folded.into_iter().map(|x| x.into()).collect()),
                            table: Vec::from(table),
                            bet: (pot, expected_bet),
                            history: Vec::from(history)
                        }
                    )
                }
                BettingRound::River{ play_list, bet: (pot, expected_bet, _), table, history, .. } => {
                    let (actives, folded) = play_list.into_lists();
                    GameState::Finished(
                        Showdown {
                            players: (distribute_pot(pot, &Vec::from(table), actives), folded.into_iter().map(|x| x.into()).collect()),
                            table: Vec::from(table),
                            bet: (pot, expected_bet),
                            history: Vec::from(history)
                        }
                    )
                }
            }
        }
        // Otherwise check if we must proceed to the next round
        else if play_list.is_finished() {
            play_list.restart();

            match self {
                BettingRound::PreFlop { mut deck, play_list, bet, history } => {
                    let abc = deck.draw_n();
                    GameState::BettingRound(
                        BettingRound::Flop {
                            deck,
                            play_list,
                            table: abc,
                            bet,
                            history: [history, Vec::new()]
                        }
                    )
                }
                BettingRound::Flop{ mut deck, play_list, bet, table: [a,b,c], history: [h1, h2] } => {
                    let d = deck.draw();
                    let play_len = play_list.len();
                    GameState::BettingRound(
                        BettingRound::Turn {
                            deck,
                            play_list,
                            table: [a, b, c, d],
                            bet,
                            history: [h1,h2,Vec::with_capacity(play_len)]
                        }
                    )
                }
                BettingRound::Turn{ mut deck, play_list, bet, table: [a,b,c, d], history: [h1, h2, h3] } => {
                    let e = deck.draw();
                    let play_len = play_list.len();
                    GameState::BettingRound(
                        BettingRound::River {
                            deck,
                            play_list,
                            table: [a, b, c, d, e],
                            bet,
                            history: [h1,h2,h3,Vec::with_capacity(play_len)]
                        }
                    )
                }
                BettingRound::River{ play_list, bet: (pot, expected_bet, _), table, history, .. } => {
                    let (actives, folded) = play_list.into_lists();
                    GameState::Finished(
                        Showdown {
                            players: (distribute_pot(pot, &Vec::from(table), actives), folded.into_iter().map(|x| x.into()).collect()),
                            table: Vec::from(table),
                            bet: (pot, expected_bet),
                            history: Vec::from(history)
                        }
                    )
                }
            }
        }
        // If neither, continue with this round
        else {
            GameState::BettingRound(self)
        }
    }

    pub fn get_environment(&self) -> Environment {
        let (game_history, table_cards, pot): (Vec<GameHistory>, Vec<Card>, usize) = match self {
            BettingRound::PreFlop { history, bet: (pot, ..), .. } => (vec![history.clone()], Vec::with_capacity(0), *pot),
            BettingRound::Flop { history: [h1, h2], table, bet: (pot, ..), .. } => (Vec::from([h1.clone(),h2.clone()]), Vec::from(table), *pot),
            BettingRound::Turn { history: [h1, h2, h3], table, bet: (pot, ..), .. } => (vec![h1.clone(),h2.clone(), h3.clone()], Vec::from(table), *pot),
            BettingRound::River { history: [h1, h2, h3, h4], table, bet: (pot, ..), .. } => (vec![h1.clone(),h2.clone(), h3.clone(), h4.clone()], Vec::from(table), *pot)
        };

        let (current_player, player_states): (DealtPlayer, Vec<DealtPlayerVisible>) = {
            let (BettingRound::PreFlop { play_list, .. }
            | BettingRound::Flop { play_list, .. }
            | BettingRound::Turn { play_list, .. }
            | BettingRound::River { play_list, .. }) = self;

            let mut x = play_list.1.clone();
            let current_player = x.pop_front()
                .expect("There must always be at least one player");
            (current_player, x.into_iter().chain(play_list.0.iter().cloned()).map(|x| x.into()).collect())
        };

        Environment {
            table_cards,
            current_player,
            player_states,
            game_history,
            pot
        }
    }
}

impl <'a, R: Rng + Sized> BettingRound<R> {
    pub fn get_players(&'a self) -> Vec<DealtPlayer> {
        let (BettingRound::PreFlop { play_list, .. }
        | BettingRound::Flop { play_list, .. }
        | BettingRound::Turn { play_list, .. }
        | BettingRound::River { play_list, .. }) = self;

        play_list.clone().into_lists().0
    }
}

#[derive(Clone)]
pub enum GameState<R: Rng + Sized> {
    BettingRound(BettingRound<R>),
    Finished(Showdown)
}

impl <R: Rng + Sized> GameState<R> {
    pub fn new_with_players(rng: R, players: Vec<Player>, minimum_bet: usize) -> Self {
        let mut deck: Deck<R> = Deck::new_with_rng(rng);
        let n_players = players.len();

        let dealt_players: Vec<DealtPlayer> = players
            .into_iter().enumerate()
            .map(|(i, Player { player_id, balance}) | {
                let blind = std::cmp::max(0, i as isize - n_players as isize + 3) as usize * minimum_bet;
                let actual_blind = min(blind, balance);
                DealtPlayer {
                    player_id,
                    hand: deck.draw_n(),
                    balance: (balance - actual_blind, actual_blind),
                    expectation: actual_blind
                }
            })
            .collect();
        let pot = dealt_players.iter().map(|x| x.balance.1).sum();
        GameState::BettingRound(
            BettingRound::PreFlop {
                play_list: Playlist::new(dealt_players),
                deck,
                bet: (pot, minimum_bet * 2, minimum_bet),
                history: Vec::with_capacity(n_players)
            }
        )
    }
}

#[derive(Clone)]
pub struct Showdown {
    pub players: (Vec<DealtPlayer>, Vec<Player>),
    pub bet: (usize, usize),
    pub table: Vec<Card>,
    pub history: Vec<GameHistory>
}

pub fn distribute_pot(pot: usize, table_cards: &Vec<Card>, mut active_players: Vec<DealtPlayer>) -> Vec<DealtPlayer> {
    let winner_list: Vec<&mut DealtPlayer> = active_players.iter_mut().max_set_by_key(|x| x.hand);

    let sum_exp: usize = winner_list.iter().map(|x| x.expectation).sum();

    for winner in winner_list {
        let share = (winner.expectation * pot) / sum_exp;

        winner.balance.0 += share;
    }

    // Reset all the current bets and expectations for all the dealt players
    let all_players = active_players.into_iter().map(|mut x| {
        x.balance.1 = 0;
        x.expectation = 0;

        x
    }).collect();

    all_players
}

