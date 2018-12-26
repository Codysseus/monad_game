use crate::game::{
    NumPlayers,
    card::{self, Monad, Card, Deck},
};
use std::{
    fmt,
    iter::repeat_with,
};

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
    pub fn new(players: NumPlayers) -> Self {
        let mut table = Table {
            discard: Deck::multiple(players as usize),
            common:  Deck::multiple(players as usize),
            bi:      Deck::multiple(1),
            tri:     Deck::multiple(1),
            quad:    Deck::multiple(1),
            quint:   Deck::multiple(1),
            monad:   repeat_with(|| Monad).take(12).collect(),
        };

        for &color in &card::COLORS {
            use self::card::Value::*;

            table.common.extend(
                repeat_with(|| Card { value: Common, color }).take(players as usize)
            );
            table.bi   .push(Card { value: Bi   , color });
            table.tri  .push(Card { value: Tri  , color });
            table.quad .push(Card { value: Quad , color });
            table.quint.push(Card { value: Quint, color });
        }

        table.shuffle_decks();

        table
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

    pub fn deck(&self, value: card::Value) -> &Deck {
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

impl fmt::Display for Table {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
"Common:  {}
Discard: {}
Bi:      {}
Tri:     {}
Quad:    {}
Quint:   {}
",
            self.common.len(),
            self.discard,
            self.bi,
            self.tri,
            self.quad,
            self.quint,
        )
    }
}
