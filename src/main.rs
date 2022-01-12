extern crate raylib;

use raylib::prelude::*;

struct Game_State {
    player_x: i32;
    player_y: i32;
    
    sword_state: i32;
    
    background_color: Color;
}

fn get_background_color(game_state: &Game_State) -> Color
{
    return game_state.background_color;
}

fn main() {
    let screen_width = 800;
    let screen_height = 600;
    let target_fps = 60;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Rust Demo - Standalone app showcasing a Raylib binding for Rust - Chelsea")
        .build();

    rl.set_target_fps(target_fps)
    
    let game_state = GameState { screen_width / 2, screen_height / 2, 0, Color::GREEN };

    while !rl.window_should_close() &&
        !rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_W) &&
        !rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_A) &&
        !rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_S) &&
        !rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_D)
    {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(get_background_color(game_state));

        d.draw_text("Hello, world!", 10, 10, 20, Color::BLACK);
        d.draw_text("Press W, A, S, or D to continue...", 10, 30, 20, Color::BLACK);
    }
    
    while !rl.window_should_close()
    {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(get_background_color(game_state));
        
        d.draw_text("Thank you for playing! Press ESC to exit...", 10, 10, 20, Color::BLACK);
    }
}
