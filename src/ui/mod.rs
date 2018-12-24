use std::io::{Read, Write};
use ::game::{Game, read_uint_from_user};
use game::card::{Value, Deck, Card};
use game::player::Player;

pub fn play(game: &mut Game, num_players: usize, mut input: impl Read, mut output: impl Write) {
    for player in (0..num_players).into_iter().cycle() {
        let mut can_play = true;
        write!(output, "It is now player {}'s turn!\n", player);
        game.init_turn(player);

        loop {
            write!(output, "Do you want to 0: Print State, 1: Draw, 2: Flip, 3: Trade, 4: Buy, 5: Leap, 6: End Turn?\n");
            write!(output, "> ");
            output.flush().unwrap();

            match read_uint_from_user() {
                0 => { game.print_state(player); },
                1 => {
                    if can_play {
                        if let Err(message) = game.draw(player) {
                            write!(output, "{}\n", message);
                            continue;
                        }
                        break;
                    }
                    write!(output, "You already did something else this turn! You can't draw!\n");
                },
                2 => {
                    if can_play {
                        if let Err(message) = game.flip() {
                            write!(output, "{}\n", message);
                            continue;
                        }
                        break;
                    }
                    write!(output, "You already did something else this turn! You can't flip!\n");
                },
                3 => {
                    let (card1, card2, bonus) = match pick_trade(&mut output, game, player) {
                        Ok(result) => result,
                        Err(message) => { write!(output, "{}\n", message); continue; },
                    };
                    let (cards, monad_drawn) = match game.trade(player, card1, card2, bonus) {
                        Ok(result) => result,
                        Err(message) => { write!(output, "{}\n", message); continue; },
                    };
                    if monad_drawn {
                        write!(output, "Player traded for a monad!\n");
                    }
                    write!(output, "You traded for {} card(s)!\n", cards);
                    can_play = false;
                },
                4 => {
                    let (mut cards, deck_value) = match pick_buy(&mut output, game, player) {
                        Ok(result)   => result,
                        Err(message) => { write!(output, "{}\n", message); continue; },
                    };
                    let drew_card = match game.buy(player, &mut cards, deck_value) {
                        Ok(result)   => result,
                        Err(message) => { write!(output, "{}\n", message); continue; }
                    };
                    if drew_card {
                        write!(output, "Player bought a card!\n");
                    }
                    else {
                        write!(output, "Player bought a monad!\n");
                    }
                    can_play = false;
                },
                5 => {
                    let mut cards = match pick_leap(&mut output, game, player){
                        Ok(result)   => result,
                        Err(message) => { write!(output, "{}\n", message); continue; },
                    };
                    if let Err(message) = game.leap(player, &mut cards) {
                        write!(output, "{}\n", message);
                        continue;
                    }
                    write!(output, "Player leapt ahead and drew a card!\n");
                    can_play = false;
                },
                6 => {
                    break;
                },
                _ => { write!(output, "That's not a valid selection!\n"); },
            }
        }
    }
}

fn pick_trade(output: &mut impl Write, game: &mut Game, player: usize) -> Result<(usize, usize, bool), String> {
    let pobj = &game.players[player];
    write!(output, "Please select the first card to trade!\n");
    let card1 = select_card_hand(output, &pobj)?;
    write!(output, "Please select the second card to trade!\n");
    let card2 = select_card_hand(output, &pobj)?;
    write!(output, "If these cards are a bonus, will you take it? (0: no, 1: yes)\n");
    let bonus = read_uint_from_user() == 1;
    Ok((card1, card2, bonus))
}

fn pick_buy(output: &mut impl Write, game: &mut Game, player: usize) -> Result<(Vec<usize>, Option<Value>), String> {
    let player = &mut game.players[player];
    let mut cards: Vec<usize> = Vec::new();
    loop {
        write!(output, "Select a card you want to use to buy! Enter {} to exit selection.\n", player.hand.len());
        output.flush();

        match select_card_hand(output, player) {
            Ok(card)     => cards.push(card),
            Err(message) => {
                if cards.is_empty() {
                    return Err(String::from("No cards selected! Exiting buying mode."));
                }
                write!(output, "{} Let's see if you can buy anything with this!\n", message);
                output.flush();
                break;
            },
        };
    }
    cards.dedup();
    let deck_value = select_deck_value(output)?;
    Ok((cards, deck_value))
}

fn pick_leap(output: &mut impl Write, game: &mut Game, player: usize) -> Result<Vec<usize>, String> {
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

    let num_commons = select_num_commons_leap(output)?;
    let deck_value = Game::translate_commons_for_leap(num_commons);


    commons = select_commons_leap(output, player, commons, num_commons);
    Ok(commons)
}

fn select_num_commons_leap(output: &mut impl Write) -> Result<usize, String> {
    loop {
        write!(output, "Enter how many commons you want to trade! (4: Tri, 5: Quad, 6: Quint, 7: Exit)\n");
        write!(output, "> ");
        output.flush();

        let x = read_uint_from_user();
        if x == 7 {
            break Err(String::from("You have decided not to leap! Exiting..."));
        }
        if x > 3 && x < 7 {
            break Ok(x);
        }
        write!(output, "That is an incorrect selection!\n");
    }
}

fn select_commons_leap(output:&mut impl Write, player: &Player, commons: Vec<usize>, num_commons: usize) -> Vec<usize> {
    let mut commons = commons.clone();
    if num_commons == commons.len() {
        return commons;
    }
    let mut translated_decks: Deck = Deck::default();
    loop {
        translated_decks.0 = player.indexes_to_cards(&commons);
        write!(output, "Here are all the commons to select. The first {} cards on the left will be traded in.\n", num_commons);
        write!(output, "Enter the number of the card to move it left.\n");
        write!(output, "Enter {} to accept selection.\n", commons.len());
        write!(output, "{}\n", translated_decks);
        write!(output, "> ");
        output.flush();

        let card_num = read_uint_from_user();
        if card_num == commons.len() {
            write!(output, "Exiting card selection.\n");
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
            write!(output, "Not a valid selection! Please try again.\n");
        }
    }
    commons.split_off(num_commons);
    commons
}

fn select_card_hand(output: &mut impl Write, player: &Player) -> Result<usize, String> {
    loop {
        write!(output, "{}\n", player.hand);
        write!(output, "> ");
        output.flush().unwrap();
        let card = read_uint_from_user();
        if card > player.hand.len() {
            write!(output, "{} is not a valid selection!\n", card);
            continue;
        }
        if card == player.hand.len() {
            break Err(String::from("Exiting hand selection.."));
        }
        break Ok(card);
    }
}

fn select_deck_value(output: &mut impl Write) -> Result<Option<Value>, String> {
    use game::card::Value::*;
    write!(output, "Please select a deck to buy from!\n");
    loop {
        write!(output, "0: Common, 1: Bi, 2: Tri, 3: Quad, 4: Quint, 5: Monad, 6: Exit\n");
        write!(output, "> ");
        output.flush();

        let value = match read_uint_from_user() {
            0 => Some(Common),
            1 => Some(Bi),
            2 => Some(Tri),
            3 => Some(Quad),
            4 => Some(Quint),
            5 => None,
            6 => {
                break Err(String::from("Exiting deck selection..."));
            }
            n => {
                write!(output, "{} is an invalid selection! Please try again.", n);
                continue;
            }
        };
        break Ok(value);
    }
}


fn extract_value(res: Result<String, String>) -> String {
    match res {
        Ok(msg)  => msg,
        Err(msg) => msg,
    }
}

pub fn print_state(game: &Game, player: usize) {
    println!("{}", "-".repeat(20));
    println!("Player color: {}", game.players[player].identity);
    println!("Player {}'s hand: ", player);
    game.players[player].print_hand();
    println!("{}", "-".repeat(20));
    println!("Table state!");
    game.table.print_decks();
    println!("{}", "-".repeat(20));
}
