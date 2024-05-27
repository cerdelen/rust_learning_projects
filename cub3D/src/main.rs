use piston_window;
// use rand;
mod config;
mod game;
mod draw;
mod player;
mod consts;


use std::env;

use config::Config;
use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;

// use crate::draw::scale_coordinate_u32;

const BACKGROUND_COLOUR: Color = [0.5, 0.5, 0.5, 1.0];

// bot values > 18 mandatory
const GAME_W: i32 = 25;
const GAME_H: i32 = 25;

fn main()
{
    let conf = Config::parse(env::args().collect()).expect("Config Error!");
    let window: PistonWindow = draw::init_window();

    Game::start_new_game(window, conf);

    println!("Game closed Gracefully!");
}