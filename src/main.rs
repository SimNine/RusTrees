use macroquad::prelude::*;

use crate::rustree::Rustree;

pub mod rustree;

#[macroquad::main("rustrees")]
async fn main() {

    let tree = Rustree::new(100, 200, 30);

    loop {
        clear_background(BLUE);

        tree.draw();

        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
