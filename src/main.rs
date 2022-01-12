/**
 * Chelsea Meyers
 *
 * rust_demo - a standalone application showcasing some basic 2D features
 * This demo uses a Rust binding of Raylib.
 *
 **/

extern crate raylib;

use raylib::prelude::*;

// GameState - a data structure used to keep track of state info
struct GameState {
    player_x: i32,
    player_y: i32,
    player_down_up: i32,
    player_right_left: i32,
    sword_state: i32,

    background_color: Color,
    player_color: Color,

    loop_phase: i32,

    respectful: bool,
}

// get_next_color - look up next color for switching player color using 'C'
fn get_next_color(player_color: Color) -> Color
{
    if player_color == Color::BLACK
    {
        return Color::RED;
    }
    else if player_color == Color::RED
    {
        return Color::BLUE;
    }
    else if player_color == Color::BLUE
    {
        return Color::WHITE;
    }

    return Color::BLACK;
}

// Application Entry Point
fn main() {
    let screen_width = 800;
    let screen_height = 600;
    let target_fps = 60;

    let player_speed = 2;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Rust Demo - Standalone app showcasing a Raylib binding for Rust - Chelsea")
        .build();

    rl.set_target_fps(target_fps);

    let mut game_state = GameState {
        player_x: screen_width / 2,
        player_y: screen_height / 2,
            player_down_up: 0,
        player_right_left: 1,
        sword_state: 0,
        background_color: Color::GREEN,
        player_color: Color::BLACK,
        loop_phase: 0,
        respectful: false
    };

    // Main Loop Phase 0
    while !rl.window_should_close() &&
        !rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_W) &&
        !rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_A) &&
        !rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_S) &&
        !rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_D)
    {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(game_state.background_color);

        d.draw_text("Hello, world!", 10, 10, 20, Color::BLACK);
        d.draw_text("Press W, A, S, or D to continue...", 10, 30, 20, Color::BLACK);
    }

    // Main Loop Phase 1
    game_state.loop_phase = 1;
    while !rl.window_should_close()
    {
        // Draw/sheath sword
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ENTER)
        {
            game_state.sword_state = 1 - game_state.sword_state;
        }

        // Change color
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_C)
        {
            game_state.player_color = get_next_color(game_state.player_color);
        }

        // Easter egg - pay respects
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_F)
        {
            game_state.respectful = !game_state.respectful;

            if game_state.respectful
            {
                game_state.background_color = Color::YELLOW;
            }
            else
            {
                game_state.background_color = Color::GREEN;
            }
        }

        // Move
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_W)
        {
            game_state.player_down_up = -2;
            game_state.player_right_left = 0;
            game_state.player_y += player_speed * game_state.player_down_up;
        }
        else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_A)
        {
            game_state.player_down_up = 0;
            game_state.player_right_left = -2;
            game_state.player_x += player_speed * game_state.player_right_left;
        }
        else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_S)
        {
            game_state.player_down_up = 2;
            game_state.player_right_left = 0;
            game_state.player_y += player_speed * game_state.player_down_up;
        }
        else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_D)
        {
            game_state.player_down_up = 0;
            game_state.player_right_left = 2;
            game_state.player_x += player_speed * game_state.player_right_left;
        }

        // Now, draw...

        let mut d = rl.begin_drawing(&thread);

        // Draw background
        d.clear_background(game_state.background_color);

        // Draw player
        d.draw_circle(game_state.player_x, game_state.player_y, 20.0, game_state.player_color);

        // Draw sword and/or sword info
        if game_state.sword_state == 0
        {
            d.draw_text("Press ENTER to toggle sword...", 10, 30, 20, Color::BLACK);
        }
        else if game_state.sword_state == 1
        {
            if game_state.player_down_up == -2
            {
                d.draw_text("Upward strike!", 10, 30, 20, Color::BLACK);
                d.draw_rectangle(game_state.player_x, game_state.player_y - 35, 5, 30, Color::LIGHTGRAY);
             }
            else if game_state.player_down_up == 2
            {
                d.draw_text("Downward strike!", 10, 30, 20, Color::BLACK);
                d.draw_rectangle(game_state.player_x, game_state.player_y + 5, 5, 30, Color::LIGHTGRAY);
            }
            else if game_state.player_right_left == -2
            {
                d.draw_text("Leftward strike!", 10, 30, 20, Color::BLACK);
                d.draw_rectangle(game_state.player_x - 35, game_state.player_y, 30, 5, Color::LIGHTGRAY);
            }
            else if game_state.player_right_left == 2
            {
                d.draw_text("Rightward strike!", 10, 30, 20, Color::BLACK);
                d.draw_rectangle(game_state.player_x + 5, game_state.player_y, 30, 5, Color::LIGHTGRAY);
            }
        }

        // Thank player for paying respects
        if game_state.respectful
        {
            d.draw_text("Thank you for paying your respects. Press F to toggle this mode...", 10, 50, 20, Color::BLACK);
        }

        d.draw_text("Thank you for playing! Press ESC to exit. Press C to change player color...", 10, 10, 20, Color::BLACK);
    }
}
