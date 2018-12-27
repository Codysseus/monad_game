use rand::{seq::SliceRandom, thread_rng};
use std::{
    fmt,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[must_use]
pub struct Monad;

impl FromStr for Monad {
    type Err = ();
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        if source == "Monad" { Ok(Monad) } else { Err(()) }
    }
}

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
        use self::Value::*;
        write!(fmt, "{}", match self {
            Common => "①",
            Bi => "②",
            Tri => "③",
            Quad => "④",
            Quint => "⑤",
        })
    }
}

impl FromStr for Value {
    type Err = ();
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        use self::Value::*;
        let value = match source {
            "Common" => Common,
            "Bi"     => Bi,
            "Tri"    => Tri,
            "Quad"   => Quad,
            "Quint"  => Quint,
            _ => return Err(()),
        };

        Ok(value)
    }
}

impl Value {
    pub fn try_from(source: usize) -> Result<Self, ()> {
        use self::Value::*;
        let value = match source {
            1 => Common,
            2 => Bi,
            3 => Tri,
            4 => Quad,
            5 => Quint,
            _ => return Err(()),
        };

        Ok(value)
    }
    pub fn succ(self) -> Option<Value> {
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
    pub fn points(self) -> usize {
        use self::Value::*;
        match self {
            Common => 1,
            Bi     => 3,
            Tri    => 7,
            Quad   => 16,
            Quint  => 36,
        }
    }
    pub fn is_common(self) -> bool {
        self == Value::Common
    }
}

pub enum ValueOrMonad { Value(Value), Monad }

impl ValueOrMonad {
    pub fn points(&self) -> usize {
        match self {
            ValueOrMonad::Value(value) => value.points(),
            ValueOrMonad::Monad => 80,
        }
    }
}

#[derive(Clone)]
pub struct Card {
    pub value: Value,
    pub color: Color,
}

impl fmt::Display for Card {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[{} {}]", self.value, self.color)
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
        self.value.points()
    }

    pub fn is_common(&self) -> bool {
        self.value.is_common()
    }
}

#[derive(Default)]
pub struct Deck(Vec<Card>);

impl fmt::Display for Deck {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self
            .iter()
            .enumerate()
            .map(|(i, card)| write!(fmt, "{}: {} ", i, card))
            .collect()
    }
}

impl Deck {
    pub fn multiple(multiple: usize) -> Self {
        Deck(Vec::with_capacity(COLORS.len() * multiple))
    }

    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut thread_rng());
    }

    pub fn find_all(&self, predicate: impl Fn(&Card) -> bool) -> Vec<usize> {
        (0..self.len())
            .filter(|&i| predicate(&self[i]))
            .collect()
    }
}

impl From<Vec<Card>> for Deck {
    fn from(vec: Vec<Card>) -> Deck {
        Deck(vec)
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
