use crate::utils::structs::Piece;

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

// enum PieceCoulour {
//     Black,
//     White,
// }
pub enum SelectionState {
    None,
    ToMove(Piece),
}
