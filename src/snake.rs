use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;
use rand::{thread_rng, Rng};

use crate::draw::draw_block;

// i can make this a choce in a menu for example
const SNAKE_COLOUR: Color = [0.000, 0.80, 0.00, 1.0]; // r,g,b,opacity

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction{
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	pub fn opposite(&self) -> Direction {
		match *self {
			Direction::Down => Direction::Up,
			Direction::Up => Direction::Down,
			Direction::Right => Direction::Left,
			Direction::Left => Direction::Right,
		}
	} 
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
	pub x: i32,
	pub y: i32,
}

pub struct Snake {
	direction: Direction,
	body: LinkedList<Position>,
	tail: Option<Position>,
}

impl Snake {
	pub fn next_block(pos: &Position, dir: &Direction) -> Position {
		match dir {
			Direction::Up => {Position{x: pos.x, y: pos.y - 1}},
			Direction::Down => {Position{x: pos.x, y: pos.y + 1}},
			Direction::Left => {Position{x: pos.x - 1, y: pos.y}},
			Direction::Right => {Position{x: pos.x + 1, y: pos.y}},
		}
	}

	pub fn new() -> Snake {
		let mut rng = thread_rng();
		let head = Position {x: rng.gen_range(4..15), y: rng.gen_range(4..15)};
		let dir = rng.gen_range(0..4);
		let mut body = LinkedList::new();
		body.push_back(head);
		let dir:Direction = match dir {
			0 => Direction::Left,
			1 => Direction::Right,
			2 => Direction::Up,
			_ => Direction::Down,
		};
		body.push_back(Self::next_block(body.back().unwrap(), &dir.opposite()));
		body.push_back(Self::next_block(body.back().unwrap(), &dir.opposite()));
		Snake { direction: dir, body, tail: None }
	}

	pub fn draw_snake(&self, con: &Context, g: &mut G2d) {
		for pos in &self.body {
			draw_block(SNAKE_COLOUR, pos.x, pos.y, con, g);
		}
	}

	pub fn head_pos(&self) -> &Position {
		self.body.front().expect("Head gone missing in head_pos")
	}
	
	// for now no "warping"
	

	pub fn move_snake(& mut self, change_dir: Option<Direction>) {
		match change_dir {
			Some(d) => self.direction = d,
			None => (),
		}

		let old_head = self.head_pos();
		self.body.push_front(Snake::next_block(&old_head, &self.direction));
		self.tail = Some(self.body.pop_back().expect("No body"));
	}

	pub fn biting_itself(&self, next_head: &Position) -> bool {
		for body_part in &self.body {
			if *body_part == *next_head {
				if body_part != self.body.back().expect("No body in an iter of body elems"){
					return true;
				}
			}
		};
		false
	}

	pub fn snake_dir(&self) -> Direction {
		self.direction
	}

	pub fn grow(&mut self) {
		self.body.push_back(self.tail.clone().unwrap());
	}
}
