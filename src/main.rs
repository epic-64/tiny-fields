use game::{Fonts, Textures};
use macroquad::math::{f32, f64};
use macroquad::prelude::*;
use std::time::Instant;

mod draw;
pub mod game;

use crate::draw::{draw, UiElement};
use crate::game::{Assets, GameState, Intent, Job, UiRect};

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

    let mut job_ui = ScrollContainer::new(UiRect { x: 50.0, y: 50.0, w: 500.0, h: 600.0 });
    let mut job_ui_2 = ScrollContainer::new(UiRect { x: 600.0, y: 50.0, w: 500.0, h: 600.0 });

    loop {
        let frame_start = Instant::now();
        let dt = get_frame_time();

        // collect inputs (IO)
        let mouse_input = MouseInput {
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
        let job_elements = job_ui.build(&state, &assets, get_all_job_elements);
        let job_elements_2 = job_ui_2.build(&state, &assets, get_all_job_elements);

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
        state.game_meta.raw_fps = 1.0 / frame_start.elapsed().as_secs_f32();
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
    let textures = Textures { hut1, hut2, wood_1, wood_2, frame1 };

    let main_font = load_ttf_font("Menlo-Regular.ttf").await.expect("Couldn't load font");
    let fonts = Fonts { main: Some(main_font) };

    Assets { fonts, textures }
}

fn get_all_job_elements(state: &GameState, assets: &Assets, clip_rect: &UiRect, offset: Vec2) -> Vec<UiElement>
{
    let mut elements: Vec<UiElement> = vec![];

    let mut container_offset = offset;
    let card_height = 180.0;
    let card_width = clip_rect.w;
    let card_spacing = 15.0;
    let card_padding_x = 55.0;
    let card_padding_y = 40.0;

    let container_clip = Some((
        clip_rect.x as i32,
        clip_rect.y as i32,
        clip_rect.w as i32,
        clip_rect.h as i32,
    ));

    for (id, job) in state.jobs.iter().enumerate() {
        let job_draw_container = build_job_card(
            &container_clip,
            assets,
            job,
            id,
            container_offset,
            card_height as f64,
            card_width as f64,
            card_padding_x,
            card_padding_y,
            card_spacing,
        );

        elements.extend(job_draw_container);

        container_offset += Vec2::new(0.0, card_height as f32 + 15.0);
    }

    elements
}

pub fn build_job_card(
    clip: &Option<(i32, i32, i32, i32)>,
    assets: &Assets,
    job: &Job,
    job_id: usize,
    offset: Vec2,
    card_height: f64,
    card_width: f64,
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

    let image_width = 90.0f32;
    let inner_x = offset.x + card_padding_x + image_width + card_spacing;
    let progress_bar_width = card_width as f32 - card_padding_x - image_width - card_spacing - card_padding_x;
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
                x: offset.x + card_width as f32 - button_width - card_padding_x,
                y: offset.y + card_padding_y,
                w: button_width,
                h: 46.0,
            },
            parent_clip: clip.clone(),
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

pub struct ScrollContainer {
    rect: UiRect,
    scroll_offset: Vec2,
    last_mouse_position: Vec2,
}

impl ScrollContainer {
    pub fn new(rect: UiRect) -> Self {
        Self {
            rect,
            scroll_offset: Vec2::new(0.0, 0.0),
            last_mouse_position: Vec2::new(0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        if self.rect.is_hovered() {
            self.handle_scroll()
        }
    }

    fn handle_scroll(&mut self) {
        let mouse_wheel_delta = clamp(mouse_wheel().1, -1.0, 1.0);

        if mouse_wheel_delta.abs() > 0.0 {
            self.scroll_offset.y += mouse_wheel_delta * 40.0;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            self.last_mouse_position = Vec2::from(mouse_position());
        }

        if is_mouse_button_down(MouseButton::Right) {
            let current_mouse_pos = Vec2::from(mouse_position());
            let delta = current_mouse_pos - self.last_mouse_position;

            if delta.length_squared() > 0.0 {
                let new_offset = self.scroll_offset + Vec2::new(0.0, delta.y);

                self.scroll_offset = new_offset;
            }

            self.last_mouse_position = current_mouse_pos;
        }
    }

    pub fn build(
        &self,
        state: &GameState,
        assets: &Assets,
        build_ui_elements: fn(&GameState, &Assets, &UiRect, Vec2) -> Vec<UiElement>
    ) -> Vec<UiElement>
    {
        let mut elements: Vec<UiElement> = vec![];

        // Create a clipping area for the scroll container
        let clip = Some((
            self.rect.x as i32,
            self.rect.y as i32,
            self.rect.w as i32,
            self.rect.h as i32,
        ));

        // Create a scissor rectangle for the clipping area
        elements.push(UiElement::Scissor { clip });

        let scrollable_pos = Vec2::new(self.rect.x, self.rect.y) + self.scroll_offset;
        elements.extend(build_ui_elements(state, assets, &self.rect, scrollable_pos));

        // Remove the clipping area
        elements.push(UiElement::Scissor { clip: None });

        elements
    }
}