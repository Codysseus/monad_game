extern crate rand;
use std::io::{stdin, stdout, Write};
mod game;
mod ui;

fn main() {
    println!("Hello and welcome to the game of monad!");
    print!("Choose the number of players: ");
    stdout().flush();

    let num_players = game::read_uint_from_user();
    let mut game = match game::Game::new(num_players){
        Ok(g) => g,
        Err(message) => {
            println!("{}", message);
            return;
        },
    };
    ui::play(&mut game, num_players, stdin(), stdout());
}
