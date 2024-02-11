mod game;
mod rules;

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt::{Debug, Display};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rand::prelude::StdRng;
use rand::SeedableRng;
use game::Player;
use crate::game::{ActionHistory, DealtPlayer, DealtPlayerVisible, Environment, GameState};
use crate::rules::Card;

#[pyclass]
#[derive(Clone)]
struct PyPlayerInfo {
    player_id: String,
    balance: usize
}

impl From<PyPlayerInfo> for Player {
    fn from(value: PyPlayerInfo) -> Self {
        let PyPlayerInfo { player_id, balance} = value;
        Player {player_id, balance}
    }
}

impl From<Player> for PyPlayerInfo {
    fn from(value: Player) -> Self {
        let Player { player_id, balance} = value;
        PyPlayerInfo {player_id, balance}
    }
}

#[pyclass]
struct PyPokerGame {
    game: GameState<StdRng>
}

#[pyclass]
struct PyPokerDealtPlayer {
    player_id: String,
    remaining_balance: usize,
    committed_balance: usize,
    hand: Vec<Card>
}

impl From<DealtPlayer> for PyPokerDealtPlayer {
    fn from(value: DealtPlayer) -> Self {
        PyPokerDealtPlayer {
            player_id: value.player_id,
            remaining_balance: value.balance.0,
            committed_balance: value.balance.1,
            hand: Vec::from(value.hand)
        }
    }
}

#[pyclass]
struct PyPokerDealtPlayerVisible {
    player_id: String,
    remaining_balance: usize,
    committed_balance: usize
}

impl From<DealtPlayerVisible> for PyPokerDealtPlayerVisible {
    fn from(value: DealtPlayerVisible) -> Self {
        PyPokerDealtPlayerVisible {
            player_id: value.player_id,
            remaining_balance: value.balance.0,
            committed_balance: value.balance.1
        }
    }
}

type PyPokerGameHistory = Vec<Vec<PyPokerActionHistory>>;

#[pyclass]
struct PyPokerActionHistory {
    player_id: String,
    action: String
}

impl From<ActionHistory> for PyPokerActionHistory {
    fn from(value: ActionHistory) -> Self {
        PyPokerActionHistory {
            player_id: value.0,
            action: value.1.to_string()
        }
    }
}

#[pyclass]
struct PyPokerEnvironment {
    table_cards: Vec<String>,
    current_player: PyPokerDealtPlayer,
    player_states: Vec<PyPokerDealtPlayerVisible>,
    game_history: PyPokerGameHistory
}

impl From<Environment> for PyPokerEnvironment {
    fn from(value: Environment) -> Self {
        Self {
            table_cards: value.table_cards.into_iter().map(|x| format!("{}", x)).collect(),
            current_player: value.current_player.into(),
            player_states: value.player_states.into_iter().map(|x| x.into()).collect(),
            game_history: value.game_history.into_iter().map(|x| x.into_iter().map(|x| x.into()).collect()).collect(),
        }
    }
}

#[pymethods]
impl PyPokerGame {
    #[new]
    fn py_new(players: Vec<PyPlayerInfo>, minimum_bet: usize, seed: u64) -> Self {
        Self {
            game: GameState::new_with_players(StdRng::seed_from_u64(seed), players.into_iter().map(|x| x.into()).collect(), minimum_bet)
        }
    }

    pub fn advance(&mut self, action: String) -> PyResult<Option<Vec<PyPlayerInfo>>> {
        let action_parsed = action.try_into()
            .map_err(|_| PyErr::new::<PyValueError, _>("Failed to parse action"))?;

        self.game = match self.game.clone() {
            GameState::BettingRound(br) => {
                br.update_state(action_parsed)
            },
            GameState::Finished(a) => {
                return Ok(Some(a.calculate_players().into_iter().map(|x| x.into()).collect()))
            }
        };

        Ok(None)
    }

    pub fn get_environment(&self) -> PyResult<Option<PyPokerEnvironment>> {
        Ok(match &self.game {
            GameState::BettingRound(a) => Some(a.get_environment().into()),
            GameState::Finished(_) => None
        })
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn poker_environment(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPlayerInfo>()?;
    m.add_class::<PyPokerGame>()?;
    m.add_class::<PyPokerDealtPlayer>()?;
    m.add_class::<PyPokerDealtPlayerVisible>()?;
    m.add_class::<PyPokerActionHistory>()?;
    m.add_class::<PyPokerEnvironment>()?;

    Ok(())
}
