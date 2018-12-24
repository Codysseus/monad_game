use std::{
    fmt,
    io::{BufRead, Write},
};
use crate::{
    game::{
        Game,
        player::Player,
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

            writeln!(self.output, "It is now player {}'s turn!", player);
            game.init_turn(player);

            loop {
                write!(self.output, "Actions: print draw flip trade buy leap end \n> ");
                self.output.flush().unwrap();
                self.input.read_line(&mut buffer).unwrap();

                match buffer.trim() {
                    "print" => self.print_state(&game, player),
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
                    },
                    "trade" => action_performed |= self.trade(&mut game, player).is_ok(),
                    "buy" => action_performed |= self.buy(&mut game, player).is_ok(),
                    "leap" => action_performed |= self.leap(&mut game, player).is_ok(),
                    "end" => break,
                    string => { writeln!(self.output, "{}: unrecognized command", string); },
                };
            }
        }
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
        write!(self.output, "Please select the first card to trade!\n");
        let card1 = self.select_card_hand(player_ref)?;
        write!(self.output, "Please select the second card to trade!\n");
        let card2 = self.select_card_hand(player_ref)?;

        let value = player_ref
            .trade_value(card1, card2)
            .map_err(|message| { writeln!(self.output, "{}", message); })?;

        let bonus =
            if player_ref.can_take_bonus(card1, card2) {
                self.prompt_bool("Woah! You can take a bonus! Do you want to?")
            } else {
                false
            };

        game
            .trade(player, card1, card2, value, bonus)
            .map    (|message| { writeln!(self.output, "{}", message); })
            .map_err(|message| { writeln!(self.output, "{}", message); })
    }

    fn buy(&mut self, game: &mut Game, player: usize) -> Result<(), ()> {
        game
            .buy(player)
            .map_err(|message| { writeln!(self.output, "{}", message); })
    }

    fn leap(&mut self, game: &mut Game, player: usize) -> Result<(), ()> {
        game
            .leap(player)
            .map_err(|message| { writeln!(self.output, "{}", message); })
    }

    fn select_card_hand(&mut self, player: &Player) -> Result<usize, ()> {
        loop {
            write!(self.output, "{}\n> ", player.hand);
            self.output.flush().unwrap();
            let card = self.prompt_usize();

            if card > player.hand.len() {
                write!(self.output, "{} is not a valid selection!\n", card);
                continue;
            }
            if card == player.hand.len() {
                writeln!(self.output, "Exiting hand selection.");
                break Err(());
            }
            break Ok(card);
        }
    }

    pub fn print_state(&mut self, game: &Game, player: usize) {
        const SEPARATOR: &str = "--------------------";
        let player = game.players[player];

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

    pub fn prompt_usize(&mut self) -> usize {
        let mut buffer = String::new();
        loop {
            self.input.read_line(&mut buffer).expect("Problem reading input from user!");
            if let Ok(r) = buffer.trim().parse::<usize>() {
                break r;
            }
            write!(self.output, "What you entered is not an unsigned integer! Please try again.\n");
        }
    }

    pub fn prompt_bool(&mut self, message: impl fmt::Display) -> bool {
        let mut buffer = String::new();
        loop {
            write!(self.output, "{} (yes/no) > ", message);
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
