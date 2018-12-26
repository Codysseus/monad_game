mod game;
mod ui;

use std::{
    env::args,
    io::{stdin, stdout},
};

use crate::{
    ui::Ui,
    game::Game,
};

fn main() {
    let num_players = args()
        .skip(1)
        .next()
        .expect("First argument should be the number of players")
        .parse()
        .expect("Unable to parse number of players");

    let (stdin, stdout) = (stdin(), stdout());
    let ui = Ui { input: stdin.lock(), output: stdout.lock() };
    ui.play(Game::new(num_players));
}
