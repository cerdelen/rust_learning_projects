use crate::board::Board;
use crate::{consts::*, coord_to_bitmap, PieceColour};
use crate::{PieceTexture, PieceType};
use piston_window::types::Vec2d;
use piston_window::*;
use std::collections::HashMap;

impl Board {
    fn draw_board_background(&self, c: &Context, g: &mut G2d) {
        clear([0.0, 0.0, 0.0, 1.0], g);
        for x in 0..8 {
            for y in 0..8 {
                let x_start = (TILE_S * x) as f64;
                let y_start = (TILE_S * y) as f64;
                let colour = match (x, y) {
                    (0 | 2 | 4 | 6, 0 | 2 | 4 | 6) | (1 | 3 | 5 | 7, 1 | 3 | 5 | 7) => WHITE_TILE,
                    _ => DARK_TILE,
                };
                rectangle(
                    colour,
                    [
                        x_start,
                        y_start,
                        x_start + TILE_S as f64,
                        y_start + TILE_S as f64,
                    ],
                    c.transform,
                    g,
                );
            }
        }
    }

    pub fn highligh_tiles(&self, bit_board: u64, c: &Context, g: &mut G2d) {
        for x in 0..8 {
            for y in 0..8 {
                let shift = 63 - (x * 8 + y);
                if ((bit_board >> shift) & 1) == 1 {
                    rectangle(
                        RED_HIGLITED_TILE,
                        [
                            y as f64 * TILE_S as f64,
                            x as f64 * TILE_S as f64,
                            TILE_S as f64,
                            TILE_S as f64,
                        ],
                        c.transform,
                        g,
                    );
                }
            }
        }
    }

    fn draw_piece_type(
        c: &Context,
        g: &mut G2d,
        map: &HashMap<(PieceType, PieceColour), PieceTexture>,
        coord: [f64; 2],
        piece: &(PieceType, PieceColour),
    ) {
        let tex = map.get(piece).unwrap();
        let pos: Vec2d = Vec2d::from([
            coord[0] as f64 * TILE_S as f64,
            coord[1] as f64 * TILE_S as f64,
        ]);
        let size: Vec2d = Vec2d::from([
            (TILE_S as f64 / tex.tex.get_height() as f64) as f64,
            (TILE_S as f64 / tex.tex.get_width() as f64) as f64,
        ]);
        let pos_con = c.trans_pos(pos);
        let final_con = pos_con.scale_pos(size);
        piston_window::image(&tex.tex, final_con.transform, g);
    }

    fn get_full_piece_type(&self, bit_board: u64) -> Option<(PieceType, PieceColour)> {
        if bit_board & self.black.pawns != 0 {
            return Some((PieceType::Pawn, PieceColour::Black));
        }
        if bit_board & self.black.king != 0 {
            return Some((PieceType::King, PieceColour::Black));
        }
        if bit_board & self.black.queen != 0 {
            return Some((PieceType::Queen, PieceColour::Black));
        }
        if bit_board & self.black.knight != 0 {
            return Some((PieceType::Knight, PieceColour::Black));
        }
        if bit_board & self.black.bishop != 0 {
            return Some((PieceType::Bish, PieceColour::Black));
        }
        if bit_board & self.black.rook != 0 {
            return Some((PieceType::Rook, PieceColour::Black));
        }
        if bit_board & self.white.pawns != 0 {
            return Some((PieceType::Pawn, PieceColour::White));
        }
        if bit_board & self.white.king != 0 {
            return Some((PieceType::King, PieceColour::White));
        }
        if bit_board & self.white.queen != 0 {
            return Some((PieceType::Queen, PieceColour::White));
        }
        if bit_board & self.white.knight != 0 {
            return Some((PieceType::Knight, PieceColour::White));
        }
        if bit_board & self.white.bishop != 0 {
            return Some((PieceType::Bish, PieceColour::White));
        }
        if bit_board & self.white.rook != 0 {
            return Some((PieceType::Rook, PieceColour::White));
        }
        None
    }
    fn draw_pieces(
        &self,
        c: &Context,
        g: &mut G2d,
        map: &HashMap<(PieceType, PieceColour), PieceTexture>,
    ) {
        for x in 0..8 {
            for y in 0..8 {
                if let Some(piece) = self.get_full_piece_type(1 << 63 - ((y * 8) + x)) {
                    Self::draw_piece_type(c, g, map, [x as f64, y as f64], &piece);
                }
            }
        }
    }

    pub fn standard_board_draw(
        &self,
        c: &Context,
        g: &mut G2d,
        map: &HashMap<(PieceType, PieceColour), PieceTexture>,
    ) {
        self.draw_board_background(c, g);
        // self.highligh_tiles(self.all_pieces(), c, g);
        self.draw_pieces(c, g, map);
    }
}
