use piston_window;
// use rand;

mod game;
mod draw;
mod snake;

use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;

use crate::draw::scale_coordinate_u32;

const BACKGROUND_COLOUR: Color = [0.5, 0.5, 0.5, 1.0];

// bot values > 18 mandatory
const GAME_W: i32 = 25;
const GAME_H: i32 = 25;

fn main()
{
    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [scale_coordinate_u32(GAME_W), scale_coordinate_u32(GAME_H)],
    ).exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(GAME_W, GAME_H);
    while let Some(event) = window.next(){
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_press(key);
        }
        window.draw_2d(&event, |c, g, _|{
            clear(BACKGROUND_COLOUR, g);
            game.draw(&c, g)
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}