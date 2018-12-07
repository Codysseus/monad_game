#![allow(dead_code)]
use rand::{seq::SliceRandom, thread_rng};

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
    players: Vec<Player>,
    table: Table,
}

impl Game {
    // Public functions
    pub fn leap(&mut self, player: usize) -> Result<(), String> {
        use self::card::Value::*;
        let player = &mut self.players[player];
        let mut commons = player
            .hand
            .iter()
            .filter(|card| card.is_common()) // Predicate takes &&Card
            .collect::<Vec<_>>();

        if commons.len() < 4 {
            return Err(String::from("Not enough commons to leap!"));
        }

        let num_commons = loop {
            println!("Enter how many commons you want to trade! (4 -> Tri; 5 -> Quad; 6 -> Quint)");
            let x = read_uint_from_user();
            if x == 3 {
                return Err(String::from("You decided not to leap!"));
            }
            if x > 3 && x <= commons.len() {
                let deck = match x {
                    4 => self.table.deck(Tri),
                    5 => self.table.deck(Quad),
                    _ => self.table.deck(Quint),
                };
                if deck.len() > 0 {
                    break x;
                }
                println!("That deck is out of cards!");
                continue;
            }
            if x > commons.len() && x < 7 {
                println!("You don't have enough cards to trade in!");
            }
            println!("That is an incorrect selection!");
        };

        if num_commons != commons.len() {
            for _i in 0..num_commons {
                loop {
                    let card_num = read_uint_from_user();
                    if card_num == 0 {
                        return Err(String::from("You decided not to leap!"));
                    }
                    if card_num <= commons.len() {
                        let card = commons.remove(card_num);
                        commons.push(card);
                        break;
                    }
                    println!("Not a valid selection! Please try again.");
                }
            }
            commons = commons.clone().split_off(commons.len() - num_commons);
        }

        let card = match num_commons {
            4 => self.table.draw_top(Tri),
            5 => self.table.draw_top(Quad),
            _ => self.table.draw_top(Quint),
        };



        Ok(())
    }

    pub fn buy(&mut self, player: usize) -> Result<(), String>{
        let player = &mut self.players[player];
        let mut cards: Vec<usize> = Vec::new();
        loop {
            println!("Enter the number of a card you want to use.");
            match player.select_card_in_hand() {
                Ok(card)     => cards.push(card),
                Err(message) => {
                    if cards.is_empty() {
                        return Err(String::from("No cards selected! Exiting buying mode."));
                    }
                    println!("{} Let's see if you can buy anything with this!", message);
                    break;
                },
            };
        }
        cards.dedup();

        let nums = cards
            .iter()
            .map(|p| &player.hand[*p])
            .map(Card::num)
            .collect::<Vec<_>>();

        let buy_value = nums.iter().sum::<usize>();
        let max_value = nums.into_iter().max().unwrap_or(0);

        loop {
            println!("Pick the deck you want to buy from!");
            let choice = self.table.select_deck_value()?;

            let cost = choice
                .as_ref()
                .map(Value::num)
                .unwrap_or(80);

            if max_value >= cost {
                println!("Can't buy something of the same value!");
                continue;
            }
            if buy_value < cost {
                println!("Not enough points!");
                continue;
            }

            if let Some(value) = choice {
                player.draw_card(value, &mut self.table);
            } else {
                player.draw_monad(&mut self.table);
            }

            for i in cards {
                self.table.return_card(player.hand.remove(i));
            }

            break Ok(());
        }
    }

    pub fn trade(&mut self, player: usize) -> Result<(), String> {
        let player = &mut self.players[player];

        let (card1, card2, value) = loop {
            println!("Please enter the first card for trading!");
            let card1 = player.select_card_in_hand()?;

            println!("Please enter the second card for trading!");
            let card2 = player.select_card_in_hand()?;

            let value = match player.trade_value(card1, card2) {
                Ok(v) => v,
                Err(m) => {
                    println!("{}", m);
                    continue;
                },
            };

            if let Some(v) = value.succ() {
                if player.draw_card(v, &mut self.table).is_none() {
                    println!("You can't draw any more {} cards! Please choose different cards!", v);
                    continue;
                }
            } else {
                player
                    .draw_monad(&mut self.table)
                    .expect("Woah! We ran out of Monads! This isn't supposed to happen!");
            }

            break (card1, card2, value);
        };

        if player.is_bonus_pair(card1, card2) {
            println!("Woah! You picked a bonus pair!");
            let mut maybe_curr_value = value.prev();

            while let Some(curr_value) = maybe_curr_value {
                player
                    .draw_card(curr_value, &mut self.table)
                    .map(|card| println!("Drew a {} {} card!", card.color, card.value));

                maybe_curr_value = curr_value.prev();
            }
        }

        self.table.return_card(player.hand.remove(card1));
        self.table.return_card(player.hand.remove(card2));

        Ok(())
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
            },
            _ => return Err(String::from("There should only be 2-4 players!")),
        }

        colors.shuffle(&mut thread_rng());

        Ok(colors.into_iter().map(Player::from).collect())
    }
}
