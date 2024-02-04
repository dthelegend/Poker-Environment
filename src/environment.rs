use std::cmp::{min, Ordering};
use std::collections::VecDeque;
use rand;
use rand::Rng;
use rand::rngs::ThreadRng;
use uuid::Uuid;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum CardSuit {
    Hearts = 0,
    Diamonds = 1,
    Clubs = 2,
    Spades = 3,
}

enum CardColor {
    Red,
    Black
}

impl CardSuit {
    fn get_color(&self) -> CardColor {
        match self {
            CardSuit::Hearts | CardSuit::Diamonds => CardColor::Red,
            CardSuit::Clubs | CardSuit::Spades => CardColor::Black,
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum CardValue {
    Ace = 14,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13
}

#[derive(Eq, PartialEq)]
struct Card(CardSuit, CardValue);

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let &Card(own_suit, own_value) = self;
        let &Card(other_suit, other_value) = other;

        let order = own_value.cmp(&other_value);
        if matches!(order, Ordering::Equal) {
            own_suit.cmp(&other_suit)
        } else {
            order
        }
    }
}


struct PlayerInfo {
    uuid: Uuid,
    balance: usize,
}

struct DealtPlayer{
    player_info: PlayerInfo,
    cards: [Card; 2],
    current_bet: usize
}

struct Deck{
    rng: ThreadRng,
    taboo_list: Vec<Card>
}

impl Deck {
    fn new() -> Self {
        Self::new_with_rng(rand::thread_rng())
    }

    fn new_with_rng(rng: ThreadRng) -> Self {
        Self { rng, taboo_list: Vec::new() }
    }

    fn draw(&mut self) -> Card {
        let card = self.rng.gen();

        if self.taboo_list.contains(card) {
            self.draw()
        }
        else {
            card
        }
    }

    fn draw_n<const N: usize>(&mut self) -> [Card; N] {
        (0..N).map(|_| self.draw()).collect()
    }
}

enum GameStage {
    Dealt(Deck),
    Flop(Deck, [Card;3]),
    River(Deck, [Card;4]),
    Turn(Deck, [Card;5]),
    Finished(Vec<Card>) // We don't know how many cards will be on the table when we finish.
}

impl GameStage {
    fn advance(self) -> Self {
        match self {
            GameStage::Dealt(mut d) => GameStage::Flop(d, d.draw_n()),
            GameStage::Flop(mut d, table) => GameStage::River(d, [table] + d.draw_n()),
            GameStage::River(mut d, table) => GameStage::Turn(d, [table] + d.draw_n()),
            GameStage::Turn(_, table) => GameStage::Finished(Vec::from(table)),
            GameStage::Finished(_) => panic!("Cannot advance finished state")
        }
    }

    fn finish(self) -> Self {
        Self::Finished(match Self {
            GameStage::Dealt(_) => Vec::with_capacity(0),
            GameStage::Flop(_, table)
            | GameStage::River(_, table)
            | GameStage::Turn(_, table) => Vec::from(table),
            GameStage::Finished(a) => a
        })
    }
}

struct StartedGame {
    players: (VecDeque<DealtPlayer>, VecDeque<DealtPlayer>),
    game_stage: GameStage,
    minimum_bet: usize,
    expected_bet: usize,
    pot: usize
}

enum PokerActions {
    Raise(usize),
    Call,
    Fold
}

impl StartedGame {
    // Last two players in the list are always small and big blind
    fn new_with_players(players: Vec<PlayerInfo>, minimum_bet: usize) -> Self {
        let mut deck: Deck = Deck::new();

        StartedGame {
            players: players.into_iter().map(|p| DealtPlayer { player_info: p, cards: deck.draw_n(), current_bet: 0}).collect(),
            game_stage: GameStage::Dealt(deck),
            minimum_bet,
            expected_bet: minimum_bet,
            pot: 0
        }
    }

    fn update_state(mut self, next_player_action: PokerActions) -> Self {
        let Self{ players: (ref mut players_played, ref mut players_yet), minimum_bet, expected_bet, .. } = self;
        match next_player_action {
            PokerActions::Raise(raise_amount) => {
                let current_player = players_yet.front()
                    .expect("game should always have a next player");

                // Player cannot raise more than they have.
                // Players who wish to stay in should call if they cannot afford in order to win the side pot
                // Also force folds on inputs smaller than the minimum bet
                // Raise gives a guarantee this is not the last player
                return if raise_amount + expected_bet > current_player.player_info.balance + current_player.current_bet || raise_amount < minimum_bet {
                    self.update_state(PokerActions::Fold)
                }
                else {
                    self.expected_bet = expected_bet + raise_amount;
                    players_yet.append(players_played);
                    self.update_state(PokerActions::Call)
                }
            }
            PokerActions::Call => {
                let mut current_player = players_yet.pop_front()
                    .expect("game should always have a next player");

                let raise_delta =  expected_bet - current_player.current_bet;

                let actual_raise = min(raise_delta, current_player.player_info.balance);
                current_player.player_info.balance -= actual_raise;
                current_player.current_bet += actual_raise;

                players_played.push_back(current_player);
            }
            PokerActions::Fold => {
                let folded_player = players_yet.pop_front()
                    .expect("game should always have a next player");
                self.pot += folded_player.current_bet;
            }
        }

        if players_yet.len() == 0 {
            players_yet.append(players_played);
            if players_played.len() == 1 {
                self.game_stage = self.game_stage.finish();
            }
            else {
                self.game_stage = self.game_stage.advance();
            }
        }

        self
    }
}