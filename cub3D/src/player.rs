use std::collections::LinkedList;
use piston_window::{Context, G2d, Key};
use piston_window::types::Color;
use rand::{thread_rng, Rng};
use std::fmt;
use std::ops::AddAssign;
// use crate::draw::draw_block;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector {
	pub x: i32,
	pub y: i32,
}

impl Vector {
	pub fn add(&mut self, other: &Vector) {
		self.x += other.x;
		self.y += other.y;
	}

	pub fn get_reverse(&self) -> Self {
		Self {
			x: self.x * -1,
			y: self.y * -1,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
	pub x: i32,
	pub y: i32,
}

pub struct Player {
	position: Vector,
	direction: Vector,
}

impl Player {
	pub fn new(position: Vector, direction: Vector) -> Self {
		Self {
			position,
			direction
		}
	}

	pub fn rotate(&mut self, key: Key) {
		match key {
			Key::Left => self.direction.x += 1,
			Key::Right => self.direction.x -=1 ,
			_ => panic!("Got into Rotate with a Key that isn't a Rotate Key!\nKey code: {}", key.code()),
		}
	}

	pub fn moves(&mut self, key: Key) {
		match key {
			Key::W => self.position.add(&self.direction),
			Key::S => self.position.add(&self.direction.get_reverse()),
			Key::A => todo!(),
			Key::D => todo!(),
			_ => panic!("Got into Move with a Key that isn't a Move Key!\nKey code: {}", key.code()),
		} 

	}
}


impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position: ({}, {}), Direction: ({}, {})", self.position.x, self.position.y, self.direction.x, self.direction.y)
    }
}