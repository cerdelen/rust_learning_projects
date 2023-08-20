use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use crate::snake::{Snake, Direction, Position};
use crate::draw::{draw_block, draw_rec};

const FOOD_COLOUR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOUR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOUR: Color = [0.90, 0.00, 0.00, 0.5];
// const BACKGROUND_COLOUR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
	snake: Snake,

	food_exists: bool,
	food_pos: Position,

	w: i32,
	h: i32,

	game_over: bool,
	waiting_time: f64,
}

impl Game {
	pub fn new(w: i32, h: i32) -> Game {
		Game {
			snake: Snake::new(),
			food_exists: true,
			food_pos: Position {x: 6, y: 6},
			w,
			h,
			game_over: false,
			waiting_time: 0.0
		}
	}

	pub fn key_press(&mut self, key: Key) {
		if self.game_over {return ;}

		let dir = match key {
			Key:: Up => Some(Direction::Up),
			Key:: Down => Some(Direction::Down),
			Key:: Left => Some(Direction::Left),
			Key:: Right => Some(Direction::Right),
			_ => None
		};

		if dir.unwrap() == self.snake.snake_dir().opposite() {
			return ;
		}

		self.update_snake(dir);
	}

	fn fill_whole_scren(&self, col: Color, con: &Context, g: &mut G2d) {
		draw_rec(col, 0, 0, self.w, self.h, con, g);
	}

	fn draw_border(&self, con: &Context, g: &mut G2d) {
		draw_rec(BORDER_COLOUR, 0, 0, self.w, 1, con, g);
		draw_rec(BORDER_COLOUR, 0, self.h - 1, self.w, 1, con, g);
		draw_rec(BORDER_COLOUR, 0, 0, 1, self.h, con, g);
		draw_rec(BORDER_COLOUR, self.w - 1, 0, 1, self.h, con, g);
	}

	pub fn draw(&self, con: &Context, g: &mut G2d ) {
		if self.game_over {
			self.fill_whole_scren(GAMEOVER_COLOUR, con, g);
			return;
		}
		
		// self.fill_whole_scren(BACKGROUND_COLOUR, con, g);

		if self.food_exists {
			draw_block(FOOD_COLOUR, self.food_pos.x, self.food_pos.y, con, g);
		}
		self.draw_border(con, g);
		self.snake.draw_snake(con, g);

	}

	pub fn update(&mut self, delta_time: f64) {
		self.waiting_time += delta_time;
		
		if self.game_over {
			if self.waiting_time > RESTART_TIME {
				self.restart();
			}
			return;
		}

		if !self.food_exists {
			self.spawn_food();
		}

		if self.waiting_time > MOVING_PERIOD {
			self.update_snake(None);
		}
	}

	fn check_eating(&mut self) {
		let head = self.snake.head_pos();
		if self.food_exists && self.food_pos == *head {
			self.food_exists = false;
			self.snake.grow();
		}
	}

	fn out_of_bounds(&self, pos: &Position) -> bool {
		pos.x > self.w - 2 || pos.x < 1 || pos.y > self.h - 2 || pos.y < 1
	}

	fn check_if_death(&self) -> bool {
		let next_head =  Snake::next_block(self.snake.head_pos(), &self.snake.snake_dir());

		if self.snake.biting_itself(&next_head) {
			return true;
		}

		self.out_of_bounds(&next_head)
	}

	fn random_coor_within_bounds(&self) -> Position{
		let mut rng = thread_rng();
		Position {x: rng.gen_range(1..self.w - 1), y: rng.gen_range(1..self.h - 1)}
	}

	fn spawn_food(&mut self) {
		let mut food_coord = self.random_coor_within_bounds();

		while self.snake.biting_itself(&food_coord) {
			food_coord = self.random_coor_within_bounds();
		}

		self.food_pos = food_coord;
		self.food_exists = true;
	}

	fn update_snake(&mut self, dir: Option<Direction>) {
		if self.check_if_death() {
			self.game_over = true
		} else {
			self.snake.move_snake(dir);
			self.check_eating();
		}

		self.waiting_time = 0.0;
	}

	fn restart(&mut self) {
		self.snake = Snake::new();
		self.waiting_time = 0.0;
		self.food_exists = true;
		self.food_pos = self.random_coor_within_bounds();
		self.game_over = false;
	}
}