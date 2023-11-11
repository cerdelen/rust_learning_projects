mod assets;
mod board;
use assets::{load_assets, PieceTexture};
use board::{Board, Piece, PieceColour, PieceType};
use std::collections::hash_map::HashMap;

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
    pub fn new(window: &piston_window::PistonWindow) -> Self {
        let textures: HashMap<(PieceType, PieceColour), PieceTexture> = load_assets(&mut window);

        Self {
            board: Board::new(),
            turn_white: true,
            selection: SelectionState::None,
            textures,
        }
    }

    pub fn standard_draw(&self, &c, g) {
        self.board.standard_board_draw(c, g, &self.textures);
    }
}
