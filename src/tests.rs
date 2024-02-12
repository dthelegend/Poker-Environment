use rand::{Rng, SeedableRng, thread_rng};
use rand::prelude::StdRng;
use crate::rules::calculate_best_hand;
use crate::game::GameState;
use crate::game::Action;
use crate::game::Player;

const SEED: u64 = 12;

#[test]
fn test_all_call() {
    let test_players = vec![
        Player {
            player_id: "Player 1".to_string(),
            balance: 20
        },
        Player {
            player_id: "Player 2".to_string(),
            balance: 20
        },
        Player {
            player_id: "Player 3".to_string(),
            balance: 20
        },
        Player {
            player_id: "Player 4".to_string(),
            balance: 20
        },
        Player {
            player_id: "Player 5".to_string(),
            balance: 20
        },
        Player {
            player_id: "Player 6".to_string(),
            balance: 20
        }
    ];
    let test_players_len = test_players.len();
    let mut game = GameState::new_with_players(StdRng::seed_from_u64(SEED), test_players, 2);

    let mut x = 0;
    while {
        match game {
            GameState::BettingRound(betting_round) => {
                game = betting_round.update_state(Action::Call);

                true
            },
            _ => false
        }
    } {
        x += 1;
    }

    assert_eq!(test_players_len * 4, x);
    println!("Finished!")
}

#[test]
fn test_random() {
    const MIN_BET : usize = 2;

    let test_players = vec![
        Player {
            player_id: "Player 1".to_string(),
            balance: 20
        },
        Player {
            player_id: "Player 2".to_string(),
            balance: 20
        },
        Player {
            player_id: "Player 3".to_string(),
            balance: 20
        },
        Player {
            player_id: "Player 4".to_string(),
            balance: 20
        },
        // Player {
        //     player_id: "Player 5".to_string(),
        //     balance: 20
        // },
        // Player {
        //     player_id: "Player 6".to_string(),
        //     balance: 20
        // }
    ];

    let mut game = GameState::new_with_players(StdRng::seed_from_u64(SEED), test_players, 2);
    let mut rng = thread_rng();

    while {
        match game {
            GameState::BettingRound(betting_round) => {
                let i : f64 = rng.gen_range(0.0..=1.0);
                let action = if i > 0.8 {
                    Action::Raise(rng.gen_range(MIN_BET..(MIN_BET * 5)))
                } else if i > 0.01 {
                    Action::Call
                } else {
                    Action::Fold
                };

                println!("---\n{}\n\n{:?}", betting_round.get_environment(), action);

                game = betting_round.update_state(action);

                true
            },
            _ => false
        }
    } {
    }
    println!("Finished!");

    println!("---\nGame Info");
    print!("Table:\t");
    if let GameState::Finished(sd) = game {
        for card in &sd.table {
            print!("{}\t", card)
        }
        print!("\nPlayers:\t");
        for player in &sd.players.0 {
            print!("{}|{}({:?})\t", player.hand[0], player.hand[1], player.balance);
        }
        println!();
        let hand_list: Vec<_> = sd.players.0.iter().map(|p| calculate_best_hand(p.hand, &sd.table)).collect();
        for hand in &hand_list {
            print!("{:?}\t", hand);
        }
        println!();
        println!("---\nBest Hand: {:?}", &hand_list.iter().max().unwrap());
        println!("---\nNew Players: {:?}", sd.players);
        assert_eq!(sd.players.0.iter().map(|x| x.balance.1).sum::<usize>(), 0);
        assert_eq!(sd.players.0.iter().map(|x| x.expectation).sum::<usize>(), 0);
        assert_eq!(sd.players.0.iter().map(|x| x.balance.0).sum::<usize>() + sd.players.1.iter().map(|x| x.balance).sum::<usize>(), 4 * 20)
    }
}

#[test]
fn test_fold_bar_one() {
    let test_players = vec![
        Player {
            player_id: "Player 1".to_string(),
            balance: 20
        },
        Player {
            player_id: "Player 2".to_string(),
            balance: 20
        },
        Player {
            player_id: "Player 3".to_string(),
            balance: 20
        },
        Player {
            player_id: "Player 4".to_string(),
            balance: 20
        },
        Player {
            player_id: "Player 5".to_string(),
            balance: 20
        },
        Player {
            player_id: "Player 6".to_string(),
            balance: 20
        }
    ];
    let test_players_len = test_players.len();
    let mut game = GameState::new_with_players(StdRng::seed_from_u64(SEED), test_players, 2);

    for _ in 0..(test_players_len - 1) {
        match game {
            GameState::BettingRound(betting_round) => {
                game = betting_round.update_state(Action::Fold);
            },
            _ => panic!("Game finished before everyone folded!")
        }
    }
    while {
        match game {
            GameState::BettingRound(betting_round) => {
                game = betting_round.update_state(Action::Fold);

                true
            },
            _ => false
        }
    } {
        panic!("Game did not immediately finish after everyone folded!")
    }

    println!("Finished!")
}