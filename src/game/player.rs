use super::{
    card::{self, Monad, Deck, Card, Color, Value},
    table::Table,
    TradeError,
};

pub struct Player {
    pub hand: Deck,
    pub identity: Color,
    pub took_bonus: bool,
    pub monads: Vec<Monad>,
}

impl From<card::Color> for Player {
    fn from(color: Color) -> Self {
        Player {
            hand: Deck::default(),
            identity: color,
            took_bonus: false,
            monads: Vec::new(),
        }
    }
}

impl Player {
    pub fn trade_value(&self, card1: usize, card2: usize) -> Result<Value, TradeError> {
        let card1 = &self.hand[card1];
        let card2 = &self.hand[card2];

        if card1.temp() == card2.temp() {
            return Err(TradeError::SameTemperature);
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

        Err(TradeError::NotSameValueOrIdentity)
    }

    pub fn can_take_bonus(&self, card1: usize, card2: usize) -> bool {
        use self::Color::*;
        let bonus_match = match (self.hand[card1].color, self.hand[card2].color) {
            (Orange, Blue  ) |
            (Blue,   Orange) |
            (Red,    Purple) |
            (Purple, Red   ) |
            (Yellow, Green ) |
            (Green,  Yellow) => true,
            _                => false,
        };
        // Not only do the colors need to match a bonus pair, but you can't use a wild to get a bonus.
        bonus_match && self.hand[card1].value == self.hand[card2].value && ! self.took_bonus
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
