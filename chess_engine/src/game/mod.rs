mod assets;
mod board;
use assets::{load_assets, PieceTexture};
use board::{Board, Piece, PieceColour, PieceType};
use piston_window::{Context, G2d};
use std::{collections::hash_map::HashMap, error::Error};

use crate::consts::{A, H};

use self::board::MoveSet;

#[derive(Debug, Clone)]
pub struct MyMoverErr;

pub enum SelectionState {
    None,
    ToMove(Piece),
}
pub struct Game {
    board: Board,
    turn_white: bool,
    selection: SelectionState,
    textures: HashMap<(PieceType, PieceColour), PieceTexture>,
}

impl Game {
    fn get_highlight_moveset(&self, mut pos: u64, dest: u64, moveset: i8) -> u64 {
        let mut highlight_set: u64 = 0;
        if pos > dest && moveset < 0 || pos < dest && moveset > 0 {
            while pos != 0 {
                pos = if moveset > 0 {
                    pos << moveset
                } else {
                    pos >> -moveset
                };
                highlight_set = highlight_set | pos;
            }
        }
        return highlight_set;
    }

    fn check_moveset(&self, mut pos: u64, dest: u64, moveset: i8, wrapping_check: bool) -> bool {
        if pos > dest && moveset < 0 || pos < dest && moveset > 0 {
            if wrapping_check {
                while pos != 0 {
                    if moveset > 0 {
                        if pos & A == 0 {
                            return pos == dest;
                        };
                        pos = pos << moveset;
                    } else {
                        if pos & H == 0 {
                            return pos == dest;
                        };
                        pos = pos >> -moveset;
                    };
                    if pos == dest {
                        println!("returning true moveset check");
                        return true;
                    }
                }
            } else {
                while pos != 0 {
                    pos = if moveset > 0 {
                        pos << moveset
                    } else {
                        pos >> -moveset
                    };
                    if pos == dest {
                        println!("returning true moveset check");
                        return true;
                    }
                }
            }
        }
        println!("returning FALSE moveset check");
        return false;
    }

    fn check_move_pattern(&self, piece: &Piece) -> bool {
        let move_pattern = piece.get_move_pattern();



        if self.check_moveset(piece.pos, dest_tile, -8, false) {
            return true;
        }
        if self.check_moveset(piece.pos, dest_tile, 8, false) {
            return true;
        }
        if self.check_moveset(piece.pos, dest_tile, -1, true) {
            return true;
        }
        if self.check_moveset(piece.pos, dest_tile, 1, true) {
            return true;
        }
        return false;
    }

    pub fn accumulate_moves(&self, piece: &Piece, move_pattern: Vec<i8>, once: bool) -> MoveSet {
        let mut move_set = MoveSet { moves: 0};
        for movve in move_pattern {
            move_set.moves = move_set.moves | self.recursive_move(piece, movve, once);
        };
        move_set
    }

    pub fn check_move(&self, piece: &Piece, dest_tile: u64) -> Result<(), MyMoverErr> {
        // check if it cannot go to dest tile (resulting check (pieces was pinned), tile is
        // occuped(by own piece))
        println!("Move is technically possible: {}", technically_possible);
        Ok(())
    }

    pub fn select_piece(&self, selection: u64) -> Option<Piece> {
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
            return Some(piece);
        }
        return None
    }

    pub fn get_possible_moves(&self, piece: &Piece) ->Result<MoveSet, MyMoverErr> {
        // Piece is able to move (even if no possible moves)
        let move_pattern = piece.get_move_pattern();
        let move_set = match piece.piece_type {
            PieceType::Rook | PieceType::Bish | PieceType::Queen => {
                self.accumulate_moves(piece, move_pattern, false)
            }
            _ => {
                self.accumulate_moves(piece, move_pattern, true)
            }
        };
        self.check_move_pattern(piece, move_pattern);
        Ok(MoveSet { moves: 0 })
        // Piece is unable to move (check / pinned)
        // Err(MyMoverErr)


    }

    pub fn tile_pressed(&mut self, tile: u64) {
        match &self.selection {
            // if nothing selected make a selection and highlight possible moves
            SelectionState::None => {
                match self.select_piece(tile) {
                    Some(piece) => {
                        if let Ok(move_set) = self.get_possible_moves(&piece) {
                            println!("if");
                        }
                        else {
                            println!("else");
                        }
                    }
                    None => {
                        println!("no Piece on Tile {:#066b}", tile);
                    }
                };
            }
            // if something was selcted and had possible move now make move
            SelectionState::ToMove(piece) => {
                if let Ok(_) = self.check_move(piece, tile) {
                    self.selection = SelectionState::None;
                }
            }
        }
    }

    pub fn new(window: &mut piston_window::PistonWindow) -> Self {
        let textures: HashMap<(PieceType, PieceColour), PieceTexture> = load_assets(window);

        Self {
            board: Board::new(),
            turn_white: true,
            selection: SelectionState::None,
            textures,
        }
    }

    pub fn standard_draw(&self, c: &Context, g: &mut G2d) {
        self.board.standard_board_draw(c, g, &self.textures);
    }
}
