use crate::consts::*;
use crate::game::{board::PlayerPosition, Board, Piece, PieceColour, PieceType};

impl Board {
    pub fn get_piece(&self, pos: u64) -> Option<Piece> {
        if let Some(piece) = self.get_white_piece(pos) {
            return Some(piece);
        }
        if let Some(piece) = self.get_black_piece(pos) {
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

    pub fn get_black_piece(&self, pos: u64) -> Option<Piece> {
        let colour = PieceColour::Black;
        if self.black.rook & pos != 0 {
            return Some(Piece {
                piece_type: PieceType::Rook,
                colour,
                pos,
            });
        }
        if self.black.bishop & pos != 0 {
            return Some(Piece {
                piece_type: PieceType::Bish,
                colour,
                pos,
            });
        }
        if self.black.knight & pos != 0 {
            return Some(Piece {
                piece_type: PieceType::Knight,
                colour,
                pos,
            });
        }
        if self.black.queen & pos != 0 {
            return Some(Piece {
                piece_type: PieceType::Queen,
                colour,
                pos,
            });
        }
        if self.black.king & pos != 0 {
            return Some(Piece {
                piece_type: PieceType::King,
                colour,
                pos,
            });
        }
        if self.black.pawns & pos != 0 {
            return Some(Piece {
                piece_type: PieceType::Pawn,
                colour,
                pos,
            });
        }
        None
    }
    pub fn get_white_piece(&self, pos: u64) -> Option<Piece> {
        let colour = PieceColour::White;
        if self.white.rook & pos != 0 {
            return Some(Piece {
                piece_type: PieceType::Rook,
                colour,
                pos,
            });
        }
        if self.white.bishop & pos != 0 {
            return Some(Piece {
                piece_type: PieceType::Bish,
                colour,
                pos,
            });
        }
        if self.white.knight & pos != 0 {
            return Some(Piece {
                piece_type: PieceType::Knight,
                colour,
                pos,
            });
        }
        if self.white.queen & pos != 0 {
            return Some(Piece {
                piece_type: PieceType::Queen,
                colour,
                pos,
            });
        }
        if self.white.king & pos != 0 {
            return Some(Piece {
                piece_type: PieceType::King,
                colour,
                pos,
            });
        }
        if self.white.pawns & pos != 0 {
            return Some(Piece {
                piece_type: PieceType::Pawn,
                colour,
                pos,
            });
        }
        None
    }
}
