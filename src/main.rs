use game::{Fonts, Textures};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;

pub mod draw;
pub mod game;
pub mod job;
pub mod ui;

use crate::draw::{draw, UiElement};
use crate::game::{Assets, GameState, Intent, UiRect};
use crate::job::JobUi;

pub fn get_mouse_buttons(check: fn(MouseButton) -> bool) -> Vec<MouseButton> {
    vec![MouseButton::Left, MouseButton::Right, MouseButton::Middle]
        .into_iter()
        .filter(|&button| check(button))
        .collect()
}

pub struct MouseInput {
    pub pressed: Vec<MouseButton>,
    pub released: Vec<MouseButton>,
    pub down: Vec<MouseButton>,
    pub position: Vec2,
    pub scroll_y: f32,
}

#[macroquad::main("Tiny Fields")]
async fn main() {
    set_pc_assets_folder("assets");
    request_new_screen_size(1600.0, 900.0);

    let mut state = GameState::new();
    let assets: Assets = load_assets().await;

    let mut job_ui = JobUi::new(UiRect{ x: 50.0, y: 100.0, w: 500.0, h: 600.0 });
    let mut job_ui_2 = JobUi::new(UiRect{ x: 600.0, y: 100.0, w: 500.0, h: 600.0 });

    loop {
        let frame_start = now();
        let dt = get_frame_time();

        // collect inputs (IO)
        let _mouse_input = MouseInput {
            pressed: get_mouse_buttons(is_mouse_button_pressed),
            released: get_mouse_buttons(is_mouse_button_released),
            down: get_mouse_buttons(is_mouse_button_down),
            position: Vec2::from(mouse_position()),
            scroll_y: mouse_wheel().1,
        };

        // The UI can be moved around.
        job_ui.update();
        job_ui_2.update();

        // Build all the UI elements from the current game state
        let job_elements = job_ui.build(&state, &assets);
        let job_elements_2 = job_ui_2.build(&state, &assets);

        // Draw all the elements. Since we build them from the old
        // game state, this should happen before state.step()
        clear_background(ORANGE);

        job_elements.iter().for_each(draw);
        job_elements_2.iter().for_each(draw);

        // Collect all intentions from the UI
        let intents = get_intents(job_elements);
        let intents_2 = get_intents(job_elements_2);

        let merged_intents = intents.into_iter().chain(intents_2).collect::<Vec<_>>();

        // Update game state
        state.step(&merged_intents, dt);

        // Keep track of FPS
        let elapsed = now() - frame_start;
        state.game_meta.raw_fps = 1.0 / elapsed as f32;
        state.game_meta.effective_fps = get_fps() as f32;

        next_frame().await;
    }
}

async fn load_assets() -> Assets {
    let hut1: Texture2D = load_texture("hut1.png").await.expect("Couldn't load file");
    let hut2: Texture2D = load_texture("hut2.png").await.expect("Couldn't load file");
    let wood_1: Texture2D = load_texture("ChopChop_1_.png").await.expect("Couldn't load file");
    let wood_2: Texture2D = load_texture("ChopChop_2_.png").await.expect("Couldn't load file");
    let frame1: Texture2D = load_texture("frame2.png").await.expect("Couldn't load file");
    let coin: Texture2D = load_texture("coin.png").await.expect("Couldn't load file");
    let affection: Texture2D = load_texture("rune_heart.png").await.expect("Couldn't load file");
    let time: Texture2D = load_texture("rune_time.png").await.expect("Couldn't load file");
    let textures = Textures { hut1, hut2, wood_1, wood_2, frame1, coin, affection, time };

    let main_font = load_ttf_font("Menlo-Regular.ttf").await.expect("Couldn't load font");
    let fonts = Fonts { main: Some(main_font) };

    Assets { fonts, textures }
}

pub fn get_intents(elements: Vec<UiElement>) -> Vec<Intent> {
    let mut intents: Vec<Intent> = vec![];

    for element in elements {
        match element {
            UiElement::Button { rectangle, intent, parent_clip: scissor, .. } => {
                // First, check if the hovered position is within the clipping area.
                // (if there is no clipping area, we skip this check)
                if let Some(area) = scissor {
                    let (x, y, width, height) = area;
                    let scissor_rect = UiRect {
                        x: x as f32,
                        y: y as f32,
                        w: width as f32,
                        h: height as f32,
                    };

                    if !scissor_rect.is_hovered() {
                        continue; // if mouse is outside the clipping area, skip intents
                    }
                }

                if rectangle.is_clicked() {
                    intents.push(intent);
                }
            }
            _ => {}
        }
    }

    intents
}