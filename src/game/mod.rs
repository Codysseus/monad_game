#![allow(dead_code)]
use rand::{seq::SliceRandom, thread_rng};
use std::io::{Write, stdout};

pub mod card;
pub mod table;
pub mod player;

use self::{
    table::Table,
    card::{Card, Value},
    player::Player,
};

use std::io::stdin;

pub fn read_uint_from_user() -> usize {
    let mut string = String::new();
    loop {
        stdin().read_line(&mut string).expect("Problem reading input from user!");
        if let Ok(r) = string.trim().parse::<usize>() {
            break r;
        }
        println!("What you entered is not an unsigned integer! Please try again.");
    }
}

pub struct Game {
    pub players: Vec<Player>,
    pub table: Table,
}

impl Game {
    // Public functions
    pub fn flip(&mut self) -> Result<(), String> {
        use self::card::Value::Common;
        if self.table.discard.is_empty() {
            return Err(String::from("The discard pile is empty!"));
        }
        if self.table.deck(Common).is_empty() {
            self.table.common.append(&mut self.table.discard);
        }
        Err(String::from("There are still commons in the common deck!"))
    }
    pub fn draw(&mut self, player: usize) -> Result<(), String> {
        use self::card::Value::Common;
        if let Some(card) = self.table.draw_top(Common) {
            self.players[player].hand.push(card);
            return Ok(());
        }
        Err(String::from("You can't draw, there are no commons left!"))
    }
    pub fn leap(&mut self, player: usize, cards: &mut Vec<usize>) -> Result<(), String> {
        let player = &mut self.players[player];

        if cards.len() < 4 || cards.len() > 6 {
            return Err(format!("Incorrect number of commons! Length is {}", cards.len()));
        }

        if ! cards.iter().all(|card| player.hand[*card].value == Value::Common) {
            return Err(String::from("Not all cards are common!"));
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

    pub fn buy(&mut self, player: usize, cards: &mut Vec<usize>, deck_value: Option<Value>) -> Result<bool, String> {
        let player = &mut self.players[player];
        let mut drew_card = true;
        let nums = cards
            .iter()
            .map(|p| &player.hand[*p])
            .map(Card::num)
            .collect::<Vec<_>>();

        let buy_value = nums.iter().sum::<usize>();
        let max_value = nums.into_iter().max().unwrap_or(0);

        let cost = deck_value
            .as_ref()
            .map(Value::num)
            .unwrap_or(80);

        if max_value >= cost {
            return Err(String::from("Can't buy something of the same value!"));
        }
        if buy_value < cost {
            return Err(String::from("Not enough points!"));
        }

        if let Some(value) = deck_value {
            if player.draw_card(value, &mut self.table).is_none() {
                return Err(format!("The {} deck is out of cards!", value));
            }
        } else {
            player.draw_monad(&mut self.table);
            drew_card = false;
        }

        cards.sort();
        for i in cards.iter().rev() {
            self.table.return_card(player.hand.remove(*i));
        }

        Ok(drew_card)
    }

    pub fn trade(&mut self, player: usize, card1: usize, card2: usize, bonus: bool) -> Result<(usize, bool), String> {
        let mut num_cards = 0;
        let mut drew_monad = false;

        let player = &mut self.players[player];
        let value = player.trade_value(card1, card2)?;

        if let Some(v) = value.succ() {
            if player.draw_card(v, &mut self.table).is_none() {
                return Err(format!("You can't draw any more {} cards! Please choose different cards!", v));
            }
            num_cards += 1;
        } else {
            player.draw_monad(&mut self.table)
                .expect("Woah! We ran out of Monads! This isn't supposed to happen!");
            drew_monad = true;
        }

        if bonus && player.can_take_bonus(card1, card2) {
            let mut maybe_curr_value = value.prev();
            while let Some(curr_value) = maybe_curr_value {
                if let Some(card) = player.draw_card(curr_value, &mut self.table) {
                    num_cards += 1;
                }
                maybe_curr_value = curr_value.prev();
            }
            player.took_bonus = true;
        }

        let mut selected_cards = [card1, card2];
        selected_cards.sort();
        for card in selected_cards.iter().rev() {
            self.table.return_card(player.hand.remove(*card));
        }

        Ok((num_cards, drew_monad))
    }

    pub fn init_turn(&mut self, player: usize) {
        self.players[player].took_bonus = false;
    }

    pub fn new(num_players: usize) -> Result<Self, String> {
        let mut table = Table::new(num_players);
        let mut players = Game::generate_players(num_players)?;

        for player in &mut players {
            player.hand.extend(table.common.drain(0..6));
        }

        Ok(Game { players, table })
    }

    pub fn print_state(&self, player: usize) {
        println!("{}", "-".repeat(20));
        println!("Player color: {}", self.players[player].identity);
        println!("Player {}'s hand: ", player);
        self.players[player].print_hand();
        println!("{}", "-".repeat(20));
        println!("Table state!");
        self.table.print_decks();
        println!("{}", "-".repeat(20));
    }

    // Private functions
    // Game::new() helper functions
    fn generate_players(num_players: usize) -> Result<Vec<Player>, String> {
        let mut colors = card::COLORS.to_vec();

        match num_players {
            2 => {
                colors.shuffle(&mut thread_rng());
                colors.drain(0..4);
            },
            3 => {
                colors.drain(0..3);
            },
            4 => {
                colors.remove(5);
                colors.remove(2);
            },
            _ => return Err(String::from("There should only be 2-4 players!")),
        }

        colors.shuffle(&mut thread_rng());

        Ok(colors.into_iter().map(Player::from).collect())
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
