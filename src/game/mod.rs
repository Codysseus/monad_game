#![allow(dead_code)]
use rand::{seq::SliceRandom, thread_rng};

pub mod card;
pub mod table;
pub mod player;

use self::table::Table;
use self::card::Value;
use self::player::Player;
use std::io;


pub struct Game {
    players: Vec<Player>,
    table: Table,
}

pub fn read_uint_from_user() -> usize {
    let mut string = String::new();
    loop {
        io::stdin().read_line(&mut string).expect("Problem reading input from user!");
        if let Ok(r) = string.trim().parse::<usize>() {
            break r;
        }
        println!("What you entered is not an unsigned integer! Please try again.");
    }
}

impl Game {
    // Public functions
    pub fn trade(&mut self, player: usize) {
        let player = &mut self.players[player];
        let mut card_value: card::Value;
        let mut card1: usize = 0;
        let mut card2: usize = 0;

        let card = loop {
            println!("Please enter the first card for trading!");
            card1 = read_uint_from_user();

            println!("Please enter the second card for trading!");
            card2 = read_uint_from_user();

            card_value = match player.get_trade_value(card1, card2) {
                Ok(v) => v,
                Err(m) => { println!("{}", m); continue; },
            };

            let card = match card_value.succ() {
                Some(v) => self.table.draw_top(v),
                None => {
                    player.monads.push(self.table.monad.pop().unwrap());
                    // Returns here because the main should already have stopped the game before the monads run out
                    return;
                },
            };

            if let Some(v) = card {
                break v;
            }

            println!("You can't draw any more of that card! Please choose different cards!");
        };

        let discard_deck = match card_value {
            Value::Common => &mut self.table.discard,
            _             => self.table.get_deck(card_value),
        };

        player.hand.push(card);

        if player.is_bonus_pair(card1, card2){
            let mut curr_value = card_value.prev();
            while curr_value.is_some() {
                let v = curr_value.unwrap();
                if let Some(c) = self.table.draw_top(v){
                    player.hand.push(c);
                }
                curr_value = v.prev();
            }
        }

        discard_deck.insert(0, player.hand.remove(card1));
        discard_deck.insert(0, player.hand.remove(card2));
    }

    pub fn new(num_players: usize) -> Result<Self, String> {
        let mut table = Table::new(num_players);
        let mut players = Game::generate_players(num_players)?;

        for player in &mut players {
            player.hand.extend(table.common.drain(0..6));
        }

        Ok(Game { players, table })
    }

    // Private functions
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
                colors.remove(2);
                colors.remove(5);
            }
            _ => return Err(String::from("There should only be 2-4 players!")),
        }

        colors.shuffle(&mut thread_rng());

        Ok(colors.into_iter().map(Player::from).collect())
    }
}
