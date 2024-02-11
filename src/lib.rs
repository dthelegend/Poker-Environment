mod game;
mod rules;

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use pyo3::prelude::*;
use game::Player;
use game::Action;
use game::Action::{Call, Fold};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn poker_environment(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

#[pyclass]
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
//
// #[pyclass]
// struct PyPokerGame {
//     game: GameState
// }

#[derive(Debug)]
pub enum ActionParseError {
    RaiseAmountError,
    UnrecognisedCommand
}

impl Display for ActionParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse action. Reason: {}", match self {
            ActionParseError::RaiseAmountError => "RaiseAmountError",
            ActionParseError::UnrecognisedCommand => "UnrecognisedCommand"
        })
    }
}

impl Error for ActionParseError {
}

impl TryFrom<String> for Action {
    type Error = ActionParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_uppercase().split_once(" ") {
            Some(("CALL", "")) => Ok(Call),
            Some(("RAISE", a)) => {
                if let Ok(raise_amount) = a.trim().parse::<usize>() {
                    Ok(Action::Raise(raise_amount))
                } else {
                    Err(ActionParseError::RaiseAmountError)
                }
            },
            Some(("FOLD", "")) => Ok(Fold),
            _ => Err(ActionParseError::UnrecognisedCommand)
        }
    }
}

#[pyclass]
struct PyPokerGameState {

}

// #[pymethods]
// impl PyPokerGame {
//     #[new]
//     fn py_new(players: Vec<PyPlayerInfo>, minimum_bet: usize) -> Self {
//         Self {
//             game: GameState::new_with_players(players.into(), minimum_bet)
//         }
//     }
//
//     pub fn advance(&mut self, action: String) -> PyResult<bool> {
//         let action_parsed = action.try_into()
//             .map_err(|_| Err(PyErr::new::<PyValueError, _>("Failed to parse action")))?;
//
//         self.game = match &self.game {
//             GameState::BettingRound(br) => {
//                 br.update_state(action_parsed)
//             },
//             GameState::Finished(a) => {
//                 return Ok(true)
//             }
//         };
//
//         Ok(matches!(&self.game, GameState::Finished(_)))
//     }
//
//     pub fn get_players(&self) -> Vec<PyPlayerInfo> {
//         todo!()
//     }
// }
