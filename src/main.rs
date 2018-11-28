extern crate rand;
use rand::{Rng,thread_rng};
use Cardcolor::*;
use Cardvalue::*;

// Value Definitions
#[derive(Clone, Copy)]
enum Cardcolor {
    Red,
    Orange,
    Yellow,
    Purple,
    Blue,
    Green,
}
enum Cardvalue {
    Common,
    Bi,
    Tri,
    Quad,
    Quint,
}

// Structure Definitions
struct Card {
    color: Cardcolor,
    value: Cardvalue,
}
struct Player {
    hand: Vec<Card>,
    identity: Cardcolor,
    monads: u8,
}
struct Table {
    discard: Vec<Card>,
    common: Vec<Card>,
    bi: Vec<Card>,
    tri: Vec<Card>,
    quad: Vec<Card>,
    quint: Vec<Card>,
    monad: u8, // Monads have no value or color. They are just there to buy.
}
struct Game {
    players: Vec<Player>,
    decks: Table,
}

// Function Definitions
impl From<Cardcolor> for Player {
    fn from(color: Cardcolor) -> Self {
        Player { identity: color, hand: Vec::new(), monads: 0 }
    }
}
impl Game {
    /*
     * Name: initialize_table
     * Desc: Initialize all of the decks in the Table struct
     * Parameters:
     *      - Name: decks
     *        Type: Table
     *        Desc:
     *      - Name: players
     *        Type: u8
     *        Desc:
     */
    fn initialize_table(&mut self, players: u8) {
        // The amount of commons is dependent on the number of players
        let players = players as usize;

        // Create vectors for the different decks
        self.decks = Table {
        	discard: Vec::with_capacity(6*players),
        	common: Vec::with_capacity(6*players),
        	bi: Vec::with_capacity(6),
        	tri: Vec::with_capacity(6),
        	quad: Vec::with_capacity(6),
        	quint: Vec::with_capacity(6),
            monad: 12
        };

        // Fill decks with appropriate cards
        for color in &[Red, Orange, Yellow, Purple, Blue, Green] {
            for _ in 1..players {
                self.decks.common.push(Card {value: Common, color: *color});
            }
            self.decks.bi.push(Card {value: Bi, color: *color});
            self.decks.tri.push(Card {value: Tri, color: *color});
            self.decks.quad.push(Card {value: Quad, color: *color});
            self.decks.quint.push(Card {value: Quint, color: *color});
        }

        // Shuffle decks
        thread_rng().shuffle(&mut self.decks.common);
        thread_rng().shuffle(&mut self.decks.bi);
        thread_rng().shuffle(&mut self.decks.tri);
        thread_rng().shuffle(&mut self.decks.quad);
        thread_rng().shuffle(&mut self.decks.quint);
    }
    fn initialize_players(&mut self, num_players: u8){
        let mut colors = vec![Red, Orange, Yellow, Purple, Blue, Green];

        match num_players {
            2 => {
                thread_rng().shuffle(&mut colors);
                colors.drain(0..4);
            },
            3 => {
                colors.drain(0..3);
            },
            4 => {
                colors.remove(2);
                colors.remove(5);
            }
            _ => {
                println!("There should only be 2-4 players!");
                return;
            }
        }
        thread_rng().shuffle(&mut colors);

        self.players = colors.into_iter().map(Player::from).collect();
    }
    pub fn new(&mut self, num_players: u8){
        self.initialize_table(num_players);
        self.initialize_players(num_players);
        for player in self.players.iter_mut() {
            player.hand.append(&mut self.decks.common.drain(0..6).collect::<Vec<_>>());
        }
    }
}

fn main(){
}
