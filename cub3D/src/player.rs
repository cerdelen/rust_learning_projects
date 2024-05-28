use std::collections::LinkedList;
// use std::intrinsics::cosf64;
use piston_window::{Context, G2d, Key};
use piston_window::types::Color;
use rand::{thread_rng, Rng};
use std::fmt;
use std::ops::AddAssign;
// use crate::draw::draw_block;
use std::f64::consts::PI;

use crate::consts::PLAYER_ROT_ANGLE;
use crate::consts::PLAYER_MOV_SPEED;
use crate::game::{Fields, Map};

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
	pub x: f64,
	pub y: f64,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vector {
	pub fn add(&self, other: &Vector) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y
        }
	}

	pub fn mul(&self, multiplicator: f64) -> Self {
        Vector {
            x: self.x * multiplicator,
            y: self.y * multiplicator
        }
	}

	pub fn get_reverse(&self) -> Self {
		Self {
			x: self.x * -1f64,
			y: self.y * -1f64,
		}
	}

    pub fn rotate_by_angle(&self, angle: f64) -> Vector {
        Vector {
            x:
                (self.x * ((angle * PI / 180f64).cos()))-
                (self.y * ((angle * PI / 180f64).sin())),

            y:
                (self.x * ((angle * PI / 180f64).sin()))+
                (self.y * ((angle * PI / 180f64).cos()))
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
			Key::Left => self.direction = self.direction.rotate_by_angle(PLAYER_ROT_ANGLE),
			Key::Right=> self.direction = self.direction.rotate_by_angle(-PLAYER_ROT_ANGLE),
			_ => panic!("Got into Rotate with a Key that isn't a Rotate Key!\nKey code: {}", key.code()),
		};
	}

	pub fn moves(&mut self, key: Key, map: &Map) {
		if let Some(dest) =  match key {
			Key::W => {
                Some(self.position.add(&self.direction.mul(PLAYER_MOV_SPEED)))
            },
			Key::S => {
                Some(self.position.add(&self.direction.get_reverse().mul(PLAYER_MOV_SPEED)))
            },
			Key::A => {
             Some(self.position.add(&self.direction.rotate_by_angle(90.0).mul(PLAYER_MOV_SPEED)))
            },
			Key::D => {
             Some(self.position.add(&self.direction.rotate_by_angle(-90.0).mul(PLAYER_MOV_SPEED)))
            },
			_ => {
                panic!("Got into Move with a Key that isn't a Move Key!\nKey code: {}", key.code());
            },
		} // get dest in Some
        {
            println!("Dest = {}", dest);
            match map.get_field(&dest) {
                Fields::EMPTY => self.position = dest,
                _ => return ,
            }
        } // check if returned dest is viable to walk towards

	}
}


impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position: ({}, {}), Direction: ({}, {})", self.position.x, self.position.y, self.direction.x, self.direction.y)
    }
}
