use crate::consts::*;
use crate::utils::enums::*;
use crate::utils::structs::*;

impl Board {
    pub fn start_board() -> Self {
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

    pub fn get_piece(&self, tile: u64) -> Option<Piece> {
        if let Some(piece) = self.get_white_piece(tile) {
            return Some(piece);
        }
        if let Some(piece) = self.get_black_piece(tile) {
            return Some(piece);
        }
        None
    }

    pub fn all_black_pieces(&self) -> u64 {
        self.black.rook
            | self.black.bishop
            | self.black.knight
            | self.black.queen
            | self.black.king
            | self.black.pawns
    }

    pub fn all_white_pieces(&self) -> u64 {
        self.white.rook
            | self.white.bishop
            | self.white.knight
            | self.white.queen
            | self.white.king
            | self.white.pawns
    }

    pub fn all_pieces(&self) -> u64 {
        self.all_white_pieces() | self.all_black_pieces()
    }

    pub fn get_black_piece(&self, tile: u64) -> Option<Piece> {
        if self.black.rook & tile != 0 {
            return Some(Piece {
                piece_type: crate::PieceType::Rook,
                tile,
            });
        }
        if self.black.bishop & tile != 0 {
            return Some(Piece {
                piece_type: crate::PieceType::Bish,
                tile,
            });
        }
        if self.black.knight & tile != 0 {
            return Some(Piece {
                piece_type: crate::PieceType::Knight,
                tile,
            });
        }
        if self.black.queen & tile != 0 {
            return Some(Piece {
                piece_type: crate::PieceType::Queen,
                tile,
            });
        }
        if self.black.king & tile != 0 {
            return Some(Piece {
                piece_type: crate::PieceType::King,
                tile,
            });
        }
        if self.black.pawns & tile != 0 {
            return Some(Piece {
                piece_type: crate::PieceType::Pawn,
                tile,
            });
        }
        None
    }
    pub fn get_white_piece(&self, tile: u64) -> Option<Piece> {
        if self.white.rook & tile != 0 {
            return Some(Piece {
                piece_type: crate::PieceType::Rook,
                tile,
            });
        }
        if self.white.bishop & tile != 0 {
            return Some(Piece {
                piece_type: crate::PieceType::Bish,
                tile,
            });
        }
        if self.white.knight & tile != 0 {
            return Some(Piece {
                piece_type: crate::PieceType::Knight,
                tile,
            });
        }
        if self.white.queen & tile != 0 {
            return Some(Piece {
                piece_type: crate::PieceType::Queen,
                tile,
            });
        }
        if self.white.king & tile != 0 {
            return Some(Piece {
                piece_type: crate::PieceType::King,
                tile,
            });
        }
        if self.white.pawns & tile != 0 {
            return Some(Piece {
                piece_type: crate::PieceType::Pawn,
                tile,
            });
        }
        None
    }
}
