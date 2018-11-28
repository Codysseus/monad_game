#![allow(dead_code)]
use rand::{seq::SliceRandom, thread_rng};
use std::ops::{Deref, DerefMut};

pub struct Monad;

#[derive(Clone, Copy)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Purple,
    Blue,
    Green,
}

pub enum Value {
    Common,
    Bi,
    Tri,
    Quad,
    Quint,
}

pub struct Card(pub Value, pub Color);

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
