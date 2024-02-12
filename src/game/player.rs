use crate::rules::Card;

#[derive(Debug, Clone)]
pub struct DealtPlayer {
    pub player_id: String,
    pub balance: (usize, usize),
    pub hand: [Card; 2],
    pub expectation: usize
}

#[derive(Debug, Clone)]
pub struct Player {
    pub player_id: String,
    pub balance: usize
}

impl From<DealtPlayer> for Player {
    fn from(value: DealtPlayer) -> Self {
        let DealtPlayer {
            player_id,
            balance: (balance, _),
            ..
        } = value;

        Player {
            balance,
            player_id
        }
    }
}

#[derive(Debug)]
pub struct DealtPlayerVisible {
    pub player_id: String,
    pub balance: (usize, usize)
}

impl From<DealtPlayer> for DealtPlayerVisible {
    fn from(value: DealtPlayer) -> Self {
        let DealtPlayer {
            player_id,
            balance,
            ..
        } = value;

        DealtPlayerVisible {
            balance,
            player_id
        }
    }
}
