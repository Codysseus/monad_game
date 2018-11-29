#![allow(dead_code)]
use rand::{seq::SliceRandom, thread_rng};

pub mod card;
pub mod table;

use self::table::Table;
use self::card::Monad;

struct Player {
    hand: card::Deck,
    identity: card::Color,
    monads: Vec<Monad>,
}

impl From<card::Color> for Player {
    fn from(color: card::Color) -> Self {
        Player {
            identity: color,
            hand: card::Deck::default(),
            monads: Vec::new(),
        }
    }
}

struct Game {
    players: Vec<Player>,
    table: Table,
}

impl Game {
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

    pub fn new(num_players: usize) -> Result<Self, String> {
        let mut table = Table::new(num_players);
        let mut players = Game::generate_players(num_players)?;

        for player in &mut players {
            player.hand.extend(table.common.drain(0..6));
        }

        Ok(Game { players, table })
    }
}
