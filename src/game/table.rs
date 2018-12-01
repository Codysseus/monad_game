#![allow(dead_code)]

use super::read_uint_from_user;
use super::card::{self, Monad, Card, Deck};
use std::iter::repeat_with;

pub struct Table {
    pub discard: Deck,
    pub common:  Deck,
    pub bi:      Deck,
    pub tri:     Deck,
    pub quad:    Deck,
    pub quint:   Deck,
    pub monad:   Vec<Monad>,
}

impl Table {
    pub fn new(players: usize) -> Self {
        let mut table = Table {
            discard: Deck::multiple(players),
            common:  Deck::multiple(players),
            bi:      Deck::multiple(1),
            tri:     Deck::multiple(1),
            quad:    Deck::multiple(1),
            quint:   Deck::multiple(1),
            monad:   repeat_with(|| Monad).take(12).collect(),
        };

        for color in &card::COLORS {
            use self::card::Value::*;

            table.common.extend(
                repeat_with(|| Card(Common, *color)).take(players)
            );
            table.bi   .push(Card(Bi   , *color));
            table.tri  .push(Card(Tri  , *color));
            table.quad .push(Card(Quad , *color));
            table.quint.push(Card(Quint, *color));
        }

        table.shuffle_decks();

        table
    }

    pub fn select_deck_value(&self) -> Result<Option<self::card::Value>, String> {
        use self::card::Value::*;
        loop {
            let n = read_uint_from_user();
            if n < 7 {
                if n == 0 {
                    return Err(String::from("Exiting deck selection."));
                }
                let n = match n {
                    1 => Some(Common),
                    2 => Some(Bi),
                    3 => Some(Tri),
                    4 => Some(Quad),
                    5 => Some(Quint),
                    _ => None,
                };
                if let Some(v) = n {
                    if self.get_deck(v).len() == 0 {
                        println!("That deck is out of cards! Please select a new deck.");
                        continue;
                    }
                }
                break Ok(n);
            }
            println!("{} is an invalid selection! Please try again.", n);
        }
    }

    pub fn get_deck_mut(&mut self, value: card::Value) -> &mut Deck{
        use self::card::Value::*;
        match value {
            Common => &mut self.common,
            Bi     => &mut self.bi,
            Tri    => &mut self.tri,
            Quad   => &mut self.quad,
            Quint  => &mut self.quint,
        }
    }

    pub fn get_deck(&self, value: card::Value) -> &Deck{
        use self::card::Value::*;
        match value {
            Common => &self.common,
            Bi     => &self.bi,
            Tri    => &self.tri,
            Quad   => &self.quad,
            Quint  => &self.quint,
        }
    }

    pub fn draw_top(&mut self, value: card::Value) -> Option<Card> {
        return self.get_deck_mut(value).pop()
    }
    pub fn return_card(&mut self, card: Card){
        let discard_deck = match card.0 {
            self::card::Value::Common => &mut self.discard,
            v                         => self.get_deck_mut(v),
        };
        discard_deck.insert(0, card);
    }

    fn shuffle_decks(&mut self) {
        self.bi    .shuffle();
        self.tri   .shuffle();
        self.quad  .shuffle();
        self.quint .shuffle();
        self.common.shuffle();
    }
}

