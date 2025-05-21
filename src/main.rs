use game::{Fonts, Textures};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;

pub mod draw;
pub mod game;
pub mod job;
pub mod ui;

use crate::draw::{draw, UiElement};
use crate::game::{Assets, Effect, EffectWithSource, GameState, Intent, JobType, MouseInput, Palette, TextParticle, UiRect};
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

    state.add_job_instance(JobType::Woodcutting);
    state.add_job_instance(JobType::Woodcutting);
    state.add_job_instance(JobType::Woodcutting);

    let assets: Assets = load_assets().await;

    let mut job_ui = JobUi::new(UiRect{ x: 25.0, y: 100.0, w: 500.0, h: 600.0 });

    // Example for using quad-storage
    // let storage = &mut quad_storage::STORAGE.lock().unwrap();
    // storage.set("test", &format!("{}", now()));
    // let value = storage.get("test").unwrap();

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

        // build all ui elements (draw commands)
        let job_elements = job_ui.build(&state, &assets);

        let top_hud_elements = get_top_hud(&state, &assets, UiRect { x: 25.0, y: 25.0, w: screen_width(), h: 50.0 });
        let inventory_elements = build_inventory_elements(&state, &assets, UiRect { x: 600.0, y: 15.0, w: 200.0, h: 80.0 });

        let cheat_buttons = get_cheat_buttons(&assets, UiRect { x: 50.0, y: 790.0, w: 200.0, h: 40.0 });
        let debug_elements = build_debug_elements(&state, &assets, UiRect { x: 50.0, y: 850.0, w: 200.0, h: 40.0 });

        // collect all intents from UI interactions
        let mut all_intents: Vec<Intent> = vec![];
        all_intents.extend(get_intents(&job_elements, &mouse_input));
        all_intents.extend(get_intents(&top_hud_elements, &mouse_input));
        all_intents.extend(get_intents(&cheat_buttons, &mouse_input));
        all_intents.extend(get_intents(&inventory_elements, &mouse_input));

        // Update game state
        let effects = state.step(&all_intents, dt);

        // trigger new text particles
        for effect in &effects {
            match effect {
                EffectWithSource::Job { job, effect } => {
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
                font: assets.fonts.main.clone(),
                x: particle.position.x,
                y: particle.position.y,
                font_size: 12.0,
                color: particle.color,
            }
        }).collect();

        // Draw everything
        clear_background(Palette::Anthracite.get_color());
        top_hud_elements.iter().for_each(|el|draw(el, &mouse_input));
        job_elements.iter().for_each(|el|draw(el, &mouse_input));
        inventory_elements.iter().for_each(|el|draw(el, &mouse_input));
        cheat_buttons.iter().for_each(|el|draw(el, &mouse_input));
        debug_elements.iter().for_each(|el|draw(el, &mouse_input));
        effects_elements.iter().for_each(|el|draw(el, &mouse_input));

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

    // FPS Text
    elements.push(UiElement::Text {
        content: format!("FPS: {:.2}", state.game_meta.effective_fps),
        font: assets.fonts.main.clone(),
        x: rect.x,
        y: rect.y,
        font_size,
        color: WHITE,
    });

    // Add Frame Time and Raw FPS Text
    elements.push(UiElement::Text {
        content: format!(
            "Frame Time: {:.2} ms | Raw FPS: {:.2}",
            state.game_meta.frame_time * 1000.0,
            state.game_meta.raw_fps
        ),
        font: assets.fonts.main.clone(),
        x: rect.x,
        y: rect.y + font_size,
        font_size,
        color: WHITE,
    });

    elements
}

async fn load_assets() -> Assets {
    let textures = Textures {
        wood_1: load_texture("ChopChop_1_.png").await.expect("Couldn't load file"),
        wood_2: load_texture("ChopChop_2_.png").await.expect("Couldn't load file"),
        mining_1: load_texture("ClingCling_1.png").await.expect("Couldn't load file"),
        mining_2: load_texture("ClingCling_2.png").await.expect("Couldn't load file"),
        hunting_1: load_texture("PewPew_1.png").await.expect("Couldn't load file"),
        hunting_2: load_texture("PewPew_2.png").await.expect("Couldn't load file"),
        smithing_1: load_texture("BomBom_1.png").await.expect("Couldn't load file"),
        smithing_2: load_texture("BomBom_2.png").await.expect("Couldn't load file"),
    };

    let fonts = Fonts {
        main: Some(load_ttf_font("Menlo-Regular.ttf").await.expect("Couldn't load font"))
    };

    Assets { fonts, textures }
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
            color: Palette::DarkGray.get_color(),
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
            color: Palette::Grass.get_color(),
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
        font: assets.fonts.main.clone(),
        intent: Intent::BuyTimeSlot,
        text: format!("Buy ({})", state.time_slots.get_upgrade_cost()),
        font_size: 14.0,
        color: Palette::DarkGray.get_color(),
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
        font: assets.fonts.main.clone(),
        intent: Intent::SkipSeconds(300),
        text: "Skip 5 min".to_string(),
        font_size: 14.0,
        color: Palette::DarkGray.get_color(),
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
        font: assets.fonts.main.clone(),
        intent: Intent::SkipSeconds(604_800),
        text: "Skip 1 week".to_string(),
        font_size: 14.0,
        color: Palette::DarkGray.get_color(),
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
        color: Palette::Mocha.get_color(),
        bordered: true,
    });

    let inventory = &state.inventory;
    let item_size = 40.0;

    let items = inventory.items.clone();

    for (index, (item_name, item_count)) in items.iter().enumerate() {
        elements.push(UiElement::Rectangle {
            x: rect.x + index as f32 * (item_size + 5.0),
            y: rect.y,
            width: item_size,
            height: item_size,
            color: Palette::DarkGray.get_color(),
            bordered: true,
        });

        elements.push(UiElement::Text {
            content: format!("{}", item_name.to_string()),
            font: assets.fonts.main.clone(),
            x: rect.x + index as f32 * (item_size + 5.0),
            y: rect.y + item_size / 2.0,
            font_size: 14.0,
            color: WHITE,
        });

        elements.push(UiElement::Text {
            content: format!("{}", item_count),
            font: assets.fonts.main.clone(),
            x: rect.x + index as f32 * (item_size + 5.0),
            y: rect.y + item_size,
            font_size: 14.0,
            color: WHITE,
        });
    }

    elements
}