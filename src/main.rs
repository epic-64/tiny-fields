use macroquad::prelude::*;
use std::time::Instant;
use macroquad::math::{f32, f64};
use game::{Fonts, Textures};

mod draw;
pub mod game;

use crate::draw::{draw, UiElement};
use crate::game::{Assets, GameState, Intent, Job, UiRect};

#[macroquad::main("Tiny Fields")]
async fn main() {
    set_pc_assets_folder("assets");
    request_new_screen_size(1600.0, 900.0);

    let hut1: Texture2D = load_texture("hut1.png").await.expect("Couldn't load file");
    let hut2: Texture2D = load_texture("hut2.png").await.expect("Couldn't load file");
    let wood_1: Texture2D = load_texture("ChopChop_1_.png").await.expect("Couldn't load file");
    let wood_2: Texture2D = load_texture("ChopChop_2_.png").await.expect("Couldn't load file");
    let frame1: Texture2D = load_texture("frame2.png").await.expect("Couldn't load file");
    let textures = Textures { hut1, hut2, wood_1, wood_2, frame1 };

    let main_font = load_ttf_font("Menlo-Regular.ttf").await.expect("Couldn't load font");
    let fonts = Fonts { main: main_font };

    let assets = Assets { fonts, textures };
    let mut state = GameState::new(assets);

    let mut ui = Ui2 {
        last_mouse_position: Vec2::new(0.0, 0.0),
        global_offset: Vec2::new(0.0, 0.0),
    };

    loop {
        let frame_start = Instant::now();
        let dt = get_frame_time();

        // The UI can be moved around.
        ui.update_offset();

        // Build all the UI elements from the current game state
        let ui_elements = ui.get_ui_elements(&state);

        // Draw all the elements. Since we build them from the old
        // game state, this should happen before state.step()
        clear_background(ORANGE);

        ui_elements.iter().for_each(draw);

        // Collect all intentions from the UI
        let intents = get_intents(ui_elements);

        // Update game state
        state.step(&intents, dt);

        // Keep track of FPS
        state.game_meta.raw_fps = 1.0 / frame_start.elapsed().as_secs_f32();
        state.game_meta.effective_fps = get_fps() as f32;

        next_frame().await;
    }
}

struct Ui2 {
    last_mouse_position: Vec2,
    global_offset: Vec2,
}

impl Ui2 {
    pub fn update_offset(&mut self) {
        let mouse_wheel_delta = mouse_wheel().1; // Get the vertical scroll delta
        if mouse_wheel_delta.abs() > 0.0 {
            let new_offset = {self.global_offset + Vec2::new(0.0, mouse_wheel_delta * 30.0)}.clamp(
                Vec2::new(-200.0, -600.0),
                Vec2::new(1000.0, 600.0),
            );

            self.global_offset = new_offset;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            self.last_mouse_position = Vec2::from(mouse_position());
        }

        if is_mouse_button_down(MouseButton::Right) {
            let current_mouse_pos = Vec2::from(mouse_position());
            let delta = current_mouse_pos - self.last_mouse_position;

            if delta.length_squared() > 0.0 {
                let new_offset = {self.global_offset + delta}.clamp(
                    Vec2::new(-200.0, -600.0),
                    Vec2::new(1000.0, 600.0),
                );

                self.global_offset = new_offset;
            }

            self.last_mouse_position = current_mouse_pos;
        }
    }
}

impl Ui2 {
    pub fn get_ui_elements(&mut self, state: &GameState) -> Vec<UiElement> {
        let mut elements: Vec<UiElement> = vec![];

        let background_image = UiElement::Image {
            x: 0., // fixed
            y: 0., // fixed
            width: screen_width() as f64,
            height: screen_height() as f64,
            texture: state.assets.textures.hut1.clone(),
            color: WHITE,
        };

        let top_bar_draw_commands = vec![
            UiElement::Rectangle {
                x: 50.0, // fixed
                y: 50.0, // fixed
                width: screen_width() as f64,
                height: 50.0,
                color: DARKGRAY,
            },
            UiElement::Text {
                content: "Tiny Fields".to_string(),
                x: self.global_offset.x + 50.0 + 10.0,
                y: self.global_offset.y + 50.0 + 10.0,
                font_size: 30.0,
                color: WHITE,
            },
        ];

        elements.push(background_image);
        elements.extend(top_bar_draw_commands);
        elements.extend(self.get_all_job_elements(state, Vec2::new(50.0, 100.0)));
        elements.extend(self.get_all_job_elements(state, Vec2::new(650.0, 100.0)));

        elements
    }

    fn get_all_job_elements(&self, state: &GameState, relative_offset: Vec2) -> Vec<UiElement>
    {
        let mut elements: Vec<UiElement> = vec![];

        let mut container_offset = relative_offset;
        let card_height = 180.0;
        let card_width = 550.0;
        let card_spacing = 15.0;
        let card_padding_x = 55.0;
        let card_padding_y = 40.0;

        elements.push(UiElement::Scissor {
            clip: Some((
                container_offset.x as i32,
                container_offset.y as i32,
                card_width as i32,
                card_height as i32 * 4 + 15 * 3,
            )),
        });

        for (id, job) in state.jobs.iter().enumerate() {
            let job_draw_container = get_job_elements(
                &state.assets,
                job,
                id,
                self.global_offset + container_offset,
                card_height,
                card_padding_x,
                card_padding_y,
                card_spacing,
            );

            elements.extend(job_draw_container);

            container_offset += Vec2::new(0.0, card_height as f32 + 15.0);
        }

        elements.push(UiElement::Scissor { clip: None });

        elements
    }
}

pub fn get_job_elements(
    assets: &Assets,
    job: &Job,
    job_id: usize,
    offset: Vec2,
    card_height: f64,
    card_padding_x: f32,
    card_padding_y: f32,
    card_spacing: f32,
) -> Vec<UiElement>
{
    let color_primary = DARKBLUE;
    let color_secondary = DARKGRAY;
    let color_button = DARKGRAY;
    let color_button_hover = SKYBLUE;

    let font_size_large = 24.0;
    let font_size_small = 20.0;

    let card_width = 550.0;
    let image_width = 90.0f32;
    let inner_x = offset.x + card_padding_x + image_width + card_spacing;
    let progress_bar_width = card_width - card_padding_x - image_width - card_spacing - card_padding_x;
    let button_width = 80.0;

    let chosen_image = if job.running && job.time_accumulator % 2.0 < 1.0 {
        &assets.textures.wood_2
    } else {
        &assets.textures.wood_1
    };

    let elements = vec![
        // Background
        UiElement::Image {
            x: offset.x,
            y: offset.y,
            width: card_width as f64,
            height: card_height,
            texture: assets.textures.frame1.clone(),
            color: WHITE
        },

        // Job Animation
        UiElement::Image {
            x: offset.x + card_padding_x,
            y: offset.y + card_padding_y,
            width: image_width as f64,
            height: card_height - card_padding_y as f64 * 2.0,
            texture: chosen_image.clone(),
            color: if job.running { WHITE } else { Color::from_rgba(90, 90, 90, 255) },
        },

        // Title Bar
        UiElement::Text {
            content: job.name.clone() + " ",
            x: inner_x,
            y: offset.y + card_padding_y + 15.0,
            font_size: font_size_large,
            color: color_primary,
        },

        // Job Info
        UiElement::Text {
            content: format!("Lvl {} | ${} | {}s | {} Slots", job.level, job.money_per_action(), job.action_duration, job.timeslot_cost),
            x: inner_x,
            y: offset.y + 80.0,
            font_size: font_size_small,
            color: color_secondary,
        },

        // Action Progress Bar
        UiElement::ProgressBar {
            x: inner_x,
            y: offset.y + 96.0,
            width: progress_bar_width - 120.0,
            height: 20.0,
            progress: job.action_progress.get(),
            background_color: GRAY,
            foreground_color: GREEN,
        },

        // Action Progress Text
        UiElement::Text {
            content: format!("{:.1} / {:.1}", job.time_accumulator, job.action_duration),
            x: inner_x + 10.0,
            y: offset.y + 111.0,
            font_size: 20.0,
            color: WHITE,
        },

        // Level Up Progress Bar
        UiElement::ProgressBar {
            x: inner_x,
            y: offset.y + 126.0,
            width: progress_bar_width - 120.0,
            height: 20.0,
            progress: job.level_up_progress.get(),
            background_color: GRAY,
            foreground_color: BLUE,
        },

        // Level Up Progress Text
        UiElement::Text {
            content: format!("Level Up: {} / {}", job.actions_done, job.actions_to_level_up()),
            x: inner_x + 10.0,
            y: offset.y + 141.0,
            font_size: 20.0,
            color: WHITE,
        },

        // Start / Stop Button
        UiElement::Button {
            rectangle: UiRect {
                x: offset.x + card_width - button_width - card_padding_x,
                y: offset.y + card_padding_y,
                width: button_width,
                height: 46.0,
            },
            scissor: None, // todo: add scissor from parent container
            font_size: font_size_large,
            text: if job.running { "Stop".to_string() } else { "Start".to_string() },
            color: color_button,
            hover_color: color_button_hover,
            intent: Intent::ToggleJob(job_id),
        }
    ];

    elements
}

pub fn get_intents(elements: Vec<UiElement>) -> Vec<Intent> {
    let mut intents: Vec<Intent> = vec![];

    for element in elements {
        match element {
            UiElement::Button { rectangle, intent, scissor, .. } => {
                // First, check if the hovered position is within the clipping area.
                // (if there is no clipping area, we skip this check)
                if let Some(area) = scissor {
                    let (x, y, width, height) = area;
                    let scissor_rect = UiRect {
                        x: x as f32,
                        y: y as f32,
                        width: width as f32,
                        height: height as f32,
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