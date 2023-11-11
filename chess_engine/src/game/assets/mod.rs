use crate::utils::enums::*;
use piston_window::{Flip, PistonWindow, Texture, TextureSettings};
use std::collections::HashMap;
// use std::fmt::format;
use std::path::Path;

pub struct PieceTexture {
    pub piece_type: PieceType,
    pub col: PieceColour,
    pub tex: <G2d<'static> as Graphics>::Texture,
}

fn load_asset(
    window: &mut PistonWindow,
    p_type: &PieceType,
    p: &str,
) -> (PieceTexture, PieceTexture) {
    let w = format!("assets/w_{}.png", p);
    let b = format!("assets/b_{}.png", p);
    (
        PieceTexture {
            col: PieceColour::White,
            piece_type: p_type.clone(),
            tex: Texture::from_path(
                &mut window.create_texture_context(),
                Path::new(&w),
                // p_w,
                Flip::None,
                &TextureSettings::new(),
            )
            .unwrap(),
        },
        PieceTexture {
            col: PieceColour::Black,
            piece_type: p_type.to_owned(),
            tex: Texture::from_path(
                &mut window.create_texture_context(),
                Path::new(&b),
                Flip::None,
                &TextureSettings::new(),
            )
            .unwrap(),
        },
    )
}

pub fn load_assets(window: &mut PistonWindow) -> HashMap<(PieceType, PieceColour), PieceTexture> {
    let mut piece_texture_map = HashMap::new();

    let white = PieceColour::White;
    let black = PieceColour::Black;

    println!("before first ");
    let p_type = PieceType::Rook;
    let double_text = load_asset(window, &p_type, "rook");
    piece_texture_map.insert((p_type.clone(), white.clone()), double_text.0);
    piece_texture_map.insert((p_type.clone(), black.clone()), double_text.1);

    println!("got rooks");

    let p_type = PieceType::Bish;
    let double_text = load_asset(window, &p_type, "bish");
    piece_texture_map.insert((p_type.clone(), white.clone()), double_text.0);
    piece_texture_map.insert((p_type.clone(), black.clone()), double_text.1);
    println!("got bishs");

    let p_type = PieceType::Knight;
    let double_text = load_asset(window, &p_type, "knight");
    piece_texture_map.insert((p_type.clone(), white.clone()), double_text.0);
    piece_texture_map.insert((p_type.clone(), black.clone()), double_text.1);
    println!("got knight");

    let p_type = PieceType::Queen;
    let double_text = load_asset(window, &p_type, "queen");
    piece_texture_map.insert((p_type.clone(), white.clone()), double_text.0);
    piece_texture_map.insert((p_type.clone(), black.clone()), double_text.1);
    println!("got queen");

    let p_type = PieceType::King;
    let double_text = load_asset(window, &p_type, "king");
    piece_texture_map.insert((p_type.clone(), white.clone()), double_text.0);
    piece_texture_map.insert((p_type.clone(), black.clone()), double_text.1);
    println!("got king");

    let p_type = PieceType::Pawn;
    let double_text = load_asset(window, &p_type, "pawn");
    piece_texture_map.insert((p_type.clone(), white.clone()), double_text.0);
    piece_texture_map.insert((p_type.clone(), black.clone()), double_text.1);
    println!("got pawn");

    piece_texture_map
}
