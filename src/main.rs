use crate::assets::{load_assets, Assets};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;

pub mod draw;
pub mod game;
pub mod job;
pub mod ui;
pub mod palette;
pub mod assets;
pub mod skill;
pub mod counts_actions;
pub mod job_slot;

use crate::draw::{draw, BorderStyle, UiElement};
use crate::game::{GameState, Intent, MouseInput, UiRect};
use crate::job_slot::JobSlotState;

pub fn get_mouse_buttons(check: fn(MouseButton) -> bool) -> Vec<MouseButton> {
    vec![MouseButton::Left, MouseButton::Right, MouseButton::Middle]
        .into_iter()
        .filter(|&button| check(button))
        .collect()
}

#[macroquad::main("Tiny Fields")]
async fn main() {
    set_pc_assets_folder("assets");
    request_new_screen_size(1280., 720.0);
    set_default_filter_mode(FilterMode::Linear);

    let mut state = GameState::new();
    let mut is_fullscreen = false;
    let mut show_debug = false;

    let assets: Assets = load_assets().await;

    for i in 0..state.job_slots.len() {
        state.job_slots[i].state = JobSlotState::Locked;
    }

    // Example for using quad-storage
    // let storage = &mut quad_storage::STORAGE.lock().unwrap();
    // storage.set("test", &format!("{}", now()));
    // let value = storage.get("test").unwrap();

    loop {
        let frame_start = now();
        let dt = get_frame_time();

        let resolution_offset_x = (screen_width() - 1280.0) / 2.0;
        let resolution_offset_y = (screen_height() - 720.0) / 2.0;
        let resolution_offset = Vec2::new(resolution_offset_x, resolution_offset_y);

        // toggle fullscreen on F11
        if is_key_pressed(KeyCode::F11) {
            is_fullscreen = !is_fullscreen;
            set_fullscreen(is_fullscreen);
            if !is_fullscreen {
                request_new_screen_size(1280.0, 720.0);
            }
        }

        // toggle debug mode on F9
        if is_key_pressed(KeyCode::F9) {
            show_debug = !show_debug;
        }

        // collect inputs (IO)
        let mouse_input = MouseInput {
            pressed: get_mouse_buttons(is_mouse_button_pressed),
            released: get_mouse_buttons(is_mouse_button_released),
            down: get_mouse_buttons(is_mouse_button_down),
            position: mouse_position(),
            scroll_y: mouse_wheel().1,
        };

        let all_ui_elements = build_ui_elements(&state, &assets, resolution_offset, show_debug);
        let all_intents: Vec<Intent> = get_intents(&all_ui_elements, &mouse_input);
        let _effects = state.step(&all_intents, dt);

        clear_background(palette::GAME_BACKGROUND.get_color());
        all_ui_elements.iter().for_each(|el| draw(el, &mouse_input));

        // Keep track of FPS
        let elapsed = now() - frame_start;
        state.game_meta.frame_time = elapsed;
        state.game_meta.raw_fps = 1.0 / elapsed;
        state.game_meta.effective_fps = get_fps() as f64;

        next_frame().await;
    }
}

fn build_ui_elements(state: &GameState, assets: &Assets, resolution_offset: Vec2, show_debug: bool) -> Vec<UiElement> {
    let mut all_elements: Vec<UiElement> = vec![];

    all_elements.extend(state.get_job_slot_ui(&state, &assets, Vec2::new(25.0, 100.0) + resolution_offset));

    if show_debug {
        all_elements.extend(build_debug_elements(&state, &assets, UiRect::new(700.0, 25.0, 200.0, 40.0)));
        all_elements.extend(get_cheat_buttons(&assets, UiRect::new(25.0, 25.0, 400.0, 40.0)));
    }

    all_elements
}

fn build_debug_elements(state: &GameState, assets: &Assets, rect: UiRect) -> Vec<UiElement> {
    let mut elements = vec![];
    let font_size = 20.0;

    // Add Frame Time and Raw FPS Text
    elements.push(UiElement::Text {
        content: format!(
            "Frame Time: {:.2} ms | Raw FPS: {:.2}",
            state.game_meta.frame_time * 1000.0,
            state.game_meta.raw_fps
        ),
        font: assets.fonts.mono.clone(),
        x: rect.x,
        y: rect.y,
        font_size,
        color: WHITE,
    });

    elements
}

pub fn get_intents(elements: &Vec<UiElement>, mouse_input: &MouseInput) -> Vec<Intent> {
    let mut intents: Vec<Intent> = vec![];

    for element in elements {
        match element {
            UiElement::RectButton { rectangle, intent, parent_clip, .. } |
            UiElement::ImgButton { rectangle, intent, parent_clip, ..} => {
                // First, check if the hovered position is within the clipping area.
                // (if there is no clipping area, we skip this check)
                if let Some(area) = *parent_clip {
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
                    intents.push(intent.clone());
                }
            }
            _ => {}
        }
    }

    intents
}

pub fn get_cheat_buttons(assets: &Assets, rect: UiRect) -> Vec<UiElement> {
    let mut elements = vec![];


    let button_width = 120.0;
    let button_height = 40.0;
    let button_spacing = 10.0;
    let font_size = 14.0;

    // Button for skipping 5 minutes
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: rect.x,
            y: rect.y,
            w: button_width,
            h: button_height,
        },
        font: assets.fonts.mono.clone(),
        intent: Intent::SkipSeconds(300),
        text: "Skip 5 min".to_string(),
        font_size: font_size,
        background_color: palette::BUTTON_BACKGROUND.get_color(),
        text_color: palette::BUTTON_TEXT.get_color(),
        parent_clip: None,
        border_style: BorderStyle::Solid,
    });

    // Button for skipping 1 week
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: rect.x + button_width + button_spacing,
            y: rect.y,
            w: button_width,
            h: button_height,
        },
        font: assets.fonts.mono.clone(),
        intent: Intent::SkipSeconds(604_800),
        text: "Skip 1 week".to_string(),
        font_size: font_size,
        background_color: palette::BUTTON_BACKGROUND.get_color(),
        text_color: palette::BUTTON_TEXT.get_color(),
        parent_clip: None,
        border_style: BorderStyle::Solid,
    });

    // Button for skipping 1 month
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: rect.x + 2.0 * (button_width + button_spacing),
            y: rect.y,
            w: button_width,
            h: button_height,
        },
        font: assets.fonts.mono.clone(),
        intent: Intent::SkipSeconds(60 * 60 * 24 * 30), // 1 month in seconds
        text: "Skip 1 month".to_string(),
        font_size: font_size,
        background_color: palette::BUTTON_BACKGROUND.get_color(),
        text_color: palette::BUTTON_TEXT.get_color(),
        parent_clip: None,
        border_style: BorderStyle::Solid,
    });

    elements
}

pub fn build_inventory_elements(state: &GameState, assets: &Assets, rect: UiRect) -> Vec<UiElement> {
    let mut elements = vec![];

    // Inventory Rect
    elements.push(UiElement::Rectangle {
        x: rect.x,
        y: rect.y,
        width: rect.w,
        height: rect.h,
        color: palette::CARD_BACKGROUND.get_color(),
        border_style: BorderStyle::Solid,
    });

    let inventory = &state.inventory;
    let item_size = 40.0;

    let items = inventory.item_amounts.clone();

    for (index, (item_name, item_count)) in items.iter().enumerate() {
        elements.push(UiElement::Rectangle {
            x: rect.x + index as f32 * (item_size + 5.0),
            y: rect.y,
            width: item_size,
            height: item_size,
            color: palette::IMAGE_BACKGROUND.get_color(),
            border_style: BorderStyle::Solid,
        });

        elements.push(UiElement::Text {
            content: format!("{}", item_name.get_name()),
            font: assets.fonts.mono.clone(),
            x: rect.x + index as f32 * (item_size + 5.0),
            y: rect.y + item_size / 2.0,
            font_size: 14.0,
            color: WHITE,
        });

        elements.push(UiElement::Text {
            content: format!("{}", item_count),
            font: assets.fonts.mono.clone(),
            x: rect.x + index as f32 * (item_size + 5.0),
            y: rect.y + item_size,
            font_size: 14.0,
            color: WHITE,
        });
    }

    elements
}