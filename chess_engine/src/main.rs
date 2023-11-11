mod game;
use game::Game;

mod consts;
use consts::*;

// use piston_window::{PistonWindow, WindowSettings};
use piston_window::*;

use crate::utils::enums::{PieceType, SelectionState};
use crate::utils::structs::{Board, Game, MyMoverErr, Piece};

pub fn check_move(&self, piece: &Piece, dest_tile: u64) -> Result<(), MyMoverErr> {
    // match piece.piece_type {
    //     PieceType::Rook => print!("Rook"),
    //     PieceType::Bish => print!("Bish"),
    //     PieceType::Knight => print!("Knight"),
    //     PieceType::Queen => print!("Queen"),
    //     PieceType::King => print!("King"),
    //     PieceType::Pawn => print!("Pawn"),
    // }
    // println!(" move {:#066b} to {:#066b}", piece.tile, dest_tile);
    Ok(())
}

pub fn select_piece(&mut self, selection: u64) {
    if let Some(piece) = self.board.get_piece(selection) {
        match piece.piece_type {
            PieceType::Rook => print!("Rook"),
            PieceType::Bish => print!("Bish"),
            PieceType::Knight => print!("Knight"),
            PieceType::Queen => print!("Queen"),
            PieceType::King => print!("King"),
            PieceType::Pawn => print!("Pawn"),
        }
        println!(" selected on tile {:#066b}", selection);
        self.selection = SelectionState::ToMove(piece);
    }
}

pub fn tile_pressed(&mut self, tile: u64) {
    match &self.selection {
        SelectionState::None => {
            self.select_piece(tile);
        }
        SelectionState::ToMove(piece) => {
            if let Ok(_) = self.check_move(piece, tile) {
                self.selection = SelectionState::None;
            }
        }
    }
}

fn coord_to_bitmap(coord: [u32; 2]) -> u64 {
    1 << 63 - ((coord[1] * 8) + coord[0])
}

// println!("highligh_tiles, bit_board: {:#066b}", bit_board);
// idea: have an all peices bitboard in game struct (maybe also all black/ all white)
// so when you wanna check certain stuff you dont have to calc it
// downside ... you need to update it(all 3 of them) every turn
// implement fenn notation
// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR
fn main() {
    let mut window: PistonWindow = WindowSettings::new("ODIOCHESS", [WINDOW_W, WINDOW_H])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // let board = Board::start_board();
    let mut game = Game::new(&window);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // key pressed ( can use this for different modes debugging )
        }
        if let Some(Button::Mouse(mouse)) = event.press_args() {
            if let MouseButton::Left = mouse {
                if let Some(event) = window.next() {
                    if let Some(coord) = event.mouse_cursor_args() {
                        let u_coord: [u32; 2] =
                            [coord[0] as u32 / TILE_S, coord[1] as u32 / TILE_S];
                        game.tile_pressed(coord_to_bitmap(u_coord));
                    }
                }
            }
        }
        window.draw_2d(&event, |c, g, _| game.standard_draw(&c, g));
    }
}
