pub(super) mod bit_boards;
pub(super) mod draw;
use crate::consts::*;

pub struct Piece {
    piece_type: PieceType,
    tile: u64,
    // colour: PieceCoulour,
}

#[derive(std::hash::Hash, Debug, Clone, PartialEq, Eq)]
pub enum PieceColour {
    White,
    Black,
}

#[derive(std::hash::Hash, Debug, Clone, PartialEq, Eq)]
pub enum PieceType {
    Rook,
    Bish,
    Knight,
    Queen,
    King,
    Pawn,
}

pub struct PlayerPosition {
    pub pawns: u64,
    pub rook: u64,
    pub bishop: u64,
    pub knight: u64,
    pub king: u64,
    pub queen: u64,
}

pub struct Board {
    pub(super) white: PlayerPosition,
    pub(super) black: PlayerPosition,
}

impl Board {
    pub(super) fn new() -> Self {
        Self {
            white: PlayerPosition {
                pawns: A2 | B2 | C2 | D2 | E2 | F2 | G2 | H2,
                rook: A1 | H1,
                knight: B1 | G1,
                bishop: C1 | F1,
                queen: D1,
                king: E1,
            },
            black: PlayerPosition {
                pawns: A7 | B7 | C7 | D7 | E7 | F7 | G7 | H7,
                rook: A8 | H8,
                knight: B8 | G8,
                bishop: C8 | F8,
                queen: D8,
                king: E8,
            },
        }
    }
}
