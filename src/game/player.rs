use super::card::{self, Monad, Deck, Color, Value};
use super::read_uint_from_user;

pub struct Player {
    pub hand: Deck,
    pub identity: Color,
    pub monads: Vec<Monad>,
}

impl From<card::Color> for Player {
    fn from(color: Color) -> Self {
        Player {
            identity: color,
            hand: Deck::default(),
            monads: Vec::new(),
        }
    }
}

impl Player {
    pub fn get_trade_value(&self, card1: usize, card2: usize) -> Result<Value, String> {
        let card1 = &self.hand[card1];
        let card2 = &self.hand[card2];

        if card1.get_temp() == card2.get_temp() {
            return Err(String::from("Cards should not be the same tempurature!"));
        }
        if card1.0 != card2.0 {
            if card1.1 != self.identity && card2.1 != self.identity {
                return Err(String::from("Cards should have the same value when trading! Or if they don't, one should at least be your identity color!"));
            }
            else {
                return if card1.1 == self.identity {
                    Ok(card2.0)
                }
                else {
                    Ok(card1.0)
                }
            }
        }
        return Ok(card1.0);
    }
    pub fn is_bonus_pair(&self, card1: usize, card2: usize) -> bool {
        use self::Color::*;
        match (self.hand[card1].1, self.hand[card2].1){
            (Orange, Blue)   => true,
            (Blue,   Orange) => true,
            (Red,    Purple) => true,
            (Purple, Red)    => true,
            (Yellow, Green)  => true,
            (Green,  Yellow) => true,
            (_, _)           => false,
        }
    }
    pub fn select_card_in_hand(&self) -> Result<usize, String> {
        loop {
            let n = read_uint_from_user();
            if n <= self.hand.len() + 1 {
                if n == self.hand.len() + 1 {
                    return Err(String::from("Exiting hand selection."));
                }
                break Ok(n);
            }
            println!("{} is an invalid selection! Please try again.", n);
        }
    }
}
