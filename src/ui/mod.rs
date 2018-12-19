use std::io::{Read, Write};
use ::game::{Game, read_uint_from_user};

pub fn play(game: &mut Game, num_players: usize, mut input: impl Read, mut output: impl Write) {
    for player in (0..num_players).into_iter().cycle() {
        let mut can_play = true;
        println!("It is now player {}'s turn!", player);
        game.init_turn(player);

        loop {
            println!("Do you want to 0: Print State, 1: Draw, 2: Flip, 3: Trade, 4: Buy, 5: Leap, 6: End Turn?");
            print!("> ");
            output.flush();

            match read_uint_from_user() {
                0 => game.print_state(player),
                1 => {
                    if can_play {
                        if let Err(message) = game.draw(player) {
                            println!("{}", message);
                            continue;
                        }
                        break;
                    }
                    println!("You already did something else this turn! You can't draw!");
                },
                2 => {
                    if can_play {
                        if let Err(message) = game.flip() {
                            println!("{}", message);
                            continue;
                        }
                        break;
                    }
                    println!("You already did something else this turn! You can't flip!");
                },
                3 => {
                    if let Err(message) = game.trade(player) {
                        println!("{}", message);
                    }
                    else {
                        can_play = false;
                    }
                },
                4 => {
                    if let Err(message) = game.buy(player) {
                        println!("{}", message);
                    }
                    else {
                        can_play = false;
                    }
                },
                5 => {
                    if let Err(message) = game.leap(player) {
                        println!("{}", message);
                    }
                    else {
                        can_play = false;
                    }
                },
                6 => {
                    break;
                },
                _ => println!("That's not a valid selection!"),
            }
        }
    }
}
