mod constants;
mod entities;
mod game;
mod math;
mod levels;
mod globals;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.update();
}
