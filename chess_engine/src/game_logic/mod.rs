use crate::utils::enums::{PieceType, SelectionState};
use crate::utils::structs::{Board, Game, MyMoverErr, Piece};

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::start_board(),
            turn_white: true,
            selection: SelectionState::None,
        }
    }

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
}
