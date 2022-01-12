extern crate raylib;

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Rust Demo - Standalone app showcasing a Raylib binding for Rust - Chelsea")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text("Hello, world! Press ESC to exit...", 12, 12, 20, Color::BLACK);
    }
}
