use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn scale_coordinate(cord: i32) -> f64
{
	(cord as f64) * BLOCK_SIZE
}

pub fn scale_coordinate_u32(cord: i32) -> u32
{
	scale_coordinate(cord) as u32
}

pub fn draw_block(col: Color, x: i32, y: i32, con: &Context, g: &mut G2d)
{
	rectangle(
		col,
		[scale_coordinate(x), scale_coordinate(y), BLOCK_SIZE, BLOCK_SIZE],
		con.transform,
		g
	);
}

pub fn draw_rec(col: Color, x: i32, y: i32, w:i32, h:i32, con: &Context, g: &mut G2d)
{
	rectangle(
		col,
		[scale_coordinate(x), scale_coordinate(y), BLOCK_SIZE * (w as f64), BLOCK_SIZE * (h as f64)],
		con.transform,
		g
	);
}