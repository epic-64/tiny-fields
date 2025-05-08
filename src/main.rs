use game::{Fonts, Textures};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;

pub mod draw;
pub mod game;
pub mod job;
pub mod ui;

use crate::draw::{draw, UiElement};
use crate::game::{Assets, GameState, Intent, MouseInput, UiRect};
use crate::job::JobUi;

pub fn get_mouse_buttons(check: fn(MouseButton) -> bool) -> Vec<MouseButton> {
    vec![MouseButton::Left, MouseButton::Right, MouseButton::Middle]
        .into_iter()
        .filter(|&button| check(button))
        .collect()
}

#[macroquad::main("Tiny Fields")]
async fn main() {
    set_pc_assets_folder("assets");
    request_new_screen_size(1600.0, 900.0);

    let mut state = GameState::new();
    let assets: Assets = load_assets().await;

    let mut job_ui = JobUi::new(UiRect{ x: 50.0, y: 160.0, w: 500.0, h: 600.0 });
    let mut job_ui_2 = JobUi::new(UiRect{ x: 600.0, y: 160.0, w: 500.0, h: 600.0 });

    loop {
        let frame_start = now();
        let dt = get_frame_time();

        // collect inputs (IO)
        let mouse_input = MouseInput {
            pressed: get_mouse_buttons(is_mouse_button_pressed),
            released: get_mouse_buttons(is_mouse_button_released),
            down: get_mouse_buttons(is_mouse_button_down),
            position: mouse_position(),
            scroll_y: mouse_wheel().1,
        };

        // Some UIs can be moved around based on inputs
        job_ui.update(&mouse_input);
        job_ui_2.update(&mouse_input);

        // build all ui elements (draw commands)
        let job_elements = job_ui.build(&state, &assets, &mouse_input);
        let job_elements_2 = job_ui_2.build(&state, &assets, &mouse_input);
        let top_hud_elements = get_top_hud(&state, &assets, UiRect { x: 50.0, y: 15.0, w: screen_width(), h: 50.0 });

        // Draw all the elements. Since we build them from the
        // old game state, this should happen before state.step()
        clear_background(ORANGE);
        job_elements.iter().for_each(draw);
        job_elements_2.iter().for_each(draw);
        top_hud_elements.iter().for_each(draw);

        // collect all intents from UI interactions
        let mut all_intents: Vec<Intent> = vec![];
        all_intents.extend(get_intents(job_elements, &mouse_input));
        all_intents.extend(get_intents(job_elements_2, &mouse_input));
        all_intents.extend(get_intents(top_hud_elements, &mouse_input));

        // Update game state
        state.step(&all_intents, dt);

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
    let time: Texture2D = load_texture("rune_time2.png").await.expect("Couldn't load file");

    let main_font = load_ttf_font("Menlo-Regular.ttf").await.expect("Couldn't load font");

    let textures = Textures { hut1, hut2, wood_1, wood_2, frame1, coin, affection, time };
    let fonts = Fonts { main: Some(main_font) };

    Assets { fonts, textures }
}

pub fn get_intents(elements: Vec<UiElement>, mouse_input: &MouseInput) -> Vec<Intent> {
    let mut intents: Vec<Intent> = vec![];

    for element in elements {
        match element {
            UiElement::Button { rectangle, intent, parent_clip: scissor, .. } => {
                // First, check if the hovered position is within the clipping area.
                // (if there is no clipping area, we skip this check)
                if let Some(area) = scissor {
                    let (x, y, w, h) = area;
                    let scissor_rect = UiRect {
                        x: x as f32,
                        y: y as f32,
                        w: w as f32,
                        h: h as f32,
                    };

                    if !scissor_rect.is_hovered(mouse_input) {
                        continue; // if the mouse is outside the clipping area, skip intents
                    }
                }

                if rectangle.is_clicked(mouse_input) {
                    intents.push(intent);
                }
            }
            _ => {}
        }
    }

    intents
}

pub fn get_top_hud(state: &GameState, assets: &Assets, rect: UiRect) -> Vec<UiElement> {
    let mut elements = vec![];

    let icon_size = 60.0;
    let font_size = 30.0;

    // Money Image
    elements.push(UiElement::Image {
        x: rect.x,
        y: rect.y,
        width: icon_size,
        height: icon_size,
        texture: assets.textures.coin.clone(),
        color: WHITE,
    });

    // Money Text
    elements.push(UiElement::Text {
        content: state.total_money.to_string(),
        font: assets.fonts.main.clone(),
        x: rect.x + 60.0,
        y: rect.y + font_size,
        font_size,
        color: WHITE,
    });

    // Free Time Slots
    for i in 0..state.time_slots.get_free() {
        elements.push(UiElement::Image {
            x: rect.x + i as f32 * (icon_size + 5.0),
            y: rect.y + icon_size + 5.0,
            width: icon_size,
            height: icon_size,
            texture: assets.textures.time.clone(),
            color: WHITE,
        });
    }

    // Used Time Slots
    for i in 0..state.time_slots.used {
        elements.push(UiElement::Image {
            x: rect.x + (state.time_slots.get_free() + i) as f32 * (icon_size + 5.0),
            y: rect.y + icon_size + 5.0,
            width: icon_size,
            height: icon_size,
            texture: assets.textures.time.clone(),
            color: GRAY,
        });
    }

    // Button for buying time slots
    elements.push(UiElement::Button {
        rectangle: UiRect {
            x: rect.x + 200.0,
            y: rect.y,
            w: 200.0,
            h: icon_size,
        },
        intent: Intent::BuyTimeSlot,
        text: format!("Buy ({})", state.time_slots.get_upgrade_cost()),
        font_size: 20.0,
        color: DARKGRAY,
        parent_clip: None,
        is_hovered: false,
    });

    // Button for skipping 5 minutes
    elements.push(UiElement::Button {
        rectangle: UiRect {
            x: rect.x + 200.0 + 220.0,
            y: rect.y,
            w: 200.0,
            h: icon_size,
        },
        intent: Intent::SkipSeconds(300),
        text: "Skip 5 min".to_string(),
        font_size: 20.0,
        color: DARKGRAY,
        parent_clip: None,
        is_hovered: false,
    });

    // Button for skipping 1 year
    elements.push(UiElement::Button {
        rectangle: UiRect {
            x: rect.x + 200.0 + 220.0 + 220.0,
            y: rect.y,
            w: 200.0,
            h: icon_size,
        },
        intent: Intent::SkipSeconds(31_536_000),
        text: "Skip 1 year".to_string(),
        font_size: 20.0,
        color: DARKGRAY,
        parent_clip: None,
        is_hovered: false,
    });


    elements
}