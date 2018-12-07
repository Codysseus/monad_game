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
    pub fn leap(&mut self, player: usize) -> Result<(), String> {
        let player = &mut self.players[player];
        let mut commons: Vec<usize> = Vec::new();
        let mut num_commons: usize;

        for i in 0..player.hand.len() {
            if player.hand[i].is_common() {
                commons.push(i);
            }
        }

        if commons.len() < 4 {
            return Err(String::from("Not enough commons to leap!"));
        }

        loop {
            num_commons = Game::select_commons_leap(&commons)?;
            let deck_value = Game::translate_commons_for_leap(num_commons);
            if ! self.table.deck(deck_value).is_empty() {
                break;
            }
            println!("That deck is empty!");
        }

        if num_commons != commons.len() {
            loop {
                println!("Here are all the commons to select. The first {} cards on the left will be traded in.", num_commons);
                println!("Enter the number of the card to move it left.");
                println!("Enter {} to accept selection.", commons.len());
                let card_refs = player.indexes_to_cards(&commons);
                for i in 0..card_refs.len() {
                    print!("{}: {}  ", i, card_refs[i]);
                }
                println!("");

                let card_num = read_uint_from_user();
                if card_num == 0 {
                    continue;
                }
                if card_num == commons.len() {
                    return Err(String::from("You decided not to leap!"));
                }
                if card_num < commons.len() {
                    let index = card_num - 1;
                    commons.swap(card_num, index);
                    break;
                }
                println!("Not a valid selection! Please try again.");
            }
            commons.split_off(num_commons);
        }

        let card = self.table.draw_top(
            Game::translate_commons_for_leap(num_commons)
        ).unwrap();

        for elt in commons {
            self.table.return_card(player.hand.remove(elt));
        }

        player.hand.push(card);

        Ok(())
    }

    pub fn buy(&mut self, player: usize) -> Result<(), String>{
        let player = &mut self.players[player];
        let mut cards: Vec<usize> = Vec::new();
        loop {
            player.print_hand();
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

        table.print_decks();
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
                colors.remove(2);
                colors.remove(5);
            },
            _ => return Err(String::from("There should only be 2-4 players!")),
        }

        colors.shuffle(&mut thread_rng());

        Ok(colors.into_iter().map(Player::from).collect())
    }

    // Game::leap() helper functions
    fn select_commons_leap(commons: & Vec<usize>) -> Result<usize, String> {
        loop {
            println!("Enter how many commons you want to trade! (3 -> Quit; 4 -> Tri; 5 -> Quad; 6 -> Quint)");
            let x = read_uint_from_user();
            if x == 3 {
                break Err(String::from("You have decided not to leap! Exiting..."));
            }
            if x > 3 && x <= commons.len() {
                break Ok(x);
            }
            if x > commons.len() && x < 7 {
                println!("You don't have enough cards to trade in for that!!");
            }
            println!("That is an incorrect selection!");
        }
    }

    // Using option for this makes it ugly and checks ensure that values range from 4-6
    fn translate_commons_for_leap(value: usize) -> self::card::Value {
        use self::card::Value::*;
        match value {
            4 => Tri,
            5 => Quad,
            _ => Quint,
        }
    }
}
