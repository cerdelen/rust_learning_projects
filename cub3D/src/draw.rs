use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

use piston_window;
use piston_window::*;

const BLOCK_SIZE: f64 = 25.0;

use crate::consts::WINDOW_H;
use crate::consts::WINDOW_W;

pub fn init_window() -> PistonWindow {
	WindowSettings::new(
		"Player",
		[WINDOW_W, WINDOW_H],
	).exit_on_esc(true)
		.build()
		.unwrap()
}
