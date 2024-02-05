use super::*;
use uuid::Uuid;
#[test]
fn test_poker_game() {
    let players = vec![
        PlayerInfo {
            uuid: Uuid::new_v4(),
            balance: 20
        },
        PlayerInfo {
            uuid: Uuid::new_v4(),
            balance: 20
        },
        PlayerInfo {
            uuid: Uuid::new_v4(),
            balance: 20
        },
        PlayerInfo {
            uuid: Uuid::new_v4(),
            balance: 20
        },
        PlayerInfo {
            uuid: Uuid::new_v4(),
            balance: 20
        },
        PlayerInfo {
            uuid: Uuid::new_v4(),
            balance: 20
        }
    ];
    let game = StartedGame::new_with_players(players, 2);
}