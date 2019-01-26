use std::{
    fmt,
    io::{self, BufRead, Write},
};
use crate::game::{
    Game,
    card::{Deck, Value, Monad, ValueOrMonad},
};

pub struct Ui<Input, Output> {
    pub input: Input,
    pub output: Output,
}

impl<Input: BufRead, Output: Write> Ui<Input, Output> {
    pub fn play(mut self, mut game: Game) -> io::Result<()> {
        let mut buffer = String::new();

        for player in (0..game.players.len()).cycle() {
            let mut action_performed = false;

            writeln!(self.output, "It is now player {}'s turn!", player + 1).unwrap();
            *game.player_took_bonus(player) = false;

            loop {
                buffer.clear();
                write!(self.output, "Actions: show draw flip trade buy leap end \n> ").unwrap();
                self.output.flush().unwrap();
                self.input.read_line(&mut buffer).unwrap();

                match buffer.trim() {
                    "show" => self.print_state(&game, player)?,
                    "draw" => {
                        if action_performed {
                            writeln!(self.output, "You can't draw, you already did something else this turn!")?;
                            continue;
                        }
                        self.draw(&mut game, player)?;
                        break;
                    },
                    "flip" => {
                        if action_performed {
                            writeln!(self.output, "You can't flip, you already did something else this turn!")?;
                            continue;
                        }
                        self.flip(&mut game)?;
                        break;
                    },
                    "trade" => action_performed |= self.trade(&mut game, player)?.is_ok(),
                    "buy" => action_performed |= self.buy(&mut game, player)?.is_ok(),
                    "leap" => action_performed |= self.leap(&mut game, player)?.is_ok(),
                    "end" => {
                        if action_performed || game.check_player_end(player) {
                            break;
                        }
                        writeln!(self.output, "You can't end your turn yet! You can still take an action.")?;
                    },
                    command => writeln!(self.output, "Command not recognized: {}", command)?,
                }
            }
        }

        Ok(())
    }


    fn draw(&mut self, game: &mut Game, player: usize) -> io::Result<()> {
        if game.draw(player).is_err() {
            writeln!(self.output, "Unable to draw: no commons left")?;
        }

        Ok(())
    }

    fn flip(&mut self, game: &mut Game) -> io::Result<()> {
        if let Err(error) = game.flip() {
            writeln!(self.output, "Unable to flip: {}", error)?;
        }

        Ok(())
    }

    fn trade(&mut self, game: &mut Game, player: usize) -> io::Result<Result<(), ()>> {
        let player_ref = &game.players[player];
        let card1 = self.prompt_hand_selection(game, player, &"Please select the first card to trade!")?;
        let card2 = self.prompt_hand_selection(game, player, &"Please select the second card to trade!")?;

        let bonus =
            if player_ref.can_take_bonus(card1, card2) {
                self.prompt_bool(&"Woah! You can take a bonus! Do you want to?")?
            } else {
                false
            };

        let result = match game.trade(player, card1, card2, bonus) {
            Ok((count, monad)) => {
                if monad { writeln!(self.output, "You traded for a monad!")?; }
                writeln!(self.output, "You traded for {} card(s)!", count)?;
                Ok(())
            },
            Err(error) => {
                writeln!(self.output, "{}", error)?;
                Err(())
            },
        };

        Ok(result)
    }

    fn buy(&mut self, game: &mut Game, player: usize) -> io::Result<Result<(), ()>> {
        let mut cards: Vec<usize> = Vec::new();
        loop {
            cards.push(
                self.prompt_hand_selection(
                    game,
                    player,
                    &"Select a card you want to use to buy!"
                )?
            );
            if !self.prompt_bool(&"More cards?")? { break; }
        }

        cards.sort();
        cards.dedup();
        let deck_value = self.prompt_value_or_monad(&game)?;

        let drew_card = match game.buy(player, &mut cards, deck_value) {
            Ok(drew_card) => drew_card,
            Err(buy_error) => {
                writeln!(self.output, "{}", buy_error)?;
                return Ok(Err(()));
            },
        };

        writeln!(
            self.output,
            "Player bought a {}!",
            if drew_card { "card" } else { "Monad" }
        )?;

        Ok(Ok(()))
    }

    fn leap(&mut self, game: &mut Game, player: usize) -> io::Result<Result<(), ()>> {
        let mut cards = match self.prompt_leap(game, player)? {
            Ok(cards) => cards,
            Err(()) => {
                return Ok(Err(()));
            }
        };

        let result = match game.leap(player, &mut cards) {
            Ok(()) => {
                writeln!(self.output, "Player leapt ahead and drew a card!")?;
                Ok(())
            },
            Err(error) => {
                writeln!(self.output, "{}", error)?;
                Err(())
            },
        };

        Ok(result)
    }

    fn print_state(&mut self, game: &Game, player: usize) -> io::Result<()> {
        const SEPARATOR: &str = "--------------------";
        let player = &game.players[player];

        write!(
            self.output,
            "{separator}\n\
            Color: {color}\n\
            Hand: {hand}\n\
            {separator}\n\
            Table: {table}\n\
            {separator}\n",
            separator = SEPARATOR,
            color = player.identity,
            hand = player.hand,
            table = game.table,
        )
    }

    fn prompt_bool(&mut self, message: impl fmt::Display) -> io::Result<bool> {
        let mut buffer = String::new();

        loop {
            buffer.clear();
            write!(self.output, "{} (yes/no) > ", message)?;
            self.output.flush()?;
            self.input.read_line(&mut buffer)?;

            break match buffer.trim() {
                "yes" => Ok(true),
                "no" => Ok(false),
                _ => {
                    write!(self.output, "Please enter 'yes' or 'no' > ")?;
                    continue;
                }
            };
        }
    }

    fn prompt_usize(&mut self, message: impl fmt::Display) -> io::Result<usize> {
        let mut buffer = String::new();

        loop {
            buffer.clear();
            write!(self.output, "{}\n> ", message)?;
            self.output.flush()?;
            self.input.read_line(&mut buffer)?;

            if let Ok(r) = buffer.trim().parse::<usize>() {
                break Ok(r);
            }
            writeln!(self.output, "What you entered is not an unsigned integer! Please try again.")?;
        }
    }

    fn prompt_hand_selection(&mut self, game: &Game, player: usize, message: impl fmt::Display) -> io::Result<usize> {
        let player = &game.players[player];

        loop {
            writeln!(self.output, "{}", message)?;
            let selection = self.prompt_usize(&player.hand)?;

            if selection >= player.hand.len() {
                writeln!(self.output, "{} is not a valid selection!", selection)?;
                continue;
            }

            break Ok(selection);
        }
    }

    fn prompt_value_or_monad(&mut self, game: &Game) -> io::Result<ValueOrMonad> {
        let mut buffer = String::new();

        loop {
            buffer.clear();
            write!(self.output, "Select a deck (Common Bi Tri Quad Quint Monad) > ")?;
            self.output.flush()?;
            self.input.read_line(&mut buffer)?;

            let trimmed = buffer.trim();
            if let Ok(monad) = trimmed.parse::<Monad>() {
                drop(monad);
                break Ok(ValueOrMonad::Monad);
            }

            if let Ok(value) = trimmed.parse::<Value>() {
                if game.table.deck(value).is_empty() {
                    writeln!(self.output, "That deck is out of cards!")?;
                    continue;
                }

                break Ok(ValueOrMonad::Value(value));
            }

            writeln!(self.output, "Invalid input!")?;
        }
    }

    fn prompt_leap(&mut self, game: &mut Game, player: usize) -> io::Result<Result<Vec<usize>, ()>> {
        const MIN_COMMONS: usize = 4;
        const MAX_COMMONS: usize = 6;

        let player = &mut game.players[player];

        let mut commons: Vec<usize> = player
            .hand
            .iter()
            .enumerate()
            .filter(|(_, card)| card.is_common())
            .map(|(i, _)| i)
            .collect();

        if commons.len() < MIN_COMMONS {
            writeln!(self.output, "Not enough commons to leap!")?;
            return Ok(Err(()));
        }

        let selected_count = loop {
            let x = self.prompt_usize(&"Enter how many commons you want to trade! (4: Tri, 5: Quad, 6: Quint)")?;

            if x < MIN_COMMONS && x > MAX_COMMONS {
                writeln!(self.output, "Invalid selection.")?;
                continue;
            }

            break x;
        };

        if selected_count == commons.len() {
            return Ok(Ok(commons));
        }

        write!(
            self.output,
            "Here are all the commons to select.\
            The first {} cards on the left will be traded in.\n\
            Enter the number of the card to move it left.\n\
            Enter {} to accept selection.\n",
            selected_count,
            commons.len(),
        )?;
        loop {
            let card_num = self.prompt_usize(&Deck::from(player.indexes_to_cards(&commons)))?;

            if card_num == commons.len() {
                break;
            }
            if card_num < commons.len() {
                let index = card_num.saturating_sub(1);
                commons.swap(card_num, index);
            } else {
                writeln!(self.output, "Not a valid selection! Please try again.")?;
            }
        }

        while commons.len() > selected_count { commons.pop(); }
        Ok(Ok(commons))
    }
}
