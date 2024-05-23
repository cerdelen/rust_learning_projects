pub(super) mod bit_boards;
pub(super) mod draw;
use crate::consts::*;

pub struct Piece {
    pub(super) piece_type: PieceType,
    pub(super) colour: PieceColour,
    pub(super) pos: u64,
}

impl Piece {
    pub(super) fn get_move_pattern(&self) -> Vec<i8>{
        match self.piece_type {
            PieceType::Rook => {
                vec![8, -8, 1, -1]
            },
            PieceType::Bish => {
                vec![-9, -7, 7, 9]
            },
            PieceType::Knight => {
                vec![]
            },
            PieceType::Queen => {
                vec![8, -8, 1, -1, -9, -7, 7, 9]
            },
            PieceType::King => {
                vec![1, -1, -9, -8, -7, 7, 8, 9]
            },
            PieceType::Pawn => {
                match self.colour {
                    PieceColour::White => {
                        vec![-9, -8, -7]
                    }
                    PieceColour::Black => {
                        vec![9, 8, 7]
                    }
                }
            },
        }
    }
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

pub struct MoveSet {
    pub moves: u64
}

pub struct Board {
    pub(super) white: PlayerPosition,
    pub(super) black: PlayerPosition,
    pub(super) debugging_highlight: MoveSet,
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
            debugging_highlight: MoveSet { moves: 0 },
        }
    }
}
