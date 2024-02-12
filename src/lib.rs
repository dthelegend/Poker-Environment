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

#[pyclass]
#[derive(Clone)]
struct PyPokerPlayerInfo {
    #[pyo3(get)]
    player_id: String,
    #[pyo3(get)]
    balance: usize
}

#[pymethods]
impl PyPokerPlayerInfo {
    #[new]
    fn py_new(player_id: String, balance: usize) -> Self {
        PyPokerPlayerInfo {
            player_id,
            balance
        }
    }
}

impl From<PyPokerPlayerInfo> for Player {
    fn from(value: PyPokerPlayerInfo) -> Self {
        let PyPokerPlayerInfo { player_id, balance} = value;
        Player {player_id, balance}
    }
}

impl From<Player> for PyPokerPlayerInfo {
    fn from(value: Player) -> Self {
        let Player { player_id, balance} = value;
        PyPokerPlayerInfo {player_id, balance}
    }
}

#[pyclass]
struct PyPokerGame {
    game: GameState<StdRng>
}

#[pyclass]
#[derive(Clone)]
struct PyPokerDealtPlayer {
    #[pyo3(get)]
    player_id: String,
    #[pyo3(get)]
    remaining_balance: usize,
    #[pyo3(get)]
    committed_balance: usize,
    #[pyo3(get)]
    hand: Vec<String>
}

impl From<DealtPlayer> for PyPokerDealtPlayer {
    fn from(value: DealtPlayer) -> Self {
        PyPokerDealtPlayer {
            player_id: value.player_id,
            remaining_balance: value.balance.0,
            committed_balance: value.balance.1,
            hand: value.hand.into_iter().map(|x| format!("{}", x)).collect()
        }
    }
}

#[pyclass]
#[derive(Clone)]
struct PyPokerDealtPlayerVisible {
    #[pyo3(get)]
    player_id: String,
    #[pyo3(get)]
    remaining_balance: usize,
    #[pyo3(get)]
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
#[derive(Clone)]
struct PyPokerActionHistory {
    #[pyo3(get)]
    player_id: String,
    #[pyo3(get)]
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
#[derive(Clone)]
struct PyPokerEnvironment {
    #[pyo3(get)]
    table_cards: Vec<String>,
    #[pyo3(get)]
    current_player: PyPokerDealtPlayer,
    #[pyo3(get)]
    player_states: Vec<PyPokerDealtPlayerVisible>,
    #[pyo3(get)]
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
    fn py_new(players: Vec<PyPokerPlayerInfo>, minimum_bet: usize, seed: u64) -> Self {
        Self {
            game: GameState::new_with_players(StdRng::seed_from_u64(seed), players.into_iter().map(|x| x.into()).collect(), minimum_bet)
        }
    }

    fn advance(&mut self, action: String) -> PyResult<()> {
        let action_parsed = action.try_into()
            .map_err(|_| PyErr::new::<PyValueError, _>("Failed to parse action"))?;

        if let GameState::BettingRound(br) = &self.game {
            self.game = br.clone().update_state(action_parsed)
        }

        Ok(())
    }

    fn is_finished(&self) -> PyResult<bool> {
        Ok(matches!(self.game, GameState::Finished(_)))
    }

    fn get_environment(&self) -> PyResult<PyPokerEnvironment> {
        match &self.game {
            GameState::BettingRound(a) => Ok(a.get_environment().into()),
            GameState::Finished(_) => Err(PyErr::new::<PyValueError, _>("Cannot get environment of finished game!"))
        }
    }

    fn get_players(&self) -> PyResult<(Vec<PyPokerDealtPlayer>, Vec<PyPokerPlayerInfo>)> {
        match &self.game {
            GameState::BettingRound(br) => Ok((Vec::with_capacity(0), br.get_players().into_iter().map(|x| Player::from(x).into()).collect())),
            GameState::Finished(s) => Ok((s.players.0.iter().map(|x| PyPokerDealtPlayer::from(x.clone())).collect(), s.players.1.iter().map(|x| PyPokerPlayerInfo::from(x.clone())).collect()))
        }
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn poker_environment(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPokerPlayerInfo>()?;
    m.add_class::<PyPokerGame>()?;
    m.add_class::<PyPokerDealtPlayer>()?;
    m.add_class::<PyPokerDealtPlayerVisible>()?;
    m.add_class::<PyPokerActionHistory>()?;
    m.add_class::<PyPokerEnvironment>()?;

    Ok(())
}
