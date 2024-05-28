use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};
use crate::config::Config;

use crate::player::{Player, Position, Vector};
// use crate::draw::{draw_block, draw_rec};

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
	player: Player,
	map: Map

	// game_over: bool,
	// waiting_time: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fields {
	WALL,
	EMPTY,
	PLAYER,
	OUT_OF_BOUNDS
}

pub struct Map {
	map: Vec<Fields>,
	map_w: usize,
	map_h: usize
}

impl Map {
	pub fn new() -> Self {
		Map {
			map: vec![
				Fields::WALL, Fields::WALL, Fields::WALL, Fields::WALL, Fields::WALL, Fields::WALL,
				Fields::WALL, Fields::EMPTY, Fields::EMPTY, Fields::EMPTY, Fields::EMPTY, Fields::WALL,
				Fields::WALL, Fields::EMPTY, Fields::EMPTY, Fields::EMPTY, Fields::EMPTY, Fields::WALL,
				Fields::WALL, Fields::EMPTY, Fields::EMPTY, Fields::EMPTY, Fields::EMPTY, Fields::WALL,
				Fields::WALL, Fields::EMPTY, Fields::EMPTY, Fields::EMPTY, Fields::EMPTY, Fields::WALL,
				Fields::WALL, Fields::WALL, Fields::WALL, Fields::WALL, Fields::WALL, Fields::WALL,
			],
			map_w: 6,
			map_h: 6
		}
	}

	pub fn get_field(&self, field: &Vector) -> Fields {
		if field.x < 0.0 || field.y < 0.0 {
			return Fields::OUT_OF_BOUNDS;
		}
		let x = field.x as usize;
		let y = field.y as usize;
		if x > self.map_w || y > self.map_h {
			return Fields::OUT_OF_BOUNDS;
		}
		if let Some(field) = self.map.get(y*self.map_w + x) {
			return field.clone();
		}
		Fields::OUT_OF_BOUNDS
	}
}

impl Game {
	pub fn new(config: Config) -> Self {
		Game {player: config.player_start, map: config.map}
	}

	pub fn start_new_game(mut window: PistonWindow, config: Config) {
		let mut game = Self::new(config);
		while let Some(event) = window.next() {
			if let Some(Button::Keyboard(key)) = event.press_args() {
				game.key_press(key);
			}
		// 	window.draw_2d(&event, |c, g, _|{
		// 		clear(BACKGROUND_COLOUR, g);
		// 		game.draw(&c, g)
		// 	});

		// 	event.update(|arg| {
		// 		game.update(arg.dt);
		// 	});
		}
	}


	pub fn key_press(&mut self, key: Key) {
		// if self.game_over {return ;}
		match key {
			Key::Left | Key::Right => self.player.rotate(key),
			Key::W | Key::A | Key::S | Key::D => self.player.moves(key, &self.map),
			_ => println!("Other Key pressed"),
		};
		println!("{}", self.player);
	}

}
