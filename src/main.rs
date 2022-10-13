mod constants;
mod entities;
mod game_level;
mod math;
mod spawners;
mod globals;
mod player_ship_controller;

mod app;
mod input_mgr;

fn main() {
    app::App::new().run()
}
