extern crate raylib;

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Rust Demo - Standalone app showcasing a Raylib binding for Rust - Chelsea")
        .build();

    while !rl.window_should_close() &&
        !rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_W) &&
        !rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_A) &&
        !rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_S) &&
        !rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_D)
    {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text("Hello, world!", 10, 10, 20, Color::BLACK);
        d.draw_text("Press W, A, S, or D to continue...", 10, 30, 20, Color::BLACK);
    }
    
    while !rl.window_should_close()
    {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::RAYWHITE);
        
        d.draw_text("Thank you for playing! Press ESC to exit...", 10, 10, 20, Color::BLACK);
    }
}
