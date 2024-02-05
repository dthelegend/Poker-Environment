use std::any::Any;
use std::cmp::{min, Ordering};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::iter::zip;
use itertools::{Itertools};
use rand;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use rand::rngs::ThreadRng;
use uuid::Uuid;
use crate::environment::Hand::{Flush, Straight, StraightFlush};

#[cfg(test)]
mod tests;

#[derive(Ord, Eq, PartialEq, PartialOrd, Copy, Clone, Debug)]
enum CardSuit {
    Hearts = 0,
    Diamonds = 1,
    Clubs = 2,
    Spades = 3,
}

impl Distribution<CardSuit> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardSuit {
        match rng.gen_range(0..4) {
            0 => CardSuit::Hearts,
            1 => CardSuit::Diamonds,
            2 => CardSuit::Clubs,
            _ => CardSuit::Spades
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Hash)]
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

impl Distribution<CardValue> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardValue {
        match rng.gen_range(2..15) {
            2 => CardValue::Two,
            3 => CardValue::Three,
            4 => CardValue::Four,
            5 => CardValue::Five,
            6 => CardValue::Six,
            7 => CardValue::Seven,
            8 => CardValue::Eight,
            9 => CardValue::Nine,
            10 => CardValue::Ten,
            11 => CardValue::Jack,
            12 => CardValue::Queen,
            13 => CardValue::King,
            _ => CardValue::Ace,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Card(CardSuit, CardValue);

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl Display for CardSuit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CardSuit::Hearts => "H",
            CardSuit::Diamonds => "D",
            CardSuit::Clubs => "C",
            CardSuit::Spades => "S"
        };
        write!(f, "{}", s)
    }
}

impl Display for CardValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CardValue::Ace => "A".to_string(),
            CardValue::King => "K".to_string(),
            CardValue::Queen => "Q".to_string(),
            CardValue::Jack => "J".to_string(),
            a => (*a as isize).to_string()
        };
        write!(f, "{}", s)
    }
}

impl Distribution<Card> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        Card(rng.gen(), rng.gen())
    }
}

impl Eq for Card {}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(isize)]
enum Hand {
    // High Card
    StraightFlush(Card) = 9,
    // FourKind, HighCard
    FourOfAKind(CardValue, Card) = 8,
    // Big House, Small House
    FullHouse(CardValue, CardValue) = 7,
    // High Card
    Flush(Card) = 6,
    // High Card
    Straight(Card) = 5,
    // Kind, High Card
    ThreeOfAKind(CardValue, Card) = 4,
    // Pair, Pair, High Card
    TwoPair(CardValue, CardValue, Card) = 3,
    // Pair, High Card
    Pair(CardValue, Card) = 2,
    HighCard(Card) = 1
}

impl Hand {
    fn discriminant(&self) -> isize {
        unsafe { *(self as *const Self as *const isize) }
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Eq for Hand {
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (StraightFlush(a_v), StraightFlush(b_v))
            | (Flush(a_v), Flush(b_v))
            | (Straight(a_v), Straight(b_v))
            | (Hand::HighCard(a_v), Hand::HighCard(b_v)) => a_v.cmp(b_v),
            (Hand::FourOfAKind(a_v, a_high), Hand::FourOfAKind(b_v, b_high))
            | (Hand::ThreeOfAKind(a_v, a_high), Hand::ThreeOfAKind(b_v, b_high))
            | (Hand::Pair(a_v, a_high), Hand::Pair(b_v, b_high))=> {
                let cmp = a_v.cmp(b_v);

                match cmp {
                    Ordering::Equal => a_high.cmp(b_high),
                    a => a
                }
            }
            (Hand::FullHouse(aa_v, ab_v), Hand::FullHouse(ba_v, bb_v)) => {
                match aa_v.cmp(ba_v) {
                    Ordering::Equal => ab_v.cmp(bb_v),
                    a => a
                }
            },
            (Hand::TwoPair(aa_v, ab_v, a_high), Hand::TwoPair(ba_v, bb_v, b_high)) => {
                match aa_v.cmp(ba_v) {
                    Ordering::Equal => match ab_v.cmp(bb_v) {
                        Ordering::Equal => a_high.cmp(b_high),
                        a => a
                    },
                    a => a
                }
            },
            (a, b) => a.discriminant().cmp(&b.discriminant())
        }
    }
}

fn calculate_hand(mut hand: Vec<Card>) -> Hand {
    assert!(!hand.is_empty());

    hand.sort();

    let is_straight_ace_high: bool
        = zip(
            hand.iter().take(hand.len() - 1),
            hand.iter().skip(1))
        .all(|(a,b)| a.1 as isize + 1 == b.1 as isize);
    let hand_ace_low = hand.iter().map(|x| match x.1 {
        CardValue::Ace => 1,
        a => a as isize
    });
    // TODO This could be way more efficient
    let is_straight_ace_low
        = zip(
            hand_ace_low.clone().take(hand_ace_low.len() - 1),
            hand_ace_low.skip(1))
        .all(|(a,b)| a + 1 == b);
    let is_straight = is_straight_ace_low || is_straight_ace_high;

    let is_flush: bool = hand.iter().map(|x| x.0).all_equal();

    let high_card : Card = *hand.last().expect("Hand cannot have been empty");

    match (is_straight, is_flush, hand.len()) {
        (true, true, 5) => StraightFlush(high_card),
        (true, false, 5) => Straight(high_card),
        (false, true, 5) => Flush(high_card),
        (_, _, _hand_len) => {
            let hand_values = hand.iter().counts_by(|c| c.1);
            let mut hand_value_order: Vec<_> = hand_values.into_iter().sorted_by_key(|(_, count)| *count).collect();

            if let Some((first_highest_value, first_highest_count)) = hand_value_order.pop() {
                match first_highest_count {
                    1 => Hand::HighCard(high_card),
                    2 => {
                        if let Some((sec_highest_value, 2)) = hand_value_order.pop() {
                            Hand::TwoPair(first_highest_value, sec_highest_value, high_card)
                        } else {
                            Hand::Pair(first_highest_value, high_card)
                        }
                    },
                    3 => {
                        if let Some((sec_highest_value, 2)) = hand_value_order.pop() {
                            Hand::FullHouse(first_highest_value, sec_highest_value)
                        } else {
                            Hand::ThreeOfAKind(first_highest_value, high_card)
                        }
                    }
                    _ => Hand::FourOfAKind(first_highest_value, high_card),
                }
            } else {
                Hand::HighCard(high_card)
            }
        }
    }
}

fn calculate_best_hand(hand: [Card; 2], table: &Vec<Card>) -> Hand {
    let all_cards: Vec<Card> = table.iter().chain(hand.iter()).map(|x| x.to_owned()).collect();
    let hand_size = min(all_cards.len(), 5);

    // println!("{:?}", all_cards);

    let permutations = all_cards.into_iter().permutations(hand_size);

    permutations.into_iter().map(|h| calculate_hand(h)).max()
        .expect("Permutations cannot be empty")
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
        let card: Card = self.rng.gen();

        if self.taboo_list.contains(&card) {
            self.draw()
        }
        else {
            self.taboo_list.push(card);
            card
        }
    }

    fn draw_n<const N: usize>(&mut self) -> [Card; N] {
        (0..N).map(|_| self.draw()).collect::<Vec<_>>().try_into().expect("Array should be of the correct size")
    }
}

enum GameStage {
    Dealt(Deck),
    Flop(Deck, [Card;3]),
    Turn(Deck, [Card;4]),
    River(Deck, [Card;5]),
    Finished(Vec<Card>) // We don't know how many cards will be on the table when we finish.
}

impl GameStage {
    fn advance(self) -> Self {
        match self {
            GameStage::Dealt(mut deck) => {
                let abc = deck.draw_n();
                GameStage::Flop(deck, abc)
            },
            GameStage::Flop(mut deck, [a, b, c]) => {
                let d = deck.draw();
                GameStage::Turn(deck, [a,b,c,d])
            },
            GameStage::Turn(mut deck, [a, b, c,d]) => {
                let e = deck.draw();
                GameStage::River(deck, [a,b,c,d,e])
            },
            GameStage::River(_, table) => GameStage::Finished(Vec::from(table)),
            GameStage::Finished(_) => panic!("Cannot advance finished state")
        }
    }

    fn finish(self) -> Self {
        Self::Finished(match self {
            GameStage::Dealt(_) => Vec::with_capacity(0),
            GameStage::Flop(_, table) => Vec::from(table),
            GameStage::Turn(_, table) => Vec::from(table),
            GameStage::River(_, table) => Vec::from(table),
            GameStage::Finished(a) => a
        })
    }
}

struct ActiveGame {
    players: (VecDeque<DealtPlayer>, VecDeque<DealtPlayer>),
    game_stage: GameStage,
    minimum_bet: usize,
    expected_bet: usize,
    pot: usize
}

struct PlayerEnvironment {
    // players:
}

#[derive(Debug)]
enum PokerActions {
    Raise(usize),
    Call,
    Fold
}

struct NewGame {
    deck: Deck,
    players: Vec<PlayerInfo>,
    minimum_bet: usize
}

impl ActiveGame {
    // Last two players in the list are always small and big blind
    fn new_with_players(players: Vec<PlayerInfo>, minimum_bet: usize) -> Self {
        let mut deck: Deck = Deck::new();
        let n_players = players.len() as isize;
        let players_dealt: VecDeque<_> = players
            .into_iter()
            .enumerate()
            .map(|(i, p)| DealtPlayer { player_info: p, cards: deck.draw_n(), current_bet: std::cmp::max(i as isize + 2 - n_players, 0) as usize * minimum_bet / 2 })
            .collect();

        ActiveGame {
            players: (VecDeque::with_capacity(players_dealt.len()), players_dealt),
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

    fn is_finished(&self) -> bool {
        matches!(self.game_stage, GameStage::Finished(_))
    }

    fn distribute_pots(&self) {
        assert!(self.is_finished());


    }

    fn get_environment(self) -> PlayerEnvironment {
        todo!()
    }
}

impl <'a> ActiveGame {
    fn peek_next_player_info(&'a self) -> &'a PlayerInfo {
        &self.players.1.front()
            .expect("game should always have a next player")
            .player_info
    }
}
