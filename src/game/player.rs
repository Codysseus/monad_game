use super::{
    card::{self, Monad, Deck, Card, Color, Value},
    table::Table,
    read_uint_from_user,
};

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
    pub fn trade_value(&self, card1: usize, card2: usize) -> Result<Value, String> {
        let card1 = &self.hand[card1];
        let card2 = &self.hand[card2];

        if card1.temp() == card2.temp() {
            return Err(String::from("Cards should not be the same tempurature!"));
        }

        if card1.value == card2.value {
            return Ok(card1.value);
        }
        
        if self.matches_color(card1) {
            return Ok(card2.value);
        }

        if self.matches_color(card2) {
            return Ok(card1.value);
        }

        Err(String::from("Cards should have the same value when trading! Or if they don't, one should at least be your identity color!"))
    }

    pub fn is_bonus_pair(&self, card1: usize, card2: usize) -> bool {
        use self::Color::*;
        match (self.hand[card1].color, self.hand[card2].color) {
            (Orange, Blue  ) |
            (Blue,   Orange) | 
            (Red,    Purple) | 
            (Purple, Red   ) | 
            (Yellow, Green ) | 
            (Green,  Yellow) => true,
            _                => false,
        }
    }

    pub fn select_card_in_hand(&self) -> Result<usize, String> {
        loop {
            self.print_hand();
            let n = read_uint_from_user();
            if n == 0 {
                break Err(String::from("Exiting hand selection."));
            }
            if n <= self.hand.len() {
                break Ok(n);
            }
            println!("{} is an invalid selection! Please try again.", n);
        }
    }
    pub fn print_hand(&self) {
        let decks_string: String = self.hand
                                       .iter()
                                       .enumerate()
                                       .map(|(i, card)| format!("{}: {} ", i, card))
                                       .collect();
        println!("{}", decks_string);
    }

    pub fn draw_card(&mut self, value: Value, table: &mut Table) -> Option<&Card> {
        if let Some(card) = table.draw_top(value) {
            self.hand.push(card);
            self.hand.last()
        } else {
            None
        }
    }

    pub fn indexes_to_cards(&self, cards: &Vec<usize>) -> Vec<Card> {
        cards.iter().map(|x| self.hand[*x].clone()).collect::<Vec<_>>()
    }

    pub fn draw_monad(&mut self, table: &mut Table) -> Option<()> {
        table.monad.pop().map(|monad| self.monads.push(monad))
    }

    fn matches_color(&self, card: &Card) -> bool {
        self.identity == card.color
    }
}
