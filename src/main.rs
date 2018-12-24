use std::{
    env::args,
    io::{stdin, stdout},
};
mod game;
mod ui;


fn main() {
    let num_players: usize = args()
        .skip(1)
        .next()
        .expect("First argument should be the number of players")
        .parse()
        .expect("Unable to parse number of players");

    let game = match game::Game::new(num_players){
        Ok(g) => g,
        Err(message) => {
            println!("{}", message);
            return;
        },
    };
    let ui = ui::Ui { input: stdin().lock(), output: stdout().lock() };
    ui.play(game);
}
