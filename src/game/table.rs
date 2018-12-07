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

        for &color in &card::COLORS {
            use self::card::Value::*;

            table.common.extend(
                repeat_with(|| Card { value: Common, color }).take(players)
            );
            table.bi   .push(Card { value: Bi   , color });
            table.tri  .push(Card { value: Tri  , color });
            table.quad .push(Card { value: Quad , color });
            table.quint.push(Card { value: Quint, color });
        }

        table.shuffle_decks();

        table
    }

    pub fn select_deck_value(&self) -> Result<Option<self::card::Value>, String> {
        use self::card::Value::*;
        loop {
            let value = match read_uint_from_user() {
                0 => break Err(String::from("Exiting deck selection.")),
                1 => Common,
                2 => Bi,
                3 => Tri,
                4 => Quad,
                5 => Quint,
                6 => break Ok(None),
                n => {
                    println!("{} is an invalid selection! Please try again.", n);
                    continue;
                }
            };

            if self.deck(value).is_empty() {
                println!("That deck is out of cards! Please select a new deck.");
                continue;
            }

            break Ok(Some(value));
        }
    }

    pub fn deck_mut(&mut self, value: card::Value) -> &mut Deck {
        use self::card::Value::*;
        match value {
            Common => &mut self.common,
            Bi     => &mut self.bi,
            Tri    => &mut self.tri,
            Quad   => &mut self.quad,
            Quint  => &mut self.quint,
        }
    }

    pub fn deck(&self, value: card::Value) -> &Deck{
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
        self.deck_mut(value).pop()
    }

    pub fn return_card(&mut self, card: Card) {
        use self::card::Value::Common;
        match card.value {
            Common => &mut self.discard,
            value  => self.deck_mut(value),
        }.insert(0, card);
    }

    fn shuffle_decks(&mut self) {
        self.bi    .shuffle();
        self.tri   .shuffle();
        self.quad  .shuffle();
        self.quint .shuffle();
        self.common.shuffle();
    }
}
