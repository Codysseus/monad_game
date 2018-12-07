use rand::{seq::SliceRandom, thread_rng};
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

#[must_use]
pub struct Monad;

#[derive(PartialEq)]
pub enum Temp {
    Warm,
    Cold,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Purple,
    Blue,
    Green,
}

impl fmt::Display for Color {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Value {
    Common,
    Bi,
    Tri,
    Quad,
    Quint,
}

impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl Value {
    pub fn succ(&self) -> Option<Value> {
        use self::Value::*;
        match self {
            Common => Some(Bi),
            Bi     => Some(Tri),
            Tri    => Some(Quad),
            Quad   => Some(Quint),
            Quint  => None,
        }
    }
    pub fn prev(self) -> Option<Value> {
        use self::Value::*;
        match self {
            Common => None,
            Bi     => Some(Common),
            Tri    => Some(Bi),
            Quad   => Some(Tri),
            Quint  => Some(Quad),
        }
    }
    pub fn num(&self) -> usize {
        use self::Value::*;
        match self {
            Common => 1,
            Bi     => 3,
            Tri    => 7,
            Quad   => 16,
            Quint  => 36,
        }
    }
}

pub struct Card {
    pub value: Value,
    pub color: Color,
}
impl fmt::Display for Card {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}:{}", self.color, self.value)
    }
}

impl Card {
    pub fn temp(&self) -> Temp {
        use self::Color::*;
        match self.color {
            Red | Orange | Yellow => Temp::Warm,
            Purple | Blue | Green => Temp::Cold,
        }
    }

    pub fn num(&self) -> usize {
        self.value.num()
    }

    pub fn is_common(&self) -> bool {
        self.value == Value::Common
    }
}

#[derive(Default)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub fn multiple(multiple: usize) -> Self {
        Deck(Vec::with_capacity(COLORS.len() * multiple))
    }

    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut thread_rng());
    }
    pub fn to_string(&self) -> String {
        let mut string = String::new();
        for card in self.iter() {
            string += &format!("{}\t", card);
        }
        string
    }
}

impl Deref for Deck {
    type Target = Vec<Card>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target { &mut self.0 }
}

pub const COLORS: [Color; 6] = [
    Color::Red,
    Color::Orange,
    Color::Yellow,
    Color::Purple,
    Color::Blue,
    Color::Green,
];
