#![allow(dead_code)]
use rand::{seq::SliceRandom, thread_rng};
use std::ops::{Deref, DerefMut};

#[must_use]
pub struct Monad;

#[derive(PartialEq)]
pub enum Temp {
    Warm,
    Cold,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Purple,
    Blue,
    Green,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Value {
    Common,
    Bi,
    Tri,
    Quad,
    Quint,
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
}

pub struct Card(pub Value, pub Color);
impl Card {
    pub fn get_temp(&self) -> Temp {
        use self::Color::*;
        match self.1 {
            Yellow | Red | Orange => Temp::Warm,
            _ => Temp::Cold,
        }
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
