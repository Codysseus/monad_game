use std::io::{Read, Write};
use ::game::{Game, read_uint_from_user};
use game::card::Value;
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
                    let result = trade(&mut output, game, player);
                    can_play = result.is_err();
                    write!(output, "{}\n", extract_value(result));
                },
                4 => {
                    if let Err(message) = game.buy(player) {
                        write!(output, "{}\n", message);
                    }
                    else { can_play = false; }
                },
                5 => {
                    if let Err(message) = game.leap(player) {
                        write!(output, "{}\n", message);
                    }
                    else {
                        can_play = false;
                    }
                },
                6 => {
                    break;
                },
                _ => { write!(output, "That's not a valid selection!\n"); },
            }
        }
    }
}

fn trade(output: &mut impl Write, game: &mut Game, player: usize) -> Result<String, String> {
    let mut card1: usize;
    let mut card2: usize;
    let mut value: Value;
    let mut bonus = false;
    {
        let pobj = &game.players[player];
        write!(output, "Please select the first card to trade!\n");
        card1 = select_card_hand(output, &pobj)?;
        write!(output, "Please select the second card to trade!\n");
        card2 = select_card_hand(output, &pobj)?;

        value = pobj.trade_value(card1, card2)?;

        if pobj.can_take_bonus(card1, card2) {
            write!(output, "Woah! You can take a bonus! Do you want to? (0: no, 1: yes)\n");
            bonus = read_uint_from_user() == 1;
        }
    }
    game.trade(player, card1, card2, value, bonus)
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
