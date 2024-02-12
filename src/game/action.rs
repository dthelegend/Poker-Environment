use std::fmt::{Display, Formatter};
use std::error::Error;
use crate::game::Action::{Call, Fold};

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Raise(usize),
    Call,
    Fold,
}

impl Action {
    pub fn to_string(&self) -> String {
        match self {
            Action::Raise(a) => format!("RAISE {}", a),
            Call => "CALL".to_string(),
            Fold => "FOLD".to_string()
        }
    }
}

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
        let command_uppercase = value.to_uppercase();
        let mut command_parts = command_uppercase.split_whitespace();
        match (command_parts.next(), command_parts.next()) {
            (Some("CALL"), None) => Ok(Call),
            (Some("RAISE"), Some(a)) => {
                if let Ok(raise_amount) = a.trim().parse::<usize>() {
                    Ok(Action::Raise(raise_amount))
                } else {
                    Err(ActionParseError::RaiseAmountError)
                }
            },
            (Some("FOLD"), None) => Ok(Fold),
            _ => Err(ActionParseError::UnrecognisedCommand)
        }
    }
}
