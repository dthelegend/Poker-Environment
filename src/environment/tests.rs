use rand::thread_rng;
use super::*;
use uuid::Uuid;
#[test]
fn test_all_call() {
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
    let mut game = ActiveGame::new_with_players(players, 2);

    while !game.is_finished() {
        game = game.update_state(PokerActions::Call);
    }
    println!("Finished!")
}

#[test]
fn test_random() {
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

    const MIN_BET : usize = 2;
    let mut game = ActiveGame::new_with_players(players, MIN_BET);
    let mut rng = thread_rng();
    while !game.is_finished() {
        let i : f64 = rng.gen_range(0.0..=1.0);
        let action = if i > 0.8 {
            PokerActions::Raise(rng.gen_range(MIN_BET..(MIN_BET * 5)))
        } else if i > 0.01 {
            PokerActions::Call
        } else {
            PokerActions::Fold
        };

        let player = game.peek_next_player_info();
        println!("---\nPlayerId: {}\n Action: {:?}", player.uuid, action);
        game = game.update_state(action);

    }
    println!("Finished!");

    println!("---\nGame Info");
    print!("Table:\t");
    if let GameStage::Finished(table_cards) = game.game_stage {
        for card in &table_cards {
            print!("{}\t", card)
        }
        print!("\nPlayers:\t");
        for player in &game.players.1 {
            print!("{}|{}({})\t", player.cards[0], player.cards[1], player.current_bet);
        }
        println!();
        let hand_list: Vec<_> = game.players.1.iter().map(|p| calculate_best_hand(p.cards, &table_cards)).collect();
        for hand in &hand_list {
            print!("{:?}\t", hand);
        }
        println!("Pot: {}", &game.pot);
        println!("---");
        println!("Best Hand: {:?}", &hand_list.iter().max().unwrap())
    }
}