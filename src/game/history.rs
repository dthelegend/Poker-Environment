use crate::game::action::Action;

#[derive(Debug, Clone)]
pub struct ActionHistory(pub String, pub Action);

pub type GameHistory = Vec<ActionHistory>;
