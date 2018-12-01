#![allow(dead_code)]
use rand::{seq::SliceRandom, thread_rng};

pub mod card;
pub mod table;
pub mod player;

use self::table::Table;
use self::card::Value;
use self::player::Player;
use std::io;

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

pub struct Game {
    players: Vec<Player>,
    table: Table,
}

impl Game {
    // Public functions
    pub fn trade(&mut self, player: usize) -> Result<String, String>{
        let player = &mut self.players[player];
        let mut card_value: card::Value;
        let mut card1: usize = 0;
        let mut card2: usize = 0;

        loop {
            println!("Please enter the first card for trading!");
            card1 = player.select_card_in_hand()?;

            println!("Please enter the second card for trading!");
            card2 = player.select_card_in_hand()?;

            card_value = match player.get_trade_value(card1, card2) {
                Ok(v) => v,
                Err(m) => { println!("{}", m); continue; },
            };

            match card_value.succ() {
                Some(v) => {
                    if let Some(c) = self.table.draw_top(v){
                        player.hand.push(c);
                        break;
                    }
                },
                None => {
                    player.monads.push(self.table.monad.pop()
                                       .expect("Woah! We ran out of Monads! This isn't supposed to happen!"));
                    break;
                },
            };
            println!("You can't draw any more of that card! Please choose different cards!");
        };

        if player.is_bonus_pair(card1, card2) {
            println!("Woah! You picked a bonus pair!");
            let mut curr_value = card_value.prev();
            while curr_value.is_some() {
                let v = curr_value.unwrap();
                if let Some(c) = self.table.draw_top(v) {
                    println!("Drew a card!");
                    player.hand.push(c);
                }
                curr_value = v.prev();
            }
        }

        self.table.return_card(player.hand.remove(card1));
        self.table.return_card(player.hand.remove(card2));
        Ok(String::from("Trade completed successfully!"))
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
