#![allow(dead_code)]
extern crate itertools;
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};
use std::{str::FromStr, fmt};

pub mod card;
pub mod table;
pub mod player;

use self::{
    table::Table,
    card::{Card, Value, ValueOrMonad},
    player::Player,
};

#[derive(Clone, Copy)]
pub enum NumPlayers {
    Two = 2,
    Three = 3,
    Four = 4,
}

impl FromStr for NumPlayers {
    type Err = ();
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        source
            .trim()
            .parse::<usize>()
            .map_err(|_| ())
            .and_then(|num|
                match num {
                    2 => Ok(NumPlayers::Two),
                    3 => Ok(NumPlayers::Three),
                    4 => Ok(NumPlayers::Four),
                    _ => Err(()),
                }
            )
    }
}

pub enum FlipError {
    EmptyDiscardPile,
    NonEmptyCommonDeck,
}

impl fmt::Display for FlipError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::FlipError::*;
        write!(fmt, "{}", match self {
            EmptyDiscardPile => "Discard pile is empty",
            NonEmptyCommonDeck => "Common deck still has cards",
        })
    }
}

pub enum LeapError {
    NumOfCards(usize),
    NotAllCommons,
}

impl fmt::Display for LeapError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::LeapError::*;
        match self {
            NumOfCards(length) => write!(fmt, "Incorrect number of cards: {}", length),
            NotAllCommons => write!(fmt, "Not all cards are common"),
        }
    }
}

pub enum TradeOutcome {
    Cards(usize),
    Monad,
}

pub enum TradeError {
    OutOfCards(Value),
    SameTemperature,
    NotSameValueOrIdentity,
}

impl fmt::Display for TradeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::TradeError::*;
        match self {
            OutOfCards(_) => write!(fmt, "Can't buy something of the same value"),
            SameTemperature => write!(fmt, "Cards should not be the same tempurature"),
            NotSameValueOrIdentity => write!(fmt, "Cards must have the same value, or one must match your color"),
        }
    }
}

pub enum BuyError {
    SameValue,
    NotEnoughPoints,
    OutOfCards(Value),
}

impl fmt::Display for BuyError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::BuyError::*;
        match self {
            SameValue => write!(fmt, "Can't buy something of the same value!"),
            NotEnoughPoints => write!(fmt, "Not enough points!"),
            OutOfCards(value) => write!(fmt, "The {} deck is out of cards!", value),
        }
    }
}

pub struct Game {
    pub players: Vec<Player>,
    pub table: Table,
}

impl Game {
    pub fn new(num_players: NumPlayers) -> Self {
        let mut table = Table::new(num_players);
        let mut players = Game::generate_players(num_players);

        for player in &mut players {
            player.hand.extend(table.common.drain(0..6));
        }

        Game { players, table }
    }

    pub fn flip(&mut self) -> Result<(), FlipError> {
        if self.table.discard.is_empty() { return Err(FlipError::EmptyDiscardPile); }
        if !self.table.deck(card::Value::Common).is_empty() { return Err(FlipError::NonEmptyCommonDeck); }

        self.table.common.append(&mut self.table.discard);

        Ok(())
    }

    pub fn draw(&mut self, player: usize) -> Result<(), ()> {
        match self.table.draw_top(card::Value::Common) {
            Some(card) => {
                self.players[player].hand.push(card);
                Ok(())
            },
            None => Err(()),
        }
    }

    pub fn leap(&mut self, player: usize, cards: &mut Vec<usize>) -> Result<(), LeapError> {
        let player = &mut self.players[player];

        if cards.len() < 4 || cards.len() > 6 {
            return Err(LeapError::NumOfCards(cards.len()));
        }

        if ! cards.iter().all(|&card| player.hand[card].value.is_common()) {
            return Err(LeapError::NotAllCommons);
        }

        let card = self.table.draw_top(
            Game::translate_commons_for_leap(cards.len())
        ).unwrap();

        cards.sort();
        for elt in cards.iter().rev() {
            self.table.return_card(player.hand.remove(*elt));
        }

        player.hand.push(card);

        Ok(())
    }

    pub fn buy(
        &mut self,
        player: usize,
        cards: &mut Vec<usize>,
        deck_or_monad: ValueOrMonad
    ) -> Result<bool, BuyError> {
        let player = &mut self.players[player];
        let nums = cards
            .iter()
            .map(|p| &player.hand[*p])
            .map(Card::num)
            .collect::<Vec<_>>();

        let buy_value = nums.iter().sum::<usize>();
        let max_value = nums.into_iter().max().unwrap_or(0);

        let cost = deck_or_monad.points();

        if max_value >= cost {
            return Err(BuyError::SameValue);
        }
        if buy_value < cost {
            return Err(BuyError::NotEnoughPoints);
        }

        let drew_card;
        match deck_or_monad {
            ValueOrMonad::Value(value) => {
                if player.draw_card(value, &mut self.table).is_none() {
                    return Err(BuyError::OutOfCards(value));
                }
                drew_card = true;
            },
            ValueOrMonad::Monad => {
                player.draw_monad(&mut self.table);
                drew_card = false;
            },
        }

        cards.sort();
        for i in cards.iter().rev() {
            self.table.return_card(player.hand.remove(*i));
        }

        Ok(drew_card)
    }

    pub fn trade(
        &mut self,
        player: usize,
        card1: usize,
        card2: usize,
        bonus: bool,
    ) -> Result<(usize, bool), TradeError> {
        let mut num_cards = 0;
        let mut drew_monad = false;

        let player = &mut self.players[player];
        let value = player.trade_value(card1, card2)?;

        if let Some(succ_value) = value.succ() {
            if player.draw_card(succ_value, &mut self.table).is_none() {
                return Err(TradeError::OutOfCards(succ_value));
            }
            num_cards += 1;
        } else {
            player.draw_monad(&mut self.table).unwrap();
            drew_monad = true;
        }

        if bonus {
            let mut maybe_curr_value = value.prev();
            while let Some(curr_value) = maybe_curr_value {
                if player.draw_card(curr_value, &mut self.table).is_some() {
                    num_cards += 1;
                }
                maybe_curr_value = curr_value.prev();
            }
            player.took_bonus = true;
        }

        let mut selected_cards = [card1, card2];
        selected_cards.sort();

        for &card in selected_cards.iter().rev() {
            self.table.return_card(player.hand.remove(card));
        }

        Ok((num_cards, drew_monad))
    }

    pub fn check_player_end(&self, player: usize) -> bool {
        let player = &self.players[player];
        if ! (self.table.common.is_empty() || self.table.discard.is_empty() )  {
            return false;
        }
        let trade_values: Vec<Value> =
            (0..player.hand.len())
                .combinations(2)
                .filter_map(|pair| player.trade_value(pair[0], pair[1]).ok())
                .collect();

        if let Some(max_value) = trade_values.iter().max() {
            if let Some(value) = max_value.succ() {
                if ! self.table.deck(value).is_empty() {
                    return false;
                }
            }
            else {
                return false;
            }
        }

        

//        let max_value = player.hand.iter().map(|card| card.value).max().succ();
//        let buy_value = player.hand.iter().fold(0, |acc, card| acc + card.value.points());
        true
    }

    pub fn player_took_bonus(&mut self, player: usize) -> &mut bool {
        &mut self.players[player].took_bonus
    }

    fn generate_players(num_players: NumPlayers) -> Vec<Player> {
        let mut colors = card::COLORS.to_vec();

        match num_players {
            NumPlayers::Two => {
                colors.shuffle(&mut thread_rng());
                colors.drain(0..4);
            },
            NumPlayers::Three => {
                colors.drain(0..3);
            },
            NumPlayers::Four => {
                colors.remove(5);
                colors.remove(2);
            },
        }

        colors.shuffle(&mut thread_rng());

        colors.into_iter().map(Player::from).collect()
    }

    // Using option for this makes it ugly and checks ensure that values range from 4-6
    pub fn translate_commons_for_leap(value: usize) -> self::card::Value {
        use self::card::Value::*;
        match value {
            4 => Tri,
            5 => Quad,
            _ => Quint,
        }
    }
}
