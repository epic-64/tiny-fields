use crate::assets::{load_assets, Assets};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;

pub mod draw;
pub mod game;
pub mod job;
pub mod ui;
pub mod palette;
pub mod assets;

use crate::draw::{draw, UiElement};
use crate::game::{Effect, EffectWithSource, GameState, Intent, JobType, MouseInput, TextParticle, UiRect};
use crate::job::build_job_cards;

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

    state.add_job_instance(JobType::Woodcutting);
    state.add_job_instance(JobType::Woodcutting);
    state.add_job_instance(JobType::Woodcutting);
    state.add_job_instance(JobType::Woodcutting);
    state.add_job_instance(JobType::Cooking);
    state.add_job_instance(JobType::Hunting);
    state.add_job_instance(JobType::Hunting);
    state.add_job_instance(JobType::Hunting);
    state.add_job_instance(JobType::Hunting);

    let assets: Assets = load_assets().await;

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

        // collect inputs (IO)
        let mouse_input = MouseInput {
            pressed: get_mouse_buttons(is_mouse_button_pressed),
            released: get_mouse_buttons(is_mouse_button_released),
            down: get_mouse_buttons(is_mouse_button_down),
            position: mouse_position(),
            scroll_y: mouse_wheel().1,
        };

        // build all ui elements (draw commands)
        let job_elements = build_job_cards(&state, &assets, Vec2::new(25.0, 100.0) + resolution_offset);
        let debug_elements = build_debug_elements(&state, &assets, UiRect { x: 700.0, y: 25.0, w: 200.0, h: 40.0 });

        // collect all intents from UI interactions
        let mut all_intents: Vec<Intent> = vec![];
        all_intents.extend(get_intents(&job_elements, &mouse_input));

        // Update game state
        let effects = state.step(&all_intents, dt);

        // trigger new text particles
        for effect in &effects {
            match effect {
                EffectWithSource::JobSource { job, effect } => {
                    match effect {
                        Effect::AddItem { item, amount } => {
                            state.text_particles.push(TextParticle {
                                text: format!("{} +{}", item.to_string(), amount),
                                position: Vec2::from(job.get_particle_marker(&job_elements)),
                                velocity: Vec2::new(0.0, -15.0),
                                color: SKYBLUE,
                                lifetime: 1.5
                            });
                        }
                    }
                }
            }
        }

        // remove expired text particles
        state.text_particles.retain(|particle| { particle.is_alive() });

        // step through all text particles
        state.text_particles.iter_mut().for_each(|particle| particle.step(dt));

        // Build UI elements for text particles
        let effects_elements: Vec<UiElement> = state.text_particles.iter().map(|particle| {
            UiElement::Text {
                content: particle.text.clone(),
                font: assets.fonts.mono.clone(),
                x: particle.position.x,
                y: particle.position.y,
                font_size: 12.0,
                color: particle.color,
            }
        }).collect();

        // Draw everything
        clear_background(palette::GAME_BACKGROUND.get_color());
        job_elements.iter().for_each(|el|draw(el, &mouse_input));
        debug_elements.iter().for_each(|el|draw(el, &mouse_input));

        // Keep track of FPS
        let elapsed = now() - frame_start;
        state.game_meta.frame_time = elapsed;
        state.game_meta.raw_fps = 1.0 / elapsed;
        state.game_meta.effective_fps = get_fps() as f64;

        next_frame().await;
    }
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

pub fn get_top_hud(state: &GameState, assets: &Assets, rect: UiRect) -> Vec<UiElement> {
    let mut elements = vec![];

    let icon_size = 60.0;

    // Draw Rectangle for each Time Slot (used and free)
    for i in 0..state.time_slots.total {
        elements.push(UiElement::Rectangle {
            x: rect.x + i as f32 * (icon_size + 5.0),
            y: rect.y,
            width: icon_size,
            height: icon_size,
            color: palette::BUTTON_BACKGROUND.get_color(),
            bordered: false
        });
    }

    // Draw a smaller green rectangle for each used time slot
    for i in 0..state.time_slots.used {
        elements.push(UiElement::Rectangle {
            x: rect.x + i as f32 * (icon_size + 5.0) + 5.0,
            y: rect.y + 5.0,
            width: icon_size - 10.0,
            height: icon_size - 10.0,
            color: palette::PROGRESS_COLOR.get_color(),
            bordered: false
        });
    }

    // Button for buying time slots
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: rect.x + state.time_slots.total as f32 * (icon_size + 5.0) + 5.0,
            y: rect.y,
            w: 120.0,
            h: icon_size,
        },
        font: assets.fonts.mono.clone(),
        intent: Intent::BuyTimeSlot,
        text: format!("Buy ({})", state.time_slots.get_upgrade_cost()),
        font_size: 14.0,
        background_color: palette::BUTTON_BACKGROUND.get_color(),
        text_color: palette::BUTTON_TEXT.get_color(),
        parent_clip: None,
    });

    elements
}

pub fn get_cheat_buttons(assets: &Assets, rect: UiRect) -> Vec<UiElement> {
    let mut elements = vec![];

    // Button for skipping 5 minutes
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: rect.x,
            y: rect.y,
            w: 200.0,
            h: 40.0,
        },
        font: assets.fonts.mono.clone(),
        intent: Intent::SkipSeconds(300),
        text: "Skip 5 min".to_string(),
        font_size: 14.0,
        background_color: palette::BUTTON_BACKGROUND.get_color(),
        text_color: palette::BUTTON_TEXT.get_color(),
        parent_clip: None,
    });

    // Button for skipping 1 week
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: rect.x + 210.0,
            y: rect.y,
            w: 200.0,
            h: 40.0,
        },
        font: assets.fonts.mono.clone(),
        intent: Intent::SkipSeconds(604_800),
        text: "Skip 1 week".to_string(),
        font_size: 14.0,
        background_color: palette::BUTTON_BACKGROUND.get_color(),
        text_color: palette::BUTTON_TEXT.get_color(),
        parent_clip: None,
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
        bordered: true,
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
            bordered: true,
        });

        elements.push(UiElement::Text {
            content: format!("{}", item_name.to_string()),
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