extern crate rand;
use rand::{Rng,thread_rng};
use Cardcolor::*;
use Cardvalue::*;

const COLORS: [Cardcolor; 6] = [Red, Orange, Yellow, Purple, Blue, Green];

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
impl Table {
    pub fn new(players: u8) -> Table{
        // The amount of commons is dependent on the number of players
        let players = players as usize;

        // Create vectors for the different decks
        let mut t = Table {
        	discard: Vec::with_capacity(6*players),
        	common: Vec::with_capacity(6*players),
        	bi: Vec::with_capacity(6),
        	tri: Vec::with_capacity(6),
        	quad: Vec::with_capacity(6),
        	quint: Vec::with_capacity(6),
            monad: 12
        };

        // Fill decks with appropriate cards
        for color in COLORS.iter() {
            for _ in 1..players {
                t.common.push(Card {value: Common, color: *color});
            }
            t.bi.push(Card {value: Bi, color: *color});
            t.tri.push(Card {value: Tri, color: *color});
            t.quad.push(Card {value: Quad, color: *color});
            t.quint.push(Card {value: Quint, color: *color});
        }

        // Shuffle
        thread_rng().shuffle(&mut t.common);
        thread_rng().shuffle(&mut t.bi);
        thread_rng().shuffle(&mut t.tri);
        thread_rng().shuffle(&mut t.quad);
        thread_rng().shuffle(&mut t.quint);

        return t;
    }
}
impl Game {
    fn initialize_players(&mut self, num_players: u8){
        let mut colors = COLORS.to_vec().clone();

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
        self.decks = Table::new(num_players);
        self.initialize_players(num_players);
        for player in self.players.iter_mut() {
            player.hand.extend(self.decks.common.drain(0..6));
        }
    }
}

fn main(){
}
