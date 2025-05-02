use std::time::Instant;
use macroquad::prelude::*;

mod my_lib;
mod layout;
mod draw;
mod render;
pub mod game;

use draw::{draw};
use game::{GameState, UserInterface, Assets};

#[macroquad::main("Tiny Fields")]
async fn main() {
    set_pc_assets_folder("assets");
    request_new_screen_size(1600.0, 900.0);

    let wood_1: Texture2D = load_texture("WoodChop_1.png").await.expect("Couldn't load file");
    let wood_2: Texture2D = load_texture("WoodChop_2.png").await.expect("Couldn't load file");
    wood_1.set_filter(FilterMode::Nearest);

    let assets = Assets {
        wood_cutting_image_1: wood_1,
        wood_cutting_image_2: wood_2,
    };

    let mut state = GameState::new();
    let mut ui = UserInterface::new(&state);

    loop {
        let frame_start = Instant::now();
        let dt = get_frame_time();

        clear_background(ORANGE);
        let intents = ui.process_input();

        if is_mouse_button_down(MouseButton::Right) {
            let current_mouse_pos = Vec2::from(mouse_position());
            let delta = current_mouse_pos - ui.last_mouse_position;

            if delta.length_squared() > 0.0 {
                let new_offset = {ui.global_offset + delta}.clamp(
                    Vec2::new(-200.0, -600.0),
                    Vec2::new(1000.0, 600.0),
                );

                ui = ui.recreate(&state, new_offset);
            }

            ui.last_mouse_position = current_mouse_pos;
        }

        // Update game state
        state.step(&intents, dt);

        // Compile list of draw commands
        let commands = ui.render(&state, &assets);

        // Draw the game
        draw(&commands);

        // Keep track of FPS
        state.game_meta.raw_fps = 1.0 / frame_start.elapsed().as_secs_f32();
        state.game_meta.effective_fps = get_fps() as f32;

        next_frame().await;
    }
}