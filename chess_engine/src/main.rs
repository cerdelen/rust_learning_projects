mod game;
use game::Game;

mod consts;
use consts::*;

// use piston_window::{PistonWindow, WindowSettings};
use piston_window::*;

fn coord_to_bitmap(coord: [u32; 2]) -> u64 {
    1 << 63 - ((coord[1] * 8) + coord[0])
}

// println!("highligh_tiles, bit_board: {:#066b}", bit_board);
// idea: have an all peices bitboard in game struct (maybe also all black/ all white)
// so when you wanna check certain stuff you dont have to calc it
// downside ... you need to update it(all 3 of them) every turn
// implement fenn notation
// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR
fn main() {
    let mut window: PistonWindow = WindowSettings::new("ODIOCHESS", [WINDOW_W, WINDOW_H])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // let board = Board::start_board();
    let mut game = Game::new(&mut window);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(_key)) = event.press_args() {
            // key pressed ( can use this for different modes debugging )
        }
        if let Some(Button::Mouse(mouse)) = event.press_args() {
            if let MouseButton::Left = mouse {
                if let Some(event) = window.next() {
                    if let Some(coord) = event.mouse_cursor_args() {
                        let u_coord: [u32; 2] =
                            [coord[0] as u32 / TILE_S, coord[1] as u32 / TILE_S];
                        game.tile_pressed(coord_to_bitmap(u_coord));
                    }
                }
            }
        }
        window.draw_2d(&event, |c, g, _| game.standard_draw(&c, g));
    }
}
