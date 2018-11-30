#![allow(dead_code)]
use rand::{seq::SliceRandom, thread_rng};

pub mod card;
pub mod table;
pub mod player;

use self::table::Table;
use self::card::{Value, Monad};
use self::player::Player;
use std::io;


pub struct Game {
    players: Vec<Player>,
    table: Table,
}

impl Game {
    // Public functions
    pub fn trade(&mut self, player: usize){
        let mut player = &mut self.players[player];
        let mut card1 = String::new();
        let mut card2 = String::new();

        loop{
            println!("Please enter the first card you want to trade.");

            io::stdin().read_line(&mut card1)
                .expect("Failed to read line.");

            let card1: usize = card1.trim().parse()
                .expect("Please type a number!");

            io::stdin().read_line(&mut card2)
                .expect("Failed to read line.");

            let card2: usize = card2.trim().parse()
                .expect("Please type a number!");
            
            let card_value = match player.get_trade_value(card1, card2) {
                Ok(value) => value,
                Err(msg) => {
                    println!("{}", msg);
                    continue;
                },
            };

            use self::card::Value::*;
            let card = match card_value {
                Common => self.table.bi.pop(),
                Bi => self.table.tri.pop(),
                Tri => self.table.quad.pop(),
                Quad => self.table.quint.pop(),
                Quint => {
                    player.monads.push(self.table.monad.pop().unwrap());
                    return;
                },
            };

            match card {
                Some(c) => {
                    let deck = self.table.get_deck(card_value);
                    deck.insert(0, player.hand.remove(card1));
                    deck.insert(0, player.hand.remove(card2));
                    player.hand.push(c);
                }
                None => {
                    println!("You can't buy any more of that card!");
                    continue;
                }
            }
            return;
        }
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
