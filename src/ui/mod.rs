use std::{
    fmt,
    io::{BufRead, Write},
};
use crate::{
    game::{
        Game,
        player::Player,
        card::{Deck, Value, Monad, ValueOrMonad},
    }
};

pub struct Ui<Input, Output> {
    pub input: Input,
    pub output: Output,
}

impl<Input: BufRead, Output: Write> Ui<Input, Output> {
    pub fn play(mut self, mut game: Game) {
        let mut buffer = String::new();

        for player in (0..game.players.len()).cycle() {
            let mut action_performed = false;

            writeln!(self.output, "It is now player {}'s turn!", player + 1);
            game.init_turn(player);

            loop {
                buffer.clear();
                write!(self.output, "Actions: show draw flip trade buy leap end \n> ");
                self.output.flush().unwrap();
                self.input.read_line(&mut buffer).unwrap();

                match buffer.trim() {
                    "show" => self.print_state(&game, player),
                    "draw" => {
                        if !action_performed {
                            self.draw(&mut game, player);
                        } else {
                            writeln!(self.output, "You already did something else this turn! You can't draw!");
                        }
                    },
                    "flip" => {
                        if !action_performed {
                            self.flip(&mut game);
                        } else {
                            writeln!(self.output, "You already did something else this turn! You can't flip!");
                        }
                        break;
                    }
                    "trade" => action_performed |= self.trade(&mut game, player).is_ok(),
                    "buy" => action_performed |= self.buy(&mut game, player).is_ok(),
                    "leap" => action_performed |= self.buy(&mut game, player).is_ok(),
                    "end" => break,
                    command => { writeln!(self.output, "Command not recognized: {}", command); },
                }
            }
        }
    }

    fn pick_leap(&mut self, game: &mut Game, player: usize) -> Result<Vec<usize>, String> {
        let player = &mut game.players[player];
        let mut commons: Vec<usize> = Vec::new();

        for i in 0..player.hand.len() {
            if player.hand[i].is_common() {
                commons.push(i);
            }
        }

        if commons.len() < 4 {
            return Err(String::from("Not enough commons to leap!"));
        }

        let num_commons = self.select_num_commons_leap();
        let deck_value = Game::translate_commons_for_leap(num_commons);


        commons = self.select_commons_leap(player, commons, num_commons);
        Ok(commons)
    }

    fn select_num_commons_leap(&mut self) -> usize {
        loop {
            let x = self.prompt_usize(&"Enter how many commons you want to trade! (4: Tri, 5: Quad, 6: Quint)");

            if x < 4 && x > 6 {
                writeln!(self.output, "Invalid selection.");
                continue;
            }

            break x;
        }
    }

    fn select_commons_leap(&mut self, player: &Player, commons: Vec<usize>, num_commons: usize) -> Vec<usize> {
        let mut commons = commons.clone();
        if num_commons == commons.len() {
            return commons;
        }
        let mut translated_decks: Deck = Deck::default();
        loop {
            translated_decks.0 = player.indexes_to_cards(&commons);
            write!(self.output, "Here are all the commons to select. The first {} cards on the left will be traded in.\n", num_commons);
            write!(self.output, "Enter the number of the card to move it left.\n");
            write!(self.output, "Enter {} to accept selection.\n", commons.len());
            write!(self.output, "{}\n", translated_decks);

            let card_num = self.prompt_usize(&"");

            if card_num == commons.len() {
                write!(self.output, "Exiting card selection.\n");
                break;
            }
            if card_num < commons.len() {
                let index = match card_num {
                    0 => 0,
                    n => n-1,
                };
                commons.swap(card_num, index);
            }
            else {
                write!(self.output, "Not a valid selection! Please try again.\n");
            }
        }
        commons.split_off(num_commons);
        commons
    }

    fn draw(&mut self, game: &mut Game, player: usize) {
        if game.draw(player).is_err() {
            writeln!(self.output, "Unable to draw: no commons left");
        }
    }

    fn flip(&mut self, game: &mut Game) {
        if let Err(error) = game.flip() {
            writeln!(self.output, "Unable to flip: {}", error);
        }
    }

    fn trade(&mut self, game: &mut Game, player: usize) -> Result<(), ()> {
        let player_ref = &game.players[player];
        let card1 = self.prompt_hand_selection(game, player, &"Please select the first card to trade!");
        let card2 = self.prompt_hand_selection(game, player, &"Please select the second card to trade!");

        let bonus =
            if player_ref.can_take_bonus(card1, card2) {
                self.prompt_bool(&"Woah! You can take a bonus! Do you want to?")
            } else {
                false
            };

        game
            .trade(player, card1, card2, bonus)
            .map(|(count, monad)| {
                if monad { writeln!(self.output, "You traded for a monad!"); }
                writeln!(self.output, "You traded for {} card(s)!", count);
            })
            .map_err(|error| { writeln!(self.output, "{}", error); })
    }

    fn buy(&mut self, game: &mut Game, player: usize) -> Result<(), ()> {
        let mut cards: Vec<usize> = Vec::new();
        loop {
            cards.push(
                self.prompt_hand_selection(
                    game,
                    player,
                    &"Select a card you want to use to buy!"
                )
            );
            if !self.prompt_bool(&"More cards?") { break; }
        }

        cards.sort();
        cards.dedup();
        let deck_value = self.prompt_value_or_monad(&game);

        let drew_card = game
            .buy(player, &mut cards, deck_value)
            .map_err(|error| { writeln!(self.output, "{}", error); })?;

        writeln!(self.output, "Player bought a {}!", if drew_card { "card" } else { "Monad" });

        Ok(())
    }

    fn leap(&mut self, game: &mut Game, player: usize) -> Result<(), ()> {
        let mut cards = self.pick_leap(game, player)
            .map_err(|error| { writeln!(self.output, "{}", error); })?;

        game
            .leap(player, &mut cards)
            .map(|()| { writeln!(self.output, "Player leapt ahead and drew a card!"); })
            .map_err(|message| { writeln!(self.output, "{}", message); })
    }

    pub fn print_state(&mut self, game: &Game, player: usize) {
        const SEPARATOR: &str = "--------------------";
        let player = &game.players[player];

        write!(
            self.output,
"{separator}
Color: {color}
Hand: {hand},
{separator}
Table: {table}
{separator}
",
            separator = SEPARATOR,
            color = player.identity,
            hand = player.hand,
            table = game.table,
        );
    }

    fn prompt_hand_selection(&mut self, game: &Game, player: usize, message: &(impl fmt::Display)) -> usize {
        let player = &game.players[player];

        loop {
            writeln!(self.output, "{}", message);
            let selection = self.prompt_usize(&player.hand);

            if selection >= player.hand.len() {
                write!(self.output, "{} is not a valid selection!\n", selection);
                continue;
            }

            break selection;
        }
    }

    fn prompt_value_or_monad(&mut self, game: &Game) -> ValueOrMonad {
        let mut buffer = String::new();

        loop {
            write!(self.output, "Select a deck (Common Bi Tri Quad Quint Monad) > ");
            self.output.flush();
            self.input.read_line(&mut buffer).unwrap();

            let trimmed = buffer.trim();
            if let Ok(monad) = trimmed.parse::<Monad>() {
                drop(monad);
                break ValueOrMonad::Monad;
            }

            if let Ok(value) = trimmed.parse::<Value>() {
                if game.table.deck(value).is_empty() {
                    writeln!(self.output, "That deck is out of cards!");
                    continue;
                }

                break ValueOrMonad::Value(value);
            }

            writeln!(self.output, "Invalid input!");
        }
    }

    pub fn prompt_usize(&mut self, message: &(impl fmt::Display)) -> usize {
        let mut buffer = String::new();

        loop {
            write!(self.output, "{} > ", message);
            self.output.flush();
            self.input.read_line(&mut buffer).expect("Problem reading input from user!");

            if let Ok(r) = buffer.trim().parse::<usize>() {
                break r;
            }
            write!(self.output, "What you entered is not an unsigned integer! Please try again.\n");
        }
    }

    pub fn prompt_bool(&mut self, message: &(impl fmt::Display)) -> bool {
        let mut buffer = String::new();

        loop {
            write!(self.output, "{} (yes/no) > ", message);
            self.output.flush();
            self.input.read_line(&mut buffer).unwrap();

            break match buffer.trim() {
                "yes" => true,
                "no" => false,
                _ => {
                    write!(self.output, "Please enter 'yes' or 'no' > ");
                    continue;
                }
            };
        }
    }
}
