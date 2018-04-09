#[macro_use] extern crate lazy_static;

extern crate rand;

extern crate strum;
#[macro_use] extern crate strum_macros;


mod hands;
mod game;

use game::{Game};


fn play_commandline() {
    let mut game = Game::new();

    game.game_loop();
}


fn main() {
    play_commandline();
}
