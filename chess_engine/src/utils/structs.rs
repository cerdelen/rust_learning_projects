use crate::utils::enums::{PieceColour, PieceType, SelectionState};
use piston_window::{G2d, Graphics};

pub struct PlayerPosition {
    pawns: u64,
    rook: u64,
    bishop: u64,
    knight: u64,
    king: u64,
    queen: u64,
}

pub struct Board {
    pub(crate) white: PlayerPosition,
    pub(crate) black: PlayerPosition,
}

pub struct Piece {
    piece_type: PieceType,
    tile: u64,
    // colour: PieceCoulour,
}

pub struct Game {
    board: Board,
    turn_white: bool,
    selection: SelectionState,
}

pub struct PieceTexture {
    pub piece_type: PieceType,
    pub col: PieceColour,
    pub tex: <G2d<'static> as Graphics>::Texture,
}
#[derive(Debug, Clone)]
pub struct MyMoverErr;
