mod constants;
mod entities;
mod game;
mod math;
mod levels;
mod globals;
mod player_ship_controller;

mod app;

fn main() {
    let mut game = game::Game::new();
    // TODO need some input handling and re-rendering
    loop {
        game.update();
    }
}
